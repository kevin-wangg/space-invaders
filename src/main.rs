use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

struct App {
    pixels: Option<Pixels<'static>>,
}

impl App {
    fn new() -> Self {
        Self {
            pixels: None,
        }
    }

    fn draw(&mut self) {
        if let Some(ref mut pixels) = self.pixels {
            let frame = pixels.frame_mut();

            for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                let x = i % WIDTH as usize;
                let y = i / WIDTH as usize;

                if x > 200 && x < 600 && y > 200 && y < 400 {
                    pixel[0] = 255;
                } else {
                    pixel[0] = 0;
                }
                pixel[3] = 255; // Alpha
            }

            pixels.render().unwrap();
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_title("Space Invaders")
                    .with_inner_size(LogicalSize::new(WIDTH, HEIGHT)),
            )
            .unwrap();

        let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, window);
        let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();

        self.pixels = Some(pixels);
        self.draw();
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

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}
