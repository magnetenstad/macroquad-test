use macroquad::prelude::*;

const SPEED: f32  = 3.0;

#[macroquad::main("macroquad-test")]
async fn main() {

    let mut x = 0.0;
    let mut y = 0.0;
    let image_player = load_texture("assets/player.png").await.unwrap();
    
    loop {
        y -= is_key_down(KeyCode::W) as i32 as f32 * SPEED;
        x -= is_key_down(KeyCode::A) as i32 as f32 * SPEED;
        y += is_key_down(KeyCode::S) as i32 as f32 * SPEED;
        x += is_key_down(KeyCode::D) as i32 as f32 * SPEED;

        clear_background(BLACK);
        draw_texture(image_player, x, y, WHITE);
        
        draw_text(&get_frame_time().to_string(), 16.0, 32.0, 16.0, WHITE);
        draw_text(&get_fps().to_string(), 16.0, 16.0, 16.0, WHITE);

        next_frame().await
    }
}
