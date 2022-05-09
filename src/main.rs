use std::process::exit;
use macroquad::prelude::*;

use nodes::player::Player;
use resources::Resources;

use crate::scenes::planet_surface::renderer::Renderer;
use crate::world::terrain::{BetterTerrain};

mod nodes;
mod resources;
mod scenes;
mod world;

#[allow(dead_code)]
//custom window config
fn window_config() -> Conf {
    Conf {
        fullscreen: true,
        window_height: 1080,
        window_width: 1920,
        window_resizable: false,
        window_title: String::from("Stargazers"),
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let game_resources: Resources = Resources::load().await.unwrap();

    //main character, you can also create other characters with this.
    let mut player = Player::new(game_resources.player_texture, "Chrstn".to_owned());

    let mut planet_surface = Renderer::new(Box::new(BetterTerrain::new()));

    let mut camera =
        Camera2D::from_display_rect(Rect::new(0., 0., screen_width(), screen_height()));


    //main game loop
    loop {
        clear_background(WHITE);
        planet_surface.render(&player, &game_resources.block_resources);
        player.render();
        camera.target = player.position;
        set_camera(&camera);
        draw_text(get_fps().to_string().as_str(), player.position.x - 100_f32, player.position.y - 200_f32, 20., WHITE);
        draw_text(format!("XY: {}, {}", player.position.x, player.position.y).as_str(), player.position.x - 100_f32, player.position.y - 100_f32, 60.0, WHITE);
        if is_key_down(KeyCode::Escape){
            exit(0);
        }

        next_frame().await;
    }
}
