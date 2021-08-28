use macroquad::prelude::*;

use nodes::camera::Camera;
use nodes::player::Player;
use resources::Resources;

use crate::scenes::planet_surface_scene::renderer::Renderer;
use crate::world_generation::terrain::{AlphaTerrain, GrassOnly};
use bracket_noise::prelude::{FastNoise, NoiseType};

mod nodes;
mod resources;
mod scenes;
mod world_generation;

//custom window config
fn window_config() -> Conf {
    Conf {
        fullscreen: false,
        window_height: 800,
        window_width: 1920,
        window_title: String::from("Stargazers"),
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let game_resources: Resources = Resources::new().await.unwrap();

    //main character, you can also create other characters with this.
    let mut main_character = Player {
        texture: game_resources.player_texture,
        width: 50.0,
        height: 50.0,
        name: "Chrstn".to_string(),
        position: vec2(screen_width() / 2., screen_height() / 2.),
        player_speed: 5.0,
        facing_left: true,
        name_font: game_resources.font,
    };

    let mut planet_surface = Renderer {
        blocks: Vec::new(),
        blocksize: 40,
        terrain_generator: Box::new(AlphaTerrain { noise: None }),
    };

    planet_surface.init();

    let mut main_camera = Camera::default();

    //main game loop
    loop {
        clear_background(WHITE);
        main_camera.update(&main_character);
        set_camera(&main_camera.macroquad_camera);
        planet_surface.render(&main_character, &game_resources.block_resources);
        main_character.render(true);
        next_frame().await;
    }
}
