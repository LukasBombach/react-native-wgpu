use std::sync::Arc;
use std::sync::Mutex;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::EventLoopProxy;
use winit::window::Window;
use winit::window::WindowId;

use crate::gpu::Gpu;
use crate::gpu::Instance;
use crate::gui;
use crate::gui::Gui;
use crate::user_interface::UserInterface;

#[derive(Debug)]
pub enum Js {
    RectsUpdated,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub event_loop: Arc<Mutex<EventLoopProxy<Js>>>,
}

impl AppState {
    pub fn new(event_loop: Arc<Mutex<EventLoopProxy<Js>>>) -> Self {
        Self { event_loop }
    }
}

pub struct App<'window> {
    window: Option<Arc<Window>>,
    gpu: Option<Gpu<'window>>,
    pub gui: Arc<Mutex<Gui>>,
    pub state: Arc<Mutex<AppState>>,
}

impl App<'_> {
    pub fn new(event_loop: Arc<Mutex<EventLoopProxy<Js>>>) -> Self {
        let state = Arc::new(Mutex::new(AppState::new(event_loop)));

        Self {
            window: None,
            gpu: None,
            gui: Arc::new(Mutex::new(Gui::new())),
            state: state.clone(),
        }
    }

    fn get_instances_temp(&mut self, width: f32, height: f32) -> Option<Vec<Instance>> {
        fn collect_instances(
            gui: &Gui,
            node_id: taffy::NodeId,
            offset_x: f32,
            offset_y: f32,
            instances: &mut Vec<Instance>,
        ) {
            let layout = gui.layout_from_id(node_id);
            let (x, y) = (offset_x + layout.location.x, offset_y + layout.location.y);
            instances.push(Instance::new(x, y, layout.size.width, layout.size.height));

            for child_id in gui.children_from_id(node_id) {
                collect_instances(gui, *child_id, x, y, instances);
            }
        }

        let mut gui = self.gui.lock().unwrap();
        gui.compute_layout(width, height);

        let mut instances = Vec::new();
        collect_instances(&gui, gui.root, 0.0, 0.0, &mut instances);

        Some(instances)
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

                    let instances = self
                        .get_instances_temp(size.width as f32, size.height as f32)
                        .unwrap();

                    if let Some(gpu) = self.gpu.as_mut() {
                        gpu.update_instance_buffer(&instances);
                    }

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
