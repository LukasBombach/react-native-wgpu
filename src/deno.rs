use std::borrow::Cow;
use std::env::current_dir;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use serde::Deserialize;

use deno_core::error::AnyError;
use deno_core::op2;
use deno_core::resolve_path;
use deno_core::Extension;
use deno_core::FsModuleLoader;
use deno_core::JsRuntime;
use deno_core::OpDecl;
use deno_core::OpState;
use deno_core::RuntimeOptions;

use winit::event_loop::EventLoopProxy;

use crate::JsEvents;
use crate::Rect;

#[derive(Debug, Deserialize)]
struct RectInput {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

#[op2]
fn op_add_rect(
    state: &mut OpState,
    #[serde] input: RectInput,
) -> Result<(), deno_error::JsErrorBox> {
    let proxy = state
        .borrow::<Arc<Mutex<EventLoopProxy<JsEvents>>>>()
        .clone();
    {
        // let _ = sender.send(JavaScriptAction::AddRect(Rect::new(
        //     input.x, input.y, input.w, input.h,
        // )));
        proxy
            .lock()
            .unwrap()
            .send_event(JsEvents::AddRect(Rect::new(
                input.x, input.y, input.w, input.h,
            )))
            .unwrap();
        println!("Added rect: {:?}", input);
    }
    Ok(())
}

pub struct Deno {
    proxy: Arc<Mutex<EventLoopProxy<JsEvents>>>,
}

impl Deno {
    pub fn new(proxy: Arc<Mutex<EventLoopProxy<JsEvents>>>) -> Self {
        Self { proxy }
    }

    pub fn run_script(&mut self, path: &str) {
        let proxy = self.proxy.clone();
        let path = path.to_string();

        let _handle = thread::spawn(move || {
            let tokio_runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            if let Err(error) = tokio_runtime.block_on(async {
                const DECL: OpDecl = op_add_rect();
                let ext = Extension {
                    name: "add_rect_ext",
                    ops: Cow::Borrowed(&[DECL]),
                    ..Default::default()
                };

                let mut js_runtime = JsRuntime::new(RuntimeOptions {
                    extensions: vec![ext],
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
}
