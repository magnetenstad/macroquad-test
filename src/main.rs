use macroquad::prelude::*;

const SPEED: f32  = 3.0;
const PNG_PLAYER: &[u8] = include_bytes!("../assets/player.png");

#[macroquad::main("macroquad-test")]
async fn main() {

    let mut x = 0.0;
    let mut y = 0.0;
    let tex_player = Texture2D::from_file_with_format(
        PNG_PLAYER, Some(ImageFormat::Png));
    
    fn is_moving_x() -> bool { is_key_down(KeyCode::W) || is_key_down(KeyCode::A) }
    fn is_moving_y() -> bool { is_key_down(KeyCode::S) || is_key_down(KeyCode::D) }
    fn is_moving() -> bool { is_moving_x() || is_moving_y() }

    loop {
        y -= is_key_down(KeyCode::W) as i32 as f32 * SPEED;
        x -= is_key_down(KeyCode::A) as i32 as f32 * SPEED;
        y += is_key_down(KeyCode::S) as i32 as f32 * SPEED;
        x += is_key_down(KeyCode::D) as i32 as f32 * SPEED;

        clear_background(BLACK);
        draw_texture_ex(tex_player, x, y, WHITE, DrawTextureParams {
            dest_size: Some(Vec2 {x: 32.0, y: 32.0}),
            source: None,
            rotation: if is_moving() { ((get_time() * 10.0).sin() / 2.0) as f32 } else { 0.0 },
            flip_x: is_key_down(KeyCode::A),
            flip_y: false,
            pivot: None,
        });
        
        draw_text(&get_frame_time().to_string(), 16.0, 32.0, 16.0, WHITE);
        draw_text(&get_fps().to_string(), 16.0, 16.0, 16.0, WHITE);

        next_frame().await
    }
}
