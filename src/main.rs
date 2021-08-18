mod player;
use player as player_entity;
use macroquad::prelude::*;

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
    //main character, you can also create other characters with this.
    let mut main_character = player::Player {
        texture: Texture2D::empty(),
        width: 10.0,
        height: 10.0,
        name: "Main_Character".to_string(),
        pos_x: 500.0,
        pos_y: 500.0,
        player_speed: 10.0
    };

    //main game loop
    loop {
        clear_background(WHITE);
        main_character.render(true);
        next_frame().await;
    }
}