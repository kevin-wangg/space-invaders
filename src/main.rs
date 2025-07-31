use macroquad::prelude::*;

#[macroquad::main("Hello world game")]
async fn main() {
    loop {
        clear_background(WHITE);
        draw_text("Hello Macroquad!", 20.0, 40.0, 30.0, BLACK);
        next_frame().await
    }
}
