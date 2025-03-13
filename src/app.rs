use std::sync::Arc;
use std::sync::Mutex;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;
use winit::window::WindowId;

use crate::gpu::Gpu;
use crate::gpu::Instance;

#[derive(Debug, Clone, Copy)]
pub enum JsEvents {
    AddRect(u32, u32, u32, u32),
}

#[derive(Copy, Clone, Debug)]
pub struct Rect(u32, u32, u32, u32);

#[derive(Debug, Clone)]
pub struct AppState {
    rects: Arc<Mutex<Vec<Rect>>>,
}

pub struct App<'window> {
    window: Option<Arc<Window>>,
    gpu: Option<Gpu<'window>>,
    state: Arc<Mutex<AppState>>,
}

impl App<'_> {
    pub fn new() -> Self {
        let state = Arc::new(Mutex::new(AppState {
            rects: Arc::new(Mutex::new(Vec::new())),
        }));

        Self {
            window: None,
            gpu: None,
            state: state.clone(),
        }
    }

    pub fn add_rect(&mut self, x: u32, y: u32, w: u32, h: u32) {
        self.state
            .lock()
            .unwrap()
            .rects
            .lock()
            .unwrap()
            .push(Rect(x, y, w, h));
        self.sync_gpu_instance_buffer();

        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }

    fn rects_to_instances(&self) -> Vec<Instance> {
        self.state
            .lock()
            .unwrap()
            .rects
            .lock()
            .unwrap()
            .iter()
            .map(|r| Instance::new(r.0 as f32, r.1 as f32, r.2 as f32, r.3 as f32))
            .collect()
    }

    fn sync_gpu_instance_buffer(&mut self) {
        let instances = self.rects_to_instances();
        if let Some(gpu) = self.gpu.as_mut() {
            gpu.update_instance_buffer(&instances);
        }
    }
}

impl<'window> ApplicationHandler<JsEvents> for App<'window> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = Arc::new(
                event_loop
                    .create_window(
                        Window::default_attributes()
                            .with_position(winit::dpi::PhysicalPosition::new(100, 200))
                            .with_title("wgpu winit example"),
                    )
                    .expect("create window err."),
            );

            self.window = Some(window.clone());
            self.gpu = Some(Gpu::new(window.clone(), self.rects_to_instances()));
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: JsEvents) {
        let JsEvents::AddRect(x, y, w, h) = event;
        self.add_rect(x, y, w, h);
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
