use std::env::current_dir;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use deno_core::error::AnyError;
use deno_core::extension;
use deno_core::op2;
use deno_core::resolve_path;
use deno_core::v8;
use deno_core::FsModuleLoader;
use deno_core::JsRuntime;
use deno_core::OpState;
use deno_core::RuntimeOptions;

use winit::event_loop::EventLoopProxy;

use crate::app::Rect;
use crate::app::RectHandle;
use crate::JsEvents;

/* #[op2(fast)]
fn op_add_rect(
    state: &mut OpState,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
) -> Result<(), deno_error::JsErrorBox> {
    state
        .borrow::<Arc<Mutex<EventLoopProxy<JsEvents>>>>()
        .clone()
        .lock()
        .unwrap()
        .send_event(JsEvents::AddRect(x, y, w, h))
        .unwrap();

    Ok(())
} */

#[op2]
#[to_v8]
fn op_create_rect(
    state: &mut OpState,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
) -> Result<RectHandle, deno_error::JsErrorBox> {
    let rect = Arc::new(Mutex::new(Rect(x, y, w, h)));
    state
        .borrow::<Arc<Mutex<EventLoopProxy<JsEvents>>>>()
        .clone()
        .lock()
        .unwrap()
        .send_event(JsEvents::CreateRect(rect.clone()))
        .unwrap();

    Ok(RectHandle(rect))
}

#[op2]
#[to_v8]
fn op_get_rect(external: v8::Local<v8::External>) -> Result<Rect, deno_error::JsErrorBox> {
    // Hole den Pointer aus `v8::External`
    let ptr = external.value() as *const Mutex<Rect>;

    // Stelle die `Arc<Mutex<Rect>>`-Referenz wieder her
    let rect_arc = unsafe { Arc::from_raw(ptr) };

    // Sperre das Mutex und hole eine Kopie von `Rect`
    let rect = *rect_arc.lock().unwrap();

    println!("get rect: {:?}", rect);

    // Verhindere, dass `Arc::from_raw` den Speicher freigibt
    std::mem::forget(rect_arc);

    Ok(rect)
}

#[op2]
#[to_v8]
fn op_update_rect(
    external: v8::Local<v8::External>,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
) -> Result<Rect, deno_error::JsErrorBox> {
    // Hole den Pointer aus `v8::External`
    let ptr = external.value() as *const Mutex<Rect>;

    // Stelle die `Arc<Mutex<Rect>>`-Referenz wieder her
    let rect_arc = unsafe { Arc::from_raw(ptr) };

    // Sperre das Mutex und aktualisiere `Rect`
    let mut rect = rect_arc.lock().unwrap();

    println!("before update: {:?}", *rect);

    rect.0 = x;
    rect.1 = y;
    rect.2 = w;
    rect.3 = h;

    println!("after update: {:?}", *rect);

    // Verhindere, dass `Arc::from_raw` den Speicher freigibt
    std::mem::forget(Arc::clone(&rect_arc));

    Ok(*rect)
}

extension!(runjs, ops = [op_create_rect, op_get_rect, op_update_rect,]);

pub fn run_script(event_loop_proxy: Arc<Mutex<EventLoopProxy<JsEvents>>>, path: &str) {
    let proxy = event_loop_proxy.clone();
    let path = path.to_string();

    let _handle = thread::spawn(move || {
        let tokio_runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        if let Err(error) = tokio_runtime.block_on(async {
            let mut js_runtime = JsRuntime::new(RuntimeOptions {
                extensions: vec![runjs::init_ops()],
                module_loader: Some(Rc::new(FsModuleLoader)),
                ..Default::default()
            });

            js_runtime.op_state().borrow_mut().put(proxy);

            let main_module = resolve_path(&path, &current_dir().unwrap()).unwrap();
            let mod_id = js_runtime.load_main_es_module(&main_module).await?;
            let result = js_runtime.mod_evaluate(mod_id);

            js_runtime.run_event_loop(Default::default()).await?;

            result.await.map_err(|e| AnyError::from(e))
        }) {
            eprintln!("error: {}", error);
        }
    });
}
