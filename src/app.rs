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

    fn user_interface_to_instances(&self) -> Option<Vec<Instance>> {
        if let Some(window) = &self.window {
            let width = window.inner_size().width as f32;
            let height = window.inner_size().height as f32;

            let state = self.state.lock().unwrap();
            let mut user_interface = state.user_interface.lock().unwrap();

            println!("\ncompute_layout ({:?}, {:?})\n", width, height);

            user_interface.compute_layout(width, height);

            let root_layout = user_interface.taffy.layout(user_interface.root).unwrap();
            let root_size = root_layout.size;
            let root_location = root_layout.location;

            println!("root {root_size:?} {root_location:?}");

            let instances = user_interface
                .taffy
                .children(user_interface.root)
                .unwrap()
                .iter()
                .map(|child| {
                    let layout = user_interface.taffy.layout(*child).unwrap();
                    let location = layout.location;
                    let size = layout.size;

                    println!("node {size:?} {location:?}");

                    Instance::new(location.x, location.y, size.width, size.height)
                })
                .collect();

            return Some(instances);
        } else {
            return None;
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
                if let Some(instances) = self.user_interface_to_instances() {
                    if let Some(gpu) = self.gpu.as_mut() {
                        gpu.update_instance_buffer(&instances);
                    }
                    if let Some(window) = self.window.as_ref() {
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
                if let Some(instances) = self.user_interface_to_instances() {
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
