use macroquad::prelude::*;

pub struct Player {
    pub texture: Texture2D,
    pub width: f32,
    pub height: f32,
    pub name: String,
    pub position: Vec2,
    pub player_speed: f32,
    pub facing_left: bool,
    pub name_font: Font,
}

enum Direction {
    Forwards,
    Backwards,
    Left,
    Right,
}

impl Player {
    //move the player
    fn move_player(&mut self, direction: Direction) {
        match direction {
            Direction::Forwards => self.position.y -= self.player_speed,
            Direction::Backwards => self.position.y += self.player_speed,
            Direction::Left => self.position.x -= self.player_speed,
            Direction::Right => self.position.x += self.player_speed,
        }
    }

    pub fn render(&mut self, check_inputs: bool) {
        use Direction::*;
        if check_inputs {

            //TODO: make the character be able to go sideways without it going so fast lol
            //move up
            if is_key_down(KeyCode::W) {
                self.move_player(Forwards);
            }
            //move down
            else if is_key_down(KeyCode::S) {
                self.move_player(Backwards);
            }
            //move left
            else if is_key_down(KeyCode::A) {
                self.move_player(Left);
                self.facing_left = true;
            }
            else if is_key_down(KeyCode::D) {
                self.move_player(Right);
                self.facing_left = false;
            }
        }

        //render the player name
        draw_text_ex(
            self.name.as_str(),
            self.position.x
                - (measure_text(self.name.as_str(), Option::from(self.name_font), 30, 1.0).width
                    - self.width)
                    / 2.0,
            self.position.y - 10.0,
            TextParams {
                font: self.name_font,
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
