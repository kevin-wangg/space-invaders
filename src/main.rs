use std::fs;

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
    collided: bool,
}

enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver,
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

const FRAGMENT_SHADER: &str = include_str!("starfield-shader.glsl");

const VERTEX_SHADER: &str = "#version 100
    attribute vec3 position;
    attribute vec2 texcoord;
    attribute vec4 color0;
    varying float iTime;

    uniform mat4 Model;
    uniform mat4 Projection;
    uniform vec4 _Time;

    void main() {
        gl_Position = Projection * Model * vec4(position, 1);
        iTime = _Time.x;
    }
";

#[macroquad::main("Space Invaders")]
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
        collided: false,
    };
    let mut bullets = Vec::new();
    let mut game_state = GameState::MainMenu;
    let mut score: u32 = 0;
    let mut high_score: u32 = fs::read_to_string("highscore.dat")
        .map_or(Ok(0), |i| i.parse::<u32>())
        .unwrap_or(0);

    let mut direction_modifier: f32 = 0.0;
    let render_target = render_target(320, 150);
    render_target.texture.set_filter(FilterMode::Nearest);
    let material = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("iResolution", UniformType::Float2),
                UniformDesc::new("direction_modifier", UniformType::Float1),
            ],
            ..Default::default()
        },
    )
    .unwrap();

    loop {
        clear_background(BLACK);
        material.set_uniform("iResolution", (screen_width(), screen_height()));
        material.set_uniform("direction_modifier", direction_modifier);
        gl_use_material(&material);
        draw_texture_ex(
            &render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        gl_use_default_material();
        match game_state {
            GameState::MainMenu => {
                // Press escape to exit game in the main menu
                if is_key_pressed(KeyCode::Escape) {
                    std::process::exit(0);
                }
                // Press space to start the game
                if is_key_pressed(KeyCode::Space) {
                    squares.clear();
                    bullets.clear();
                    circle.x = screen_width() / 2.0;
                    circle.y = screen_height() / 2.0;
                    score = 0;
                    game_state = GameState::Playing;
                }
                let title_text = "Space Invaders";
                let text = "Press space to start";
                let title_text_dimensions = measure_text(title_text, None, 50, 1.0);
                let text_dimensions = measure_text(text, None, 25, 1.0);
                draw_text(
                    title_text,
                    screen_width() / 2.0 - title_text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    WHITE,
                );
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0 + title_text_dimensions.height,
                    25.0,
                    WHITE,
                );
            }
            GameState::Playing => {
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
                        collided: false,
                    })
                }

                // If gameover, press space to restart game
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Paused;
                }

                // Handle input and movement
                if is_key_down(KeyCode::Left) {
                    circle.x -= circle.speed * delta_time;
                    direction_modifier -= 0.05 * delta_time;
                } else if is_key_down(KeyCode::Right) {
                    circle.x += circle.speed * delta_time;
                    direction_modifier += 0.05 * delta_time;
                }
                if is_key_down(KeyCode::Up) {
                    circle.y -= circle.speed * delta_time;
                } else if is_key_down(KeyCode::Down) {
                    circle.y += circle.speed * delta_time;
                }
                if is_key_pressed(KeyCode::Space) {
                    bullets.push(Shape {
                        typ: ShapeType::Circle,
                        size: 5.0,
                        speed: circle.speed * 2.0,
                        x: circle.x,
                        y: circle.y,
                        collided: false,
                    })
                }

                for bullet in &mut bullets {
                    bullet.y -= bullet.speed * delta_time;
                }
                for square in &mut squares {
                    square.y += square.speed * delta_time;
                }
                squares.retain(|square| square.y <= screen_height() && !square.collided);
                bullets.retain(|bullet| bullet.y > bullet.size && !bullet.collided);
                circle.x = clamp(circle.x, circle.size, screen_width() - circle.size);
                circle.y = clamp(circle.y, circle.size, screen_height() - circle.size);

                // Set gameover to true if circle collides with any square
                if squares.iter().any(|square| square.collides_with(&circle)) {
                    if score == high_score {
                        fs::write("highscore.dat", high_score.to_string()).ok();
                    }
                    game_state = GameState::GameOver;
                }
                for bullet in &mut bullets {
                    for square in &mut squares {
                        if bullet.collides_with(square) {
                            bullet.collided = true;
                            square.collided = true;
                            score += square.size.round() as u32;
                            high_score = high_score.max(score);
                        }
                    }
                }

                // Render everything
                for bullet in &bullets {
                    draw_circle(bullet.x, bullet.y, bullet.size, RED);
                }
                draw_circle(circle.x, circle.y, circle.size, YELLOW);
                for square in &squares {
                    draw_rectangle(square.x, square.y, square.size, square.size, GREEN);
                }
                draw_text(
                    format!("Score: {}", score).as_str(),
                    10.0,
                    35.0,
                    25.0,
                    WHITE,
                );
                let highscore_text = format!("High score: {}", high_score);
                let text_dimensions = measure_text(highscore_text.as_str(), None, 25, 1.0);

                draw_text(
                    highscore_text.as_str(),
                    screen_width() - text_dimensions.width - 10.0,
                    35.0,
                    25.0,
                    WHITE,
                );
            }
            GameState::Paused => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
                }
                let text = "Paused. Press space to unpause";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    WHITE,
                );
            }
            GameState::GameOver => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::MainMenu;
                }
                let text = "GAME OVER";
                let subtext = "Press space to return to main menu";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                let subtext_dimensions = measure_text(subtext, None, 25, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    RED,
                );
                draw_text(
                    subtext,
                    screen_width() / 2.0 - subtext_dimensions.width / 2.0,
                    screen_height() / 2.0 + text_dimensions.height,
                    25.0,
                    RED,
                );
            }
        }
        next_frame().await
    }
}
