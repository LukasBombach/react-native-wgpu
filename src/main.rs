mod app;
mod deno;
mod gpu;

use std::sync::Arc;
use std::sync::Mutex;

use winit::error::EventLoopError;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;

use crate::app::App;
use crate::app::Rect;

#[derive(Debug, Clone, Copy)]
pub enum JavaScriptAction {
    AddRect(Rect),
}

fn main() -> Result<(), EventLoopError> {
    // let event_loop = EventLoop::new()?;
    let event_loop = EventLoop::<JavaScriptAction>::with_user_event().build()?;
    let proxy = event_loop.create_proxy();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new(Arc::new(Mutex::new(proxy)));
    app.add_rect(100, 100, 250, 250);
    app.add_rect(200, 150, 250, 250);
    app.add_rect(300, 200, 250, 250);

    event_loop.run_app(&mut app)
}
