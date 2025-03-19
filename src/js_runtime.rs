use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use rustyscript::{Module, Runtime, RuntimeOptions};

use crate::app::AppState;

pub fn run_script(app_state: Arc<Mutex<AppState>>, js_path: &str) {
    let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(js_path);

    let _handle = thread::spawn(move || {
        let mut runtime = Runtime::new(RuntimeOptions::default()).unwrap();
        runtime.set_current_dir("src").unwrap();
        let module = Module::load(js_path).unwrap();
        runtime.load_module(&module).unwrap();
    });
}
