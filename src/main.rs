use std::sync::Arc;
use std::sync::Mutex;

use winit::error::EventLoopError;
use winit::event_loop::EventLoop;

use crate::app::App;
use crate::app::Rect;

mod app;
mod deno;
mod gpu;

#[derive(Debug, Clone, Copy)]
pub enum JsEvents {
    AddRect(Rect),
}

fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::<JsEvents>::with_user_event().build()?;
    let proxy = Arc::new(Mutex::new(event_loop.create_proxy()));

    let mut app = App::new(proxy);
    app.add_rect(100, 100, 250, 250);
    app.add_rect(200, 150, 250, 250);
    app.add_rect(300, 200, 250, 250);

    event_loop.run_app(&mut app)
}
