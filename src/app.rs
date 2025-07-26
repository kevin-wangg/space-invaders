use std::collections::HashSet;
use std::sync::Arc;
use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::sprite::Sprite;
use crate::world::{World, WorldBuilder};

use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;
const FRAME_RATE: u64 = 60;

pub struct App {
    pixels: Option<Pixels<'static>>,
    window: Option<Arc<Window>>,
    world: World,
    pressed_keys: HashSet<PhysicalKey>,
    // Separate thread that issues window redraw requests at a rate equal to the frame rate
    redraw_thread: Option<JoinHandle<()>>,
    // Sender that is used to communicate shutdown to the redraw_thread
    shutdown_tx: Option<Sender<()>>,
}

impl App {
    pub fn new() -> Result<Self, String> {
        let player_sprite = Sprite::new(400, 500, 20, 20);
        let world = WorldBuilder::new().with_player(player_sprite).build()?;

        Ok(Self {
            pixels: None,
            window: None,
            world,
            pressed_keys: HashSet::new(),
            redraw_thread: None,
            shutdown_tx: None,
        })
    }

    fn draw(&mut self) {
        if let Some(ref mut pixels) = self.pixels {
            let frame = pixels.frame_mut();
            for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                let x = i as u32 % WIDTH;
                let y = i as u32 / WIDTH;
                self.world.set_pixel(x, y, pixel);
            }
            pixels.render().unwrap();
        }
    }

    fn handle_keyboard_input(&mut self, key: PhysicalKey, state: ElementState) {
        match state {
            ElementState::Pressed => {
                self.pressed_keys.insert(key);
            }
            ElementState::Released => {
                self.pressed_keys.remove(&key);
            }
        }
    }

    fn update(&mut self) {
        let mut dx = 0;
        for key in &self.pressed_keys {
            if let PhysicalKey::Code(code) = key {
                match code {
                    KeyCode::ArrowLeft | KeyCode::KeyA => dx -= 5,
                    KeyCode::ArrowRight | KeyCode::KeyD => dx += 5,
                    _ => {}
                }
            }
        }
        self.world.move_player(dx);
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("Space Invaders")
                        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT)),
                )
                .unwrap(),
        );
        let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, Arc::clone(&window));
        let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();
        // Create a sender and receiver for channel communication with the redraw thread
        let (shutdown_tx, shutdown_rx) = mpsc::channel();
        self.window = Some(Arc::clone(&window));
        self.pixels = Some(pixels);
        self.shutdown_tx = Some(shutdown_tx);
        self.redraw_thread = Some(thread::spawn(move || {
            loop {
                if shutdown_rx.try_recv().is_ok() {
                    break;
                } else {
                    window.request_redraw();
                    thread::sleep(Duration::from_millis(1000 / FRAME_RATE));
                }
            }
        }));
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
                if let Some(shutdown_tx) = self.shutdown_tx.take() {
                    shutdown_tx.send(()).unwrap();
                }
                if let Some(redraw_thread) = self.redraw_thread.take() {
                    redraw_thread.join().unwrap()
                }
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.update();
                self.draw();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                self.handle_keyboard_input(event.physical_key, event.state);
            }
            _ => {}
        }
    }
}
