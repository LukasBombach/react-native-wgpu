#![allow(clippy::print_stdout)]
#![allow(clippy::print_stderr)]

use taffy::prelude::*;

use deno_core::extension;
use deno_core::op2;
use deno_core::OpState;
use deno_error::JsErrorBox;
use notify::event::ModifyKind;
use notify::{recommended_watcher, EventKind, RecursiveMode, Watcher};
use rustyscript::{Error, Module, Runtime, RuntimeOptions};
use std::path::Path;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use taffy::NodeId;

use crate::app::AppState;
use crate::app::Js;
use crate::gui::Gui;

#[op2]
#[bigint]
fn op_create_instance(state: &mut OpState, #[serde] style: Style) -> Result<usize, JsErrorBox> {
    let node_id = state
        .borrow::<Arc<Mutex<Gui>>>()
        .lock()
        .unwrap()
        .create_node(style);

    Ok(usize::from(node_id))
}

#[op2(fast)]
fn op_append_child_to_container(
    state: &mut OpState,
    #[bigint] node_id: usize,
) -> Result<(), JsErrorBox> {
    state
        .borrow::<Arc<Mutex<Gui>>>()
        .lock()
        .unwrap()
        .append_child_to_root(NodeId::from(node_id));

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
fn op_append_child(
    state: &mut OpState,
    #[bigint] parent_id: usize,
    #[bigint] child_id: usize,
) -> Result<(), JsErrorBox> {
    state
        .borrow::<Arc<Mutex<Gui>>>()
        .lock()
        .unwrap()
        .append_child(NodeId::from(parent_id), NodeId::from(child_id));

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

extension!(
    rect_extension,
    ops = [
        op_create_instance,
        op_append_child_to_container,
        op_append_child,
    ],
);

pub fn run_script(app_state: Arc<Mutex<AppState>>, gui: Arc<Mutex<Gui>>, js_path: &str) {
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

        let mut runtime = match init_runtime(app_state.clone(), gui.clone()) {
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

                            gui.lock().unwrap().clear();

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

fn init_runtime(app_state: Arc<Mutex<AppState>>, gui: Arc<Mutex<Gui>>) -> Result<Runtime, Error> {
    println!("Initializing runtime...");
    let mut runtime = Runtime::new(RuntimeOptions {
        extensions: vec![rect_extension::init_ops_and_esm()],
        ..RuntimeOptions::default()
    })?;

    runtime
        .deno_runtime()
        .op_state()
        .borrow_mut()
        .put(app_state);

    runtime.deno_runtime().op_state().borrow_mut().put(gui);

    runtime.set_current_dir("src")?;

    println!("Runtime initialized successfully.");

    Ok(runtime)
}
