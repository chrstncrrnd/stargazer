use macroquad::prelude::*;

pub struct Player{
    pub texture: Texture2D,
    pub width: f32,
    pub height: f32,
    pub name: String,
    pub pos_x: f32,
    pub pos_y: f32,
    pub player_speed: f32
}

enum Direction {
    Forwards,
    Backwards,
    Left,
    Right
}


impl Player{
    //move the player
    fn move_player(&mut self, direction: Direction){
        match direction{
            Direction::Forwards => self.pos_y -= self.player_speed,
            Direction::Backwards => self.pos_y += self.player_speed,
            Direction::Left => self.pos_x -= self.player_speed,
            Direction::Right => self.pos_x += self.player_speed
        }
    }

    pub fn render(&mut self, check_inputs: bool){
        use Direction::*;
        draw_text(self.name.as_str(), self.pos_x, self.pos_y, 10.0, BLACK);
        // draw_texture(self.texture, self.pos_x, self.pos_y, Default::default());
        if check_inputs {
            //move up
            if is_key_down(KeyCode::W){
                self.move_player(Forwards)
            }
                //move down
            else if is_key_down(KeyCode::S){
                self.move_player(Backwards)
            }
            //move left
            else if is_key_down(KeyCode::A){
                self.move_player(Left)
            }
            else if is_key_down(KeyCode::D){
                self.move_player(Right)
            }
        }
    }

}