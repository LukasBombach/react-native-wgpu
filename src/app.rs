use std::sync::Arc;
use std::sync::Mutex;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::EventLoopProxy;
use winit::window::Window;
use winit::window::WindowId;

use crate::graphics::Gpu;
use crate::graphics::Instance;
use crate::graphics::Rect;

#[derive(Debug)]
pub enum Js {
    RectsUpdated,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub rects: Arc<Mutex<Vec<Arc<Mutex<Rect>>>>>,
    pub event_loop: Arc<Mutex<EventLoopProxy<Js>>>,
}

impl AppState {
    pub fn new(event_loop: Arc<Mutex<EventLoopProxy<Js>>>) -> Self {
        Self {
            rects: Arc::new(Mutex::new(Vec::new())),
            event_loop,
        }
    }

    pub fn add_rect(&mut self, rect: Rect) {
        let mut rects = self.rects.lock().unwrap();
        rects.push(Arc::new(Mutex::new(rect)));
    }
}

pub struct App<'window> {
    window: Option<Arc<Window>>,
    gpu: Option<Gpu<'window>>,
    pub state: Arc<Mutex<AppState>>,
}

impl App<'_> {
    pub fn new(event_loop: Arc<Mutex<EventLoopProxy<Js>>>) -> Self {
        let state = Arc::new(Mutex::new(AppState::new(event_loop)));

        Self {
            window: None,
            gpu: None,
            state: state.clone(),
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
            .map(|r| {
                let rect = r.lock().unwrap();
                Instance::new(rect.0 as f32, rect.1 as f32, rect.2 as f32, rect.3 as f32)
            })
            .collect()
    }
}

impl<'window> ApplicationHandler<Js> for App<'window> {
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
            self.gpu = Some(Gpu::new(window.clone()));
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: Js) {
        match event {
            Js::RectsUpdated => {
                let instances = self.rects_to_instances();
                if let Some(gpu) = self.gpu.as_mut() {
                    gpu.update_instance_buffer(&instances);
                }
                if let Some(window) = self.window.as_ref() {
                    window.request_redraw();
                }
            }
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
