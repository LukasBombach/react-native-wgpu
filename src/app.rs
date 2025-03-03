use std::sync::Arc;
use std::sync::Mutex;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

use crate::deno::Deno;
use crate::gpu;
use crate::gpu::Gpu;

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pos: [u32; 2],
    size: [u32; 2],
}

impl Rect {
    pub fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        Self {
            pos: [x, y],
            size: [w, h],
        }
    }

    pub fn to_instance(&self) -> gpu::Instance {
        gpu::Instance::new(
            self.pos[0] as f32,
            self.pos[1] as f32,
            self.size[0] as f32,
            self.size[1] as f32,
        )
    }
}

pub struct AppState {
    rects: Vec<Rect>,
}

impl AppState {
    pub fn new() -> Self {
        Self { rects: Vec::new() }
    }

    pub fn add_rect(&mut self, x: u32, y: u32, w: u32, h: u32) {
        self.rects.push(Rect::new(x, y, w, h));
    }
}

pub struct App<'window> {
    window: Option<Arc<Window>>,
    gpu: Option<Gpu<'window>>,
    deno: Deno,
    state: Arc<Mutex<AppState>>,
}

impl App<'_> {
    pub fn new() -> Self {
        let state = Arc::new(Mutex::new(AppState::new()));
        Self {
            window: None,
            gpu: None,
            state: state.clone(),
            deno: Deno::new(state.clone()),
        }
    }

    pub fn add_rect(&mut self, x: u32, y: u32, w: u32, h: u32) {
        self.state.lock().unwrap().rects.push(Rect::new(x, y, w, h));
        self.sync_gpu_instance_buffer();
    }

    fn rects_to_instances(&self) -> Vec<gpu::Instance> {
        self.state
            .lock()
            .unwrap()
            .rects
            .iter()
            .map(|r| r.to_instance())
            .collect()
    }

    fn sync_gpu_instance_buffer(&mut self) {
        let instances = self.rects_to_instances();
        if let Some(gpu) = self.gpu.as_mut() {
            gpu.update_instance_buffer(&instances);
        }
    }
}

impl<'window> ApplicationHandler for App<'window> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let win_attr = Window::default_attributes().with_title("wgpu winit example");
            let window = Arc::new(
                event_loop
                    .create_window(win_attr)
                    .expect("create window err."),
            );
            self.window = Some(window.clone());
            let gpu = Gpu::new(window.clone(), self.rects_to_instances());
            self.gpu = Some(gpu);

            self.deno.run_script("src/app.js");
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
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
