use macroquad::prelude::*;

fn custom_config() -> Conf{
    Conf{
        fullscreen: false,
        window_height: 1080,
        window_width: 1920,
        window_title: String::from("Stargazers"),
        ..Default::default()

    }
}
#[macroquad::main(custom_config)]
async fn main() {
    loop {
        clear_background(RED);

        next_frame().await
    }
}