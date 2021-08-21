mod utils;
mod game_views;
mod player_handlers;

use macroquad::prelude::*;
use crate::game_views::planet_view::planet_surface::PlanetSurface;
use crate::game_views::planet_view::block::block::Block;
use crate::player_handlers::player::Player;
//custom window config
fn window_config() -> Conf{
    Conf{
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



    //main character, you can also create other characters with this.
    let mut main_character = Player {
        texture: load_texture("assets/astronaut_new.png").await.unwrap(),
        width: 60.0,
        height: 60.0,
        name: "Main_Character".to_string(),
        position: vec2(100.0, 100.0),
        player_speed: 10.0,
        facing_left: true,
        name_font: load_ttf_font("assets/Font.ttf").await.unwrap()
    };

    let mut main_planet_surface = PlanetSurface{
        render_distance: vec2(window_config().window_width as f32, window_config().window_height as f32),
        origin: vec2(0.0,0.0),
        block: Block{
            texture: Option::from(load_texture("assets/dirt.png").await.unwrap()),
            ..Default::default()
        }
    };


    //main game loop
    loop {

        clear_background(WHITE);
        main_planet_surface.render();
        main_character.render(true);
        //logic to keep camera over the player
        next_frame().await;
    }
}
