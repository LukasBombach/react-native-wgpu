use std::sync::Arc;
use std::sync::Mutex;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::EventLoopProxy;
use winit::window::Window;
use winit::window::WindowId;

use crate::graphics::Gpu;
use crate::user_interface::UserInterface;

#[derive(Debug)]
pub enum Js {
    RectsUpdated,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub user_interface: Arc<Mutex<UserInterface>>,
    pub event_loop: Arc<Mutex<EventLoopProxy<Js>>>,
}

impl AppState {
    pub fn new(event_loop: Arc<Mutex<EventLoopProxy<Js>>>) -> Self {
        Self {
            user_interface: Arc::new(Mutex::new(UserInterface::new())),
            event_loop,
        }
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
                if let Some(window) = self.window.as_ref() {
                    let size = window.inner_size();
                    if let Some(instances) = self
                        .state
                        .lock()
                        .unwrap()
                        .user_interface
                        .lock()
                        .unwrap()
                        .get_instances(size.width as f32, size.height as f32)
                    {
                        if let Some(gpu) = self.gpu.as_mut() {
                            gpu.update_instance_buffer(&instances);
                        }
                        window.request_redraw();
                    }
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
                if let Some(instances) = self
                    .state
                    .lock()
                    .unwrap()
                    .user_interface
                    .lock()
                    .unwrap()
                    .get_instances(size.width as f32, size.height as f32)
                {
                    if let Some(gpu) = self.gpu.as_mut() {
                        gpu.update_instance_buffer(&instances);
                        gpu.set_size(size.width, size.height);
                    }
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
