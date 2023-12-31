use macroquad::prelude::*;

pub struct Ship {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub speed: f32,
    pub color: Color,
    pub gameover: bool,
}

impl Ship {
    pub fn left(&mut self) {
        self.x -= self.speed;
    }

    pub fn right(&mut self) {
        self.y -= self.speed;
    }

    pub fn draw(&mut self, texture: &Texture2D) {
        draw_texture(*texture, self.x, self.y, self.color)
    }
}
