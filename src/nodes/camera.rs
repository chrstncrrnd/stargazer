use macroquad::prelude::*;

use crate::nodes::player::Player;

pub struct Camera {
    pub bounds: Rect,
    pub macroquad_camera: Camera2D,
}


impl Default for Camera {
    fn default() -> Self {
        Camera {
            bounds: Rect::new(0., 0., 1000.0, 1000.0),
            macroquad_camera: Camera2D::from_display_rect(Rect::new(
                0.,
                0.,
                screen_width(),
                screen_height(),
            )),
        }
    }
}

impl Camera {
    pub fn update(&mut self, player: &Player) {
        self.macroquad_camera = Camera2D::from_display_rect(Rect::new(
            player.position.x - screen_width() / 2.,
            player.position.y - screen_height() / 2.,
            screen_width(),
            screen_height(),
        ))
    }
}
