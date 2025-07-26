mod app;
mod sprite;
mod world;

use crate::app::App;

use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new().unwrap();
    event_loop.run_app(&mut app).unwrap();
}
