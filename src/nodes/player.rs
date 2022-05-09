use macroquad::prelude::*;

pub struct Player {
    pub position: Vec2,
    texture: Texture2D,
    width: f32,
    height: f32,
    name: String,
    player_speed: f32,
    facing_left: bool,
}


impl Player {
    pub fn new(texture: Texture2D, name: String) -> Self{
        Self{
            texture,
            width: 50.0,
            height: 50.0,
            name,
            position: vec2(screen_width()/2., screen_height()/2.),
            player_speed: 100.0,
            facing_left: true,
        }
    }

    pub fn render(&mut self) {
        let speed = self.player_speed * get_frame_time();

        //[w,a,s,d]
        let mut speeds: [f32; 4] = [0.; 4];

        if is_key_down(KeyCode::W){
            speeds[0] = speed;
        }
        if is_key_down(KeyCode::A){
            speeds[1] = speed;
        }
        if is_key_down(KeyCode::S){
            speeds[2] = speed;
        }
        if is_key_down(KeyCode::D){
            speeds[3] = speed;
        }

        self.position.y -= speeds[0];
        self.position.x -= speeds[1];
        self.position.y += speeds[2];
        self.position.x += speeds[3];


        //render the player name
        draw_text_ex(
            self.name.as_str(),
            self.position.x
                - (measure_text(self.name.as_str(), None, 30, 1.0).width
                    - self.width)
                    / 2.0,
            self.position.y - 10.0,
            TextParams {
                font: Default::default(),
                font_size: 30,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
                color: BLACK,
            },
        );

        draw_texture_ex(
            self.texture,
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.height, self.width)),
                flip_x: self.facing_left,
                ..Default::default()
            },
        );
    }
}
