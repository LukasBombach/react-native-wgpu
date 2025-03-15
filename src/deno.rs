#![allow(clippy::print_stdout)]
#![allow(clippy::print_stderr)]

use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use deno_core::extension;
use deno_core::op2;
use deno_core::FsModuleLoader;
use deno_core::ModuleSpecifier;
use deno_core::OpState;
use deno_core::Resource;
use deno_fs::RealFs;
use deno_resolver::npm::DenoInNpmPackageChecker;
use deno_resolver::npm::NpmResolver;
use deno_runtime::deno_permissions::PermissionsContainer;
use deno_runtime::permissions::RuntimePermissionDescriptorParser;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::worker::WorkerServiceOptions;

use crate::app::AppState;
use crate::app::Js;
use crate::graphics::Rect;

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
) -> Result<(), deno_error::JsErrorBox> {
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
fn op_remove_rect_from_window(state: &mut OpState, rid: u32) -> Result<(), deno_error::JsErrorBox> {
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
    rn_wgpu,
    ops = [
        op_create_rect,
        op_append_rect_to_window,
        op_update_rect,
        op_remove_rect_from_window,
    ],
    esm_entry_point = "ext:rn_wgpu/rn_wgpu.js",
    esm = [dir "src", "rn_wgpu.js"]
);

pub fn run_script(app_state: Arc<Mutex<AppState>>, js_path: &str) {
    // let js_path = js_path.to_string();

    let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(js_path);

    let _handle = thread::spawn(move || {
        let tokio_runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        tokio_runtime.block_on(async {
            let main_module = ModuleSpecifier::from_file_path(js_path).unwrap();

            eprintln!("Running {main_module}...");

            let fs = Arc::new(RealFs);
            let permission_desc_parser = Arc::new(RuntimePermissionDescriptorParser::new(
                sys_traits::impls::RealSys,
            ));

            let mut worker = MainWorker::bootstrap_from_options(
                &main_module,
                WorkerServiceOptions::<
                    DenoInNpmPackageChecker,
                    NpmResolver<sys_traits::impls::RealSys>,
                    sys_traits::impls::RealSys,
                > {
                    module_loader: Rc::new(FsModuleLoader),
                    permissions: PermissionsContainer::allow_all(permission_desc_parser),
                    blob_store: Default::default(),
                    broadcast_channel: Default::default(),
                    feature_checker: Default::default(),
                    node_services: Default::default(),
                    npm_process_state_provider: Default::default(),
                    root_cert_store_provider: Default::default(),
                    fetch_dns_resolver: Default::default(),
                    shared_array_buffer_store: Default::default(),
                    compiled_wasm_module_store: Default::default(),
                    v8_code_cache: Default::default(),
                    fs,
                },
                WorkerOptions {
                    extensions: vec![rn_wgpu::init_ops_and_esm()],
                    ..Default::default()
                },
            );

            worker.js_runtime.op_state().borrow_mut().put(app_state);
            (worker.execute_main_module(&main_module).await).unwrap();
            (worker.run_event_loop(false).await).unwrap();
        });
    });
}
