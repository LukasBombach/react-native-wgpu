use std::env::current_dir;
use std::rc::Rc;

use deno_core::error::AnyError;
use deno_core::resolve_path;
use deno_core::FsModuleLoader;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;

pub struct Deno {}

impl Deno {
    pub fn new() -> Self {
        Self {}
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
        let mut js_runtime = JsRuntime::new(RuntimeOptions {
            module_loader: Some(Rc::new(FsModuleLoader)),
            ..Default::default()
        });

        let main_module = resolve_path(path, &current_dir().unwrap()).unwrap();
        let mod_id = js_runtime.load_main_es_module(&main_module).await?;
        let result = js_runtime.mod_evaluate(mod_id);

        js_runtime.run_event_loop(Default::default()).await?;

        result.await.map_err(|e| e.into())
    }
}
