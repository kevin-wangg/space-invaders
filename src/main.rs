use macroquad::prelude::*;

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
}

#[macroquad::main("Hello world game")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);
    const SPEED: f32 = 300.0;
    // let mut squares = Vec::new();
    let mut circle = Shape {
        size: 16.0,
        speed: SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };

    loop {
        clear_background(DARKPURPLE);
        let delta_time = get_frame_time();

        if is_key_down(KeyCode::Left) {
            circle.x -= SPEED * delta_time;
        } else if is_key_down(KeyCode::Right) {
            circle.x += SPEED * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            circle.y -= SPEED * delta_time;
        } else if is_key_down(KeyCode::Down) {
            circle.y += SPEED * delta_time;
        }

        draw_circle(circle.x, circle.y, circle.size, YELLOW);
        next_frame().await
    }
}
