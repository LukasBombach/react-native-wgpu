use std::sync::Arc;
use std::sync::Mutex;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;
use winit::window::WindowId;

use serde::Deserialize;
use serde::Serialize;

use deno_core::v8;
use deno_core::FromV8;
use deno_core::ToV8;
use deno_error::JsErrorBox;

use crate::gpu::Gpu;
use crate::gpu::Instance;

#[derive(Debug, Clone)]
pub enum JsEvents {
    CreateRect(Arc<Mutex<Rect>>),
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Rect(pub u32, pub u32, pub u32, pub u32);

impl<'a> ToV8<'a> for Rect {
    type Error = JsErrorBox;

    fn to_v8(
        self,
        scope: &mut v8::HandleScope<'a>,
    ) -> Result<v8::Local<'a, v8::Value>, JsErrorBox> {
        let obj = v8::Object::new(scope);

        // Konvertiere Keys zu v8::Strings und Werte zu v8::Value
        let x_key = v8::String::new(scope, "x").unwrap();
        let y_key = v8::String::new(scope, "y").unwrap();
        let w_key = v8::String::new(scope, "w").unwrap();
        let h_key = v8::String::new(scope, "h").unwrap();

        let x_value = v8::Number::new(scope, self.0 as f64).into();
        let y_value = v8::Number::new(scope, self.1 as f64).into();
        let w_value = v8::Number::new(scope, self.2 as f64).into();
        let h_value = v8::Number::new(scope, self.3 as f64).into();

        obj.set(scope, x_key.into(), x_value).unwrap();
        obj.set(scope, y_key.into(), y_value).unwrap();
        obj.set(scope, w_key.into(), w_value).unwrap();
        obj.set(scope, h_key.into(), h_value).unwrap();

        Ok(obj.into())
    }
}

#[derive(Clone)]
pub struct RectHandle(pub Arc<Mutex<Rect>>);

impl<'a> ToV8<'a> for RectHandle {
    type Error = JsErrorBox;

    fn to_v8(
        self,
        scope: &mut v8::HandleScope<'a>,
    ) -> Result<v8::Local<'a, v8::Value>, JsErrorBox> {
        let ptr = Arc::into_raw(self.0.clone()) as *mut std::ffi::c_void;
        let external = v8::External::new(scope, ptr);
        Ok(external.into())
    }
}

impl<'a> FromV8<'a> for RectHandle {
    type Error = JsErrorBox;

    fn from_v8(
        _scope: &mut v8::HandleScope<'a>,
        value: v8::Local<'a, v8::Value>,
    ) -> Result<RectHandle, JsErrorBox> {
        let external = v8::Local::<v8::External>::try_from(value).unwrap();
        let ptr = external.value() as *mut Mutex<Rect>;
        let rect = unsafe { Arc::from_raw(ptr) };
        Ok(RectHandle(rect))
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    rects: Vec<Arc<Mutex<Rect>>>,
}

pub struct App<'window> {
    window: Option<Arc<Window>>,
    gpu: Option<Gpu<'window>>,
    state: Arc<Mutex<AppState>>,
}

impl App<'_> {
    pub fn new() -> Self {
        let state = Arc::new(Mutex::new(AppState { rects: Vec::new() }));

        Self {
            window: None,
            gpu: None,
            state: state.clone(),
        }
    }

    pub fn create_rect(&mut self, rect: Arc<Mutex<Rect>>) {
        self.state.lock().unwrap().rects.push(rect.clone());
        self.sync_gpu_instance_buffer();

        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }

    fn sync_gpu_instance_buffer(&mut self) {
        let instances = self.rects_to_instances();
        if let Some(gpu) = self.gpu.as_mut() {
            gpu.update_instance_buffer(&instances);
        }
    }

    fn rects_to_instances(&self) -> Vec<Instance> {
        self.state
            .lock()
            .unwrap()
            .rects
            .iter()
            .map(|r| {
                let r = r.lock().unwrap();
                Instance::new(r.0 as f32, r.1 as f32, r.2 as f32, r.3 as f32)
            })
            .collect()
    }
}

impl<'window> ApplicationHandler<JsEvents> for App<'window> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = Arc::new(
                event_loop
                    .create_window(Window::default_attributes().with_title("wgpu winit example"))
                    .expect("create window err."),
            );

            self.window = Some(window.clone());
            self.gpu = Some(Gpu::new(window.clone(), self.rects_to_instances()));
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: JsEvents) {
        match event {
            JsEvents::CreateRect(rect) => self.create_rect(rect),
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let Some(gpu) = self.gpu.as_mut() {
                    gpu.set_size(size.width, size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(gpu) = self.gpu.as_mut() {
                    gpu.draw();
                }
            }
            _ => (),
        }
    }
}
