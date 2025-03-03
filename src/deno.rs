use std::borrow::Cow;
use std::env::current_dir;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

use deno_core::error::AnyError;
use deno_core::op2;
use deno_core::resolve_path;
use deno_core::Extension;
use deno_core::FsModuleLoader;
use deno_core::JsRuntime;
use deno_core::OpDecl;
use deno_core::OpState;
use deno_core::RuntimeOptions;

use serde::Deserialize;

use crate::app::AppState;

#[derive(Debug, Deserialize)]
struct RecteInput {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

#[op2]
fn op_add_rect(
    state: &mut OpState,
    #[serde] input: RecteInput,
) -> Result<(), deno_error::JsErrorBox> {
    let app_state = state.borrow::<Arc<Mutex<AppState>>>().clone();
    {
        let mut app_state = app_state.lock().unwrap();
        app_state.add_rect(input.x, input.y, input.w, input.h);
    }
    Ok(())
}

pub struct Deno {
    app_state: Arc<Mutex<AppState>>,
}

impl Deno {
    pub fn new(app_state: Arc<Mutex<AppState>>) -> Self {
        Self { app_state }
    }

    pub fn run_script(&mut self, path: &str) {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        if let Err(error) = runtime.block_on(self.run_script_async(path)) {
            eprintln!("error: {}", error);
        }
    }

    async fn run_script_async(&mut self, path: &str) -> Result<(), AnyError> {
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

        js_runtime
            .op_state()
            .borrow_mut()
            .put(self.app_state.clone());

        let main_module = resolve_path(path, &current_dir().unwrap()).unwrap();
        let mod_id = js_runtime.load_main_es_module(&main_module).await?;
        let result = js_runtime.mod_evaluate(mod_id);

        js_runtime.run_event_loop(Default::default()).await?;

        result.await.map_err(|e| e.into())
    }
}
