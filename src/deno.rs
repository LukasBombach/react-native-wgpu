use std::env::current_dir;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use deno_core::error::AnyError;
use deno_core::extension;
use deno_core::op2;
use deno_core::resolve_path;
use deno_core::FsModuleLoader;
use deno_core::JsRuntime;
use deno_core::OpState;
use deno_core::Resource;
use deno_core::RuntimeOptions;

use crate::app::AppState;
use crate::app::Rect;

struct RectResource(Arc<Mutex<Rect>>);
impl Resource for RectResource {}

#[op2(fast)]
fn op_create_rect(
    state: &mut OpState,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
) -> Result<u32, deno_error::JsErrorBox> {
    let rect = Arc::new(Mutex::new(Rect(x, y, w, h)));

    let resource_table = &mut state.resource_table;
    let rid = resource_table.add(RectResource(rect.clone()));

    Ok(rid)
}

#[op2(fast)]
fn op_append_rect_to_window(state: &mut OpState, rid: u32) -> Result<(), deno_error::JsErrorBox> {
    let resource_table = &mut state.resource_table;
    let rect_resource = resource_table.get::<RectResource>(rid).unwrap();
    let rect = rect_resource.0.clone();

    state
        .borrow::<Arc<Mutex<AppState>>>()
        .lock()
        .unwrap()
        .rects
        .lock()
        .unwrap()
        .push(rect.clone());

    Ok(())
}

#[op2(fast)]
fn op_update_rect(
    state: &mut OpState,
    rid: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
) -> Result<(), deno_error::JsErrorBox> {
    let resource_table = &mut state.resource_table;
    let rect_resource = resource_table.get::<RectResource>(rid).unwrap();
    let rect = rect_resource.0.clone();

    let mut rect = rect.lock().unwrap();
    rect.0 = x;
    rect.1 = y;
    rect.2 = w;
    rect.3 = h;

    Ok(())
}

#[op2(fast)]
fn op_remove_rect(state: &mut OpState, rid: u32) -> Result<(), deno_error::JsErrorBox> {
    let resource_table = &mut state.resource_table;
    let rect_resource = resource_table.take::<RectResource>(rid).unwrap();
    let rect = rect_resource.0.clone();

    state
        .borrow::<Arc<Mutex<AppState>>>()
        .lock()
        .unwrap()
        .rects
        .lock()
        .unwrap()
        .retain(|item| !Arc::ptr_eq(item, &rect));

    Ok(())
}

extension!(
    rects,
    ops = [
        op_create_rect,
        op_append_rect_to_window,
        op_update_rect,
        op_remove_rect,
    ]
);

pub fn run_script(app_state: Arc<Mutex<AppState>>, path: &str) {
    let path = path.to_string();

    let _handle = thread::spawn(move || {
        let tokio_runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        if let Err(error) = tokio_runtime.block_on(async {
            let mut js_runtime = JsRuntime::new(RuntimeOptions {
                extensions: vec![rects::init_ops_and_esm()],
                module_loader: Some(Rc::new(FsModuleLoader)),
                ..Default::default()
            });

            js_runtime.op_state().borrow_mut().put(app_state);

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
