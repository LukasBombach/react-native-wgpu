use crate::app::App;
use winit::error::EventLoopError;
use winit::event_loop::{ControlFlow, EventLoop};

mod app;
mod gpu;

fn main() -> Result<(), EventLoopError> {
    let mut app = App::new();
    app.add_rect(100, 100, 250, 250);
    app.add_rect(200, 150, 250, 250);
    app.add_rect(300, 200, 250, 250);

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app)
}
