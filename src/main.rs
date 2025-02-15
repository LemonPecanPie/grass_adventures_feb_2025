use macroquad::prelude::*;

#[macroquad::main("Super Grass Adventures!")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;
    loop {
        clear_background(LIGHTGRAY);

        if is_key_down(KeyCode::Right) {
            x += 1.0;
        }

        if is_key_down(KeyCode::Left) {
            x -= 1.0;
        }

        if is_key_down(KeyCode::Down) {
            y += 1.0;
        }

        if is_key_down(KeyCode::Up) {
            y -= 1.0;
        }

        draw_circle(x, y, 15.0, YELLOW);
        draw_text("move the ball with arrow keys", 20.0, 20.0, 20.0, DARKGRAY);
        draw_text(
            &format!("time elapsed: {}", get_time())[..],
            20.0,
            50.0,
            30.0,
            SKYBLUE,
        );
        next_frame().await
    }
}
