use winit::error::EventLoopError;
use winit::event_loop::EventLoop;

use crate::app::App;
use crate::deno::run_script;

mod app;
mod deno;
mod graphics;

fn main() -> Result<(), EventLoopError> {
    let mut app = App::new();
    let event_loop = EventLoop::new().unwrap();

    run_script(app.state.clone(), "src/main.js");

    event_loop.run_app(&mut app)
}
