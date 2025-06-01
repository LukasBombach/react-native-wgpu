#![allow(clippy::print_stdout)]
#![allow(clippy::print_stderr)]

use taffy::prelude::*;

use color::parse_color;
use color::DynamicColor;
use deno_core::extension;
use deno_core::op2;
use deno_core::OpState;
use deno_error::JsErrorBox;
use notify::event::ModifyKind;
use notify::{recommended_watcher, EventKind, RecursiveMode, Watcher};
use rustyscript::{Error, Module, Runtime, RuntimeOptions};
use std::path::Path;
use std::str::FromStr;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use taffy::NodeId;

use crate::gui::Gui;

// op2 ref: https://docs.rs/rustjs/latest/rustjs/deno_core/attr.op2.html#parameters

#[op2]
#[bigint]
#[string]
fn op_create_text_instance(
    state: &mut OpState,
    #[string] text: String,
    #[serde] layout: Style,
) -> Result<usize, JsErrorBox> {
    let node_id = state
        .borrow::<Arc<Mutex<Gui>>>()
        .lock()
        .unwrap()
        .create_text_node(text, layout);

    Ok(usize::from(node_id))
}

#[op2]
#[bigint]
#[string]
#[number]
fn op_create_instance(
    state: &mut OpState,
    #[serde] layout: Style,
    #[string] background_color: String,
    border_radius: u32,
) -> Result<usize, JsErrorBox> {
    let default_background: &str = "transparent";

    let parsed_background_color = parse_color(&background_color)
        .unwrap_or(DynamicColor::from_str(default_background).unwrap())
        .components;

    let node_id = state
        .borrow::<Arc<Mutex<Gui>>>()
        .lock()
        .unwrap()
        .create_node(layout, parsed_background_color, border_radius);

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

    Ok(())
}

extension!(
    rect_extension,
    ops = [
        op_create_instance,
        op_create_text_instance,
        op_append_child_to_container,
        op_append_child,
    ],
);

pub fn run_script(gui: Arc<Mutex<Gui>>, js_path: &str) {
    let js_path_buf = Path::new(env!("CARGO_MANIFEST_DIR")).join(js_path);

    let _handle = thread::spawn(move || {
        let (tx, rx) = mpsc::channel();

        let mut watcher = recommended_watcher(tx).unwrap();
        watcher
            .watch(
                Path::new(env!("CARGO_MANIFEST_DIR")).join("src").as_ref(),
                RecursiveMode::NonRecursive,
            )
            .unwrap();

        let mut runtime = match init_runtime(gui.clone()) {
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

fn init_runtime(gui: Arc<Mutex<Gui>>) -> Result<Runtime, Error> {
    println!("Initializing runtime...");
    let mut runtime = Runtime::new(RuntimeOptions {
        extensions: vec![rect_extension::init_ops_and_esm()],
        ..RuntimeOptions::default()
    })?;

    runtime.deno_runtime().op_state().borrow_mut().put(gui);

    runtime.set_current_dir("src")?;

    println!("Runtime initialized successfully.");

    Ok(runtime)
}
