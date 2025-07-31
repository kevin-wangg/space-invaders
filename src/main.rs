use macroquad::prelude::*;

#[macroquad::main("Hello world game")]
async fn main() {
    let mut x = 100.0;
    let mut y = 100.0;
    let radius = 40.0;
    let mut dx = 5.0;
    let mut dy = 5.0;

    loop {
        clear_background(WHITE);

        x += dx;
        y += dy;

        if x - radius < 0.0 || x + radius > screen_width() {
            dx = -dx;
        }
        if y - radius < 0.0 || y + radius > screen_height() {
            dy = -dy;
        }

        draw_text("Bouncing ball", 20.0, 40.0, 30.0, BLACK);
        draw_circle(x, y, radius, RED);
        next_frame().await
    }
}
