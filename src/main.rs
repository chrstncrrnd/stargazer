mod game_views;
mod player_handlers;
mod resources;
mod utils;
mod world_generation;

use crate::game_views::planet_view::block::block::{Block, BlockTypes};
use crate::game_views::planet_view::planet_surface::PlanetSurface;
use crate::player_handlers::player::Player;
use macroquad::prelude::*;
use resources::Resources;
use std::convert::TryInto;

//custom window config
fn window_config() -> Conf {
    Conf {
        fullscreen: false,
        window_height: 800,
        window_width: 1000,
        window_resizable: false,
        window_title: String::from("Stargazers"),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let game_resources = Resources::new().await.unwrap();

    //main character, you can also create other characters with this.
    let mut main_character = Player {
        texture: game_resources.player_texture,
        width: 100.0,
        height: 100.0,
        name: "Main_Character".to_string(),
        position: vec2(100.0, 100.0),
        player_speed: 10.0,
        facing_left: true,
        name_font: game_resources.font,
    };

    let mut main_planet_surface = PlanetSurface {
        render_distance: vec2(
            window_config().window_width as f32,
            window_config().window_height as f32,
        ),
        origin: vec2(0.0, 0.0),
        block: Block {
            texture: Option::Some(game_resources.block_resources.grass),
            block_type: BlockTypes::Grass,
            ..Default::default()
        },
    };

    //main game loop
    loop {
        main_character.texture.set_filter(FilterMode::Nearest);
        clear_background(WHITE);

        main_planet_surface.render();
        main_character.render(true);
        //logic to keep camera over the player
        next_frame().await;
    }
}
