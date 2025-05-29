use std::sync::Arc;
use std::sync::Mutex;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::EventLoopProxy;
use winit::window::Window;
use winit::window::WindowId;

use crate::gpu::Gpu;
use crate::gui::Gui;

#[derive(Debug)]
pub enum CustomEvent {
    GuiUpdate,
}

pub struct App<'window> {
    window: Option<Arc<Window>>,
    gpu: Option<Gpu<'window>>,
    pub gui: Arc<Mutex<Gui>>,
}

impl App<'_> {
    pub fn new(event_loop: Arc<Mutex<EventLoopProxy<CustomEvent>>>) -> Self {
        Self {
            window: None,
            gpu: None,
            gui: Arc::new(Mutex::new(Gui::new(event_loop.clone()))),
        }
    }
}

impl<'window> ApplicationHandler<CustomEvent> for App<'window> {
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

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: CustomEvent) {
        match event {
            CustomEvent::GuiUpdate => {
                if let Some(window) = self.window.as_ref() {
                    if let Some(gpu) = self.gpu.as_mut() {
                        if let Ok(mut gui) = self.gui.lock() {
                            let size = window.inner_size();

                            gui.compute_layout(size.width, size.height);
                            gpu.update_instance_buffer(gui.into_instances());

                            window.request_redraw();
                        }
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
                if let Some(gpu) = self.gpu.as_mut() {
                    if let Ok(mut gui) = self.gui.lock() {
                        gui.compute_layout(size.width, size.height);
                        gpu.update_instance_buffer(gui.into_instances());

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
