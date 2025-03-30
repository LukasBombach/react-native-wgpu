#![allow(clippy::print_stdout)]
#![allow(clippy::print_stderr)]

use deno_core::extension;
use deno_core::op2;
use deno_core::OpState;
use deno_error::JsErrorBox;
use notify::event::ModifyKind;
use notify::{recommended_watcher, EventKind, RecursiveMode, Watcher};
use rustyscript::{Error, Module, Runtime, RuntimeOptions};
use std::collections::HashSet;
use std::path::Path;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use taffy::NodeId;

use taffy::prelude::*;

use crate::app::AppState;
use crate::app::Js;

#[op2]
fn op_create_instance(state: &mut OpState, #[serde] style: Style) -> Result<f64, JsErrorBox> {
    // print!("\n instance {style:?}");

    let node_id = state
        .borrow::<Arc<Mutex<AppState>>>()
        .lock()
        .unwrap()
        .user_interface
        .lock()
        .unwrap()
        .create_node(style);

    Ok(u64::from(node_id) as f64)
}

#[op2(fast)]
fn op_append_child_to_container(state: &mut OpState, node_id: f64) -> Result<(), JsErrorBox> {
    state
        .borrow::<Arc<Mutex<AppState>>>()
        .lock()
        .unwrap()
        .user_interface
        .lock()
        .unwrap()
        .add_child_to_root(NodeId::from(node_id as u64));

    state
        .borrow::<Arc<Mutex<AppState>>>()
        .lock()
        .unwrap()
        .event_loop
        .lock()
        .unwrap()
        .send_event(Js::RectsUpdated)
        .unwrap();

    Ok(())
}

#[op2(fast)]
fn op_debug(state: &mut OpState) -> Result<(), JsErrorBox> {
    state
        .borrow::<Arc<Mutex<AppState>>>()
        .lock()
        .unwrap()
        .user_interface
        .lock()
        .unwrap()
        .debug();

    Ok(())
}

extension!(
    rect_extension,
    ops = [
        op_create_instance,
        op_append_child_to_container,
        op_debug,
    ],
    esm_entry_point = "rn-wgpu:rect",
    esm = [ dir "src", "rn-wgpu:rect" = "extension.js" ],
);

pub fn run_script(app_state: Arc<Mutex<AppState>>, js_path: &str) {
    let js_path_buf = Path::new(env!("CARGO_MANIFEST_DIR")).join(js_path);

    let app_state_for_thread = app_state.clone();

    let _handle = thread::spawn(move || {
        let (tx, rx) = mpsc::channel();

        let mut watcher = recommended_watcher(tx).unwrap();
        watcher
            .watch(
                Path::new(env!("CARGO_MANIFEST_DIR")).join("src").as_ref(),
                RecursiveMode::NonRecursive,
            )
            .unwrap();

        let mut runtime = match init_runtime(app_state.clone()) {
            Ok(runtime) => runtime,
            Err(error) => {
                eprintln!("{error}");
                return;
            }
        };

        let module = match Module::load(&js_path_buf) {
            Ok(module) => module,
            Err(error) => {
                eprintln!("{error}");
                return;
            }
        };

        if let Err(error) = runtime.load_module(&module) {
            eprintln!("{error}");
        }

        loop {
            match rx.recv() {
                Ok(event) => {
                    if let Ok(event) = event {
                        if let EventKind::Modify(ModifyKind::Data(_)) = event.kind {
                            println!("reloading... ");
                            let module = match Module::load(&js_path_buf) {
                                Ok(module) => module,
                                Err(error) => {
                                    eprintln!("{error}");
                                    return;
                                }
                            };

                            app_state
                                .lock()
                                .unwrap()
                                .user_interface
                                .lock()
                                .unwrap()
                                .clear();

                            app_state_for_thread
                                .lock()
                                .unwrap()
                                .event_loop
                                .lock()
                                .unwrap()
                                .send_event(Js::RectsUpdated)
                                .unwrap();

                            if let Err(error) = runtime.load_module(&module) {
                                eprintln!("{error}");
                            }
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }
                Err(error) => {
                    eprintln!("{:#?}", error);
                    break;
                }
            }
        }
    });
}

fn init_runtime(app_state: Arc<Mutex<AppState>>) -> Result<Runtime, Error> {
    let mut schema_whlist = HashSet::new();
    schema_whlist.insert("rn-wgpu:".to_string());

    let mut runtime = Runtime::new(RuntimeOptions {
        schema_whlist,
        extensions: vec![rect_extension::init_ops_and_esm()],
        ..RuntimeOptions::default()
    })?;

    runtime
        .deno_runtime()
        .op_state()
        .borrow_mut()
        .put(app_state);

    runtime.set_current_dir("src")?;

    Ok(runtime)
}
