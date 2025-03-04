use std::sync::Arc;
use std::sync::Mutex;

use winit::error::EventLoopError;
use winit::event_loop::EventLoop;

use crate::app::App;
use crate::app::JsEvents;
use crate::deno::run_script;

mod app;
mod deno;
mod gpu;

fn main() -> Result<(), EventLoopError> {
    let mut app = App::new();

    let event_loop = EventLoop::<JsEvents>::with_user_event().build()?;
    let proxy = Arc::new(Mutex::new(event_loop.create_proxy()));

    run_script(proxy, "src/main.js");

    event_loop.run_app(&mut app)
}
