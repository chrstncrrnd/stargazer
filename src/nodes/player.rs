use macroquad::prelude::*;
use std::f32::consts::SQRT_2;

pub struct Player {
    pub position: Vec2,
    texture: Texture2D,
    width: f32,
    height: f32,
    name: String,
    player_speed: f32,
    facing_left: bool,
}

enum Direction {
    Forwards,
    Backwards,
    Left,
    Right,
    BackwardsLeft,
    BackwardsRight,
    ForwardsLeft,
    ForwardsRight,
}


impl Player {
    pub fn new(texture: Texture2D, name: String) -> Self{
        Self{
            texture,
            width: 50.0,
            height: 50.0,
            name,
            position: vec2(screen_width()/2., screen_height()/2.),
            player_speed: 5.0,
            facing_left: true,
        }
    }

    //move the player
    fn move_player(&mut self, direction: Direction) {
        match direction {
            Direction::Forwards => self.position.y -= self.player_speed,
            Direction::Backwards => self.position.y += self.player_speed,
            Direction::Left => self.position.x -= self.player_speed,
            Direction::Right => self.position.x += self.player_speed,
            Direction::ForwardsLeft => {
                self.position.y -= (self.player_speed / SQRT_2);
                self.position.x -= (self.player_speed / SQRT_2);
            },
            Direction::ForwardsRight => {
                self.position.x += (self.player_speed / SQRT_2);
                self.position.y -= (self.player_speed / SQRT_2);
            },
            Direction::BackwardsLeft => {
                self.position.x -= (self.player_speed / SQRT_2);
                self.position.y += (self.player_speed / SQRT_2);
            },
            Direction::BackwardsRight => {
                self.position.x += (self.player_speed / SQRT_2);
                self.position.y += (self.player_speed / SQRT_2);
            },
        }
    }

    pub fn render(&mut self, check_inputs: bool) {
        use Direction::*;
        if check_inputs {
            //move up
            if is_key_down(KeyCode::W) {
                //
                if is_key_down(KeyCode::A){
                    self.move_player(ForwardsLeft)
                } else if is_key_down(KeyCode::D) {
                    self.move_player(ForwardsRight)
                }
                else{
                    self.move_player(Forwards);
                }
            }
            //move down
            else if is_key_down(KeyCode::S) {
                if is_key_down(KeyCode::A){
                    self.move_player(BackwardsLeft)
                }else if is_key_down(KeyCode::D) {
                    self.move_player(BackwardsRight)
                }
                else{
                    self.move_player(Backwards);
                }
            }
            //move left
            else if is_key_down(KeyCode::A) {
                self.move_player(Left);
                self.facing_left = true;
            }
            //move right
            else if is_key_down(KeyCode::D) {
                self.move_player(Right);
                self.facing_left = false;
            }
        }

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
