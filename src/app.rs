use std::sync::Arc;

use crate::world::World;

use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub struct App {
    pixels: Option<Pixels<'static>>,
    window: Option<Arc<Window>>,
    world: Option<World>
}

impl App {
    pub fn new() -> Self {
        Self {
            pixels: None,
            window: None,
            world: None,
        }
    }

    fn draw(&mut self) {
        if let Some(ref mut pixels) = self.pixels {
            let frame = pixels.frame_mut();

            for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                let x = i as u32 % WIDTH;
                let y = i as u32 / WIDTH;

                if x > 200 && x < 600 && y > 200 && y < 400 {
                    pixel[0] = 255;
                    pixel[1] = 255;
                    pixel[2] = 255;
                }
                pixel[3] = 255; // Alpha
            }

            pixels.render().unwrap();
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(event_loop
            .create_window(
                Window::default_attributes()
                    .with_title("Space Invaders")
                    .with_inner_size(LogicalSize::new(WIDTH, HEIGHT)),
            )
            .unwrap());

        self.window = Some(Arc::clone(&window));

        let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, Arc::clone(&window));
        let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();

        self.pixels = Some(pixels);
        self.window.as_ref().unwrap().request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Closing window!");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.draw();
            }
            _ => {}
        }
    }
}
