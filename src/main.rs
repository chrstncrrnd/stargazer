mod player;
mod planet_all;
mod utils;
mod planet_surface_all;

use macroquad::prelude::*;
use planet_all::planet::Planet;
use player::Player;

pub enum GameState{
    Menu,
    Loading,
    Playing,
    Won,
    Lost
}

//custom window config
fn window_config() -> Conf{
    Conf{
        fullscreen: false,
        window_height: 800,
        window_width: 1000,
        window_title: String::from("Stargazers"),
        ..Default::default()

    }
}

#[macroquad::main(window_config)]
async fn main() {
    let background = render_target(macroquad::window::screen_width() as u32, macroquad::window::screen_height() as u32);

    //main character, you can also create other characters with this.
    let mut main_character = Player {
        texture: load_texture("assets/astronaut_new.png").await.unwrap(),
        width: 60.0,
        height: 60.0,
        name: "Main_Character".to_string(),
        pos_x: 500.0,
        pos_y: 500.0,
        player_speed: 10.0,
        facing_left: true,
        name_font: load_ttf_font("assets/Font.ttf").await.unwrap()
    };


    //main game loop
    loop {

        clear_background(WHITE);
        main_character.render(true);
        //logic to keep camera over the player

        next_frame().await;
    }
}
