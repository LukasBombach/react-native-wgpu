use std::sync::Arc;
use std::sync::Mutex;

use winit::error::EventLoopError;
use winit::event_loop::EventLoop;

use crate::app::App;
use crate::app::Js;
use crate::javascript_runtime::run_script;

mod app;
mod gpu;
mod javascript_runtime;
mod user_interface;

fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::<Js>::with_user_event().build()?;
    let event_loop_proxy = Arc::new(Mutex::new(event_loop.create_proxy()));
    let mut app = App::new(event_loop_proxy);

    run_script(app.state.clone(), "src/main.tsx");

    event_loop.run_app(&mut app)
}
