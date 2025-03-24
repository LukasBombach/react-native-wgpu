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

use rustyscript::{Module, Runtime, RuntimeOptions};

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

    let _handle = thread::spawn(move || {
        let (tx, rx) = mpsc::channel();

        let mut watcher = recommended_watcher(tx).unwrap();
        watcher
            .watch(js_path_buf.as_ref(), RecursiveMode::NonRecursive)
            .unwrap();

        run_js_runtime(app_state.clone(), &js_path_buf);

        println!("🔁 Überwache Änderungen…");

        loop {
            match rx.recv() {
                Ok(event) => {
                    // Überprüfen, ob die Änderung eine Dateiänderung ist
                    if let Ok(event) = event {
                        if let EventKind::Modify(ModifyKind::Data(_)) = event.kind {
                            // Hier können Sie den Code hinzufügen, der ausgeführt werden soll,
                            // wenn eine Dateiänderung erkannt wird.
                            println!("🔁 Änderung erkannt. Starte neu…");
                            run_js_runtime(app_state.clone(), &js_path_buf);
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }
                Err(error) => {
                    eprintln!("watch error: {:#?}", error);
                    break;
                }
            }
        }
    });
}

fn run_js_runtime(app_state: Arc<Mutex<AppState>>, js_path: &Path) {
    let mut schema_whlist = HashSet::new();
    schema_whlist.insert("rn-wgpu:".to_string());

    let mut runtime = match Runtime::new(RuntimeOptions {
        schema_whlist,
        extensions: vec![rect_extension::init_ops_and_esm()],
        ..RuntimeOptions::default()
    }) {
        Ok(rt) => rt,
        Err(err) => {
            eprintln!("❌ Fehler beim Erstellen der Runtime: {}", err);
            return;
        }
    };

    // AppState injizieren
    runtime
        .deno_runtime()
        .op_state()
        .borrow_mut()
        .put(app_state);

    if let Err(err) = runtime.set_current_dir("src") {
        eprintln!("⚠️ set_current_dir fehlgeschlagen: {}", err);
    }

    let module = match Module::load(js_path) {
        Ok(m) => m,
        Err(err) => {
            eprintln!("❌ Fehler beim Laden des Moduls: {}", err);
            return;
        }
    };

    if let Err(err) = runtime.load_module(&module) {
        eprintln!("❌ Fehler beim Ausführen des Moduls: {}", err);
    }
}
