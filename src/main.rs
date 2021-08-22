use bracket_noise::prelude::{CellularDistanceFunction, CellularReturnType, FastNoise, FractalType, NoiseType};
use macroquad::prelude::*;

use nodes::camera::Camera;
use nodes::player::Player;
use resources::Resources;
use scenes::planet_surface_scene::planet_surface;

use crate::scenes::planet_surface_scene::planet_surface::PlanetSurface;

mod resources;
mod world_generation;
mod nodes;
mod scenes;

//custom window config
fn window_config() -> Conf {
    Conf {
        fullscreen: false,
        window_height: 800,
        window_width: 1000,
        window_title: String::from("Stargazers"),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let game_resources: Resources = Resources::new().await.unwrap();

    //main character, you can also create other characters with this.
    let mut main_character = Player {
        texture: game_resources.player_texture,
        width: 100.0,
        height: 100.0,
        name: "Main_Character".to_string(),
        position: vec2(screen_width()/2., screen_height()/2.),
        player_speed: 10.0,
        facing_left: true,
        name_font: game_resources.font,
    };

    let mut noise = FastNoise::seeded(crate::world_generation::seed::get_seed());
    noise.set_noise_type(NoiseType::Perlin);
    noise.set_frequency(2.0);


    let mut main_planet_surface = PlanetSurface {noise};
    let mut main_camera = Camera::default();


    //main game loop
    loop {
        main_camera.update(&main_character);
        set_camera(&main_camera.macroquad_camera);
        clear_background(WHITE);
        main_planet_surface.render(&game_resources.block_resources, &main_character);
        main_character.render(true);
        //logic to keep camera over the player
        next_frame().await;
    }
}
