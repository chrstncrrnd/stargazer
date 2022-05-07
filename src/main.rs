use std::fmt::format;
use macroquad::prelude::*;

use nodes::player::Player;
use resources::Resources;

use crate::scenes::planet_surface_scene::renderer::Renderer;
use crate::world::terrain::{AlphaTerrain, BetterTerrain, GrassOnly};

mod nodes;
mod resources;
mod scenes;
mod world;

#[allow(dead_code)]
//custom window config
fn window_config() -> Conf {
    Conf {
        fullscreen: false,
        window_height: 800,
        window_width: 1000,
        window_resizable: false,
        window_title: String::from("Stargazers"),
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let game_resources: Resources = Resources::new().await.unwrap();

    //main character, you can also create other characters with this.
    let mut player = Player {
        texture: game_resources.player_texture,
        width: 50.0,
        height: 50.0,
        name: "Chrstn".to_string(),
        position: vec2(screen_width() / 2., screen_height() / 2.),
        player_speed: 5.0,
        facing_left: true,
        name_font: game_resources.font,
    };

    let mut planet_surface = Renderer::new(Box::new(BetterTerrain::new()));

    let mut camera =
        Camera2D::from_display_rect(Rect::new(0., 0., screen_width(), screen_height()));


    //main game loop
    loop {
        clear_background(WHITE);
        planet_surface.render(&player, &game_resources.block_resources);
        player.render(true);
        camera.target = player.position;
        set_camera(&camera);

        draw_text(format!("XY: {}, {}", player.position.x, player.position.y).as_str(), player.position.x - 100_f32, player.position.y - 100_f32, 60.0, WHITE);
        next_frame().await;
    }
}
