use macroquad::prelude::*;
use rust_translate::translate_from_english;

#[macroquad::main("Super Grass Adventures!")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;
    let seed = get_time();
    rand::srand(seed as u64);
    let text: &str;
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

        text = translate_from_english("hello", "in").unwrap();

        draw_circle(x, y, 15.0, YELLOW);
        draw_text("move the ball with arrow keys", 20.0, 20.0, 20.0, DARKGRAY);
        draw_text(
            &format!{"random number: {}", rand::rand()}[..],
            20.0,
            50.0,
            30.0,
            DARKGREEN,
        );
        dt(text);
        
        next_frame().await
    }
}


fn dt(text: &str) {
    draw_text(text, 20.0, 100.0, 30.0, DARKBLUE);
}