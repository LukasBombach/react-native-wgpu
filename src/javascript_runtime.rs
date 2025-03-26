#![allow(clippy::print_stdout)]
#![allow(clippy::print_stderr)]

use std::collections::HashSet;
use std::path::Path;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use notify::event::ModifyKind;
use notify::{recommended_watcher, EventKind, RecursiveMode, Watcher};

use rustyscript::{Error, Module, Runtime, RuntimeOptions};

use deno_core::extension;
use deno_core::op2;
use deno_core::OpState;
use deno_core::Resource;
use deno_error::JsErrorBox;

use crate::app::AppState;
use crate::app::Js;
use crate::graphics::Rect;

struct RectResource(Arc<Mutex<Rect>>);
impl Resource for RectResource {}

/**
 * todo the way rect are added to the resource table and also synced with the app state
 * does not seem to be the best way to do it
 */

#[op2(fast)]
fn op_create_rect(state: &mut OpState, x: u32, y: u32, w: u32, h: u32) -> Result<u32, JsErrorBox> {
    let rect = Arc::new(Mutex::new(Rect(x, y, w, h)));

    let resource_table = &mut state.resource_table;
    let rid = resource_table.add(RectResource(rect.clone()));

    Ok(rid)
}

#[op2(fast)]
fn op_append_rect_to_window(state: &mut OpState, rid: u32) -> Result<(), JsErrorBox> {
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
fn op_update_rect(
    state: &mut OpState,
    rid: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
) -> Result<(), JsErrorBox> {
    let resource_table = &mut state.resource_table;
    let rect_resource = resource_table.get::<RectResource>(rid).unwrap();
    let rect = rect_resource.0.clone();

    let mut rect = rect.lock().unwrap();
    rect.0 = x;
    rect.1 = y;
    rect.2 = w;
    rect.3 = h;

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
fn op_remove_rect_from_window(state: &mut OpState, rid: u32) -> Result<(), JsErrorBox> {
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
        op_create_rect,
        op_append_rect_to_window,
        op_update_rect,
        op_remove_rect_from_window,
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
            .watch(js_path_buf.as_ref(), RecursiveMode::NonRecursive)
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

                            // todo clear resources from the deno runtime

                            app_state.lock().unwrap().rects.lock().unwrap().clear();

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
