use macroquad::prelude::*;

enum ShapeType {
    Circle,
    Rect,
}

struct Shape {
    typ: ShapeType,
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
}

impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    fn rect(&self) -> Rect {
        match self.typ {
            ShapeType::Circle => Rect {
                x: self.x - self.size,
                y: self.y - self.size,
                w: self.size * 2.0,
                h: self.size * 2.0,
            },
            ShapeType::Rect => Rect {
                x: self.x,
                y: self.y,
                w: self.size,
                h: self.size,
            },
        }
    }
}

#[macroquad::main("Hello world game")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);
    const SPEED: f32 = 300.0;
    let mut squares = Vec::new();
    let mut circle = Shape {
        typ: ShapeType::Circle,
        size: 16.0,
        speed: SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };
    let mut gameover = false;

    loop {
        clear_background(DARKPURPLE);
        let delta_time = get_frame_time();

        // Randomly generate the squares
        if rand::gen_range(0, 99) > 95 {
            let size = rand::gen_range(16.0, 64.0);
            let speed = rand::gen_range(50.0, 150.0);
            let x = rand::gen_range(0.0, screen_width() - size);
            squares.push(Shape {
                typ: ShapeType::Rect,
                size,
                speed,
                x,
                y: -size,
            })
        }

        if gameover && is_key_pressed(KeyCode::Space) {
            gameover = false;
            squares.clear();
            circle.x = screen_width() / 2.0;
            circle.y = screen_height() / 2.0;
        }

        // Handle input for the circle
        if !gameover {
            if is_key_down(KeyCode::Left) {
                circle.x -= circle.speed * delta_time;
            } else if is_key_down(KeyCode::Right) {
                circle.x += circle.speed * delta_time;
            }
            if is_key_down(KeyCode::Up) {
                circle.y -= circle.speed * delta_time;
            } else if is_key_down(KeyCode::Down) {
                circle.y += circle.speed * delta_time;
            }

            for square in &mut squares {
                square.y += square.speed * delta_time;
            }
            squares.retain(|square| square.y <= screen_height());
            circle.x = clamp(circle.x, circle.size, screen_width() - circle.size);
            circle.y = clamp(circle.y, circle.size, screen_height() - circle.size);
        }

        // Set gameover to true if circle collides with any square
        if squares.iter().any(|square| square.collides_with(&circle)) {
            gameover = true;
        }

        // Render everything
        draw_circle(circle.x, circle.y, circle.size, YELLOW);
        for square in &squares {
            draw_rectangle(square.x, square.y, square.size, square.size, GREEN);
        }

        if gameover {
            let text = "GAME OVER";
            let text_dimensions = measure_text(text, None, 50, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dimensions.width / 2.0,
                screen_height() / 2.0,
                50.0,
                RED
            );
        }

        next_frame().await
    }
}
