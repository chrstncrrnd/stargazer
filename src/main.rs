mod player;
mod planet_all;

use macroquad::prelude::*;
use planet_all::planet::Planet;


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



    let main_character_texture = load_texture("assets/astronaut.png").await.unwrap();

    //main character, you can also create other characters with this.
    let mut main_character = player::Player {
        texture: main_character_texture,
        width: 10.0,
        height: 10.0,
        name: "Main_Character".to_string(),
        pos_x: 500.0,
        pos_y: 500.0,
        player_speed: 10.0
};

    let planet = Planet{
        color: Color::new(329.0, 132.0, 0.0, 255.0),
        radius: 100.0,
        pos_y: 300.0,
        pos_x: 500.0
    };

    //main game loop
    loop {
        clear_background(WHITE);
        planet.render();
        main_character.render(true);
        next_frame().await;
    }
}