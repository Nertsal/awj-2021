use super::*;

pub struct Stick {
    pub position: Vec2<f32>,
}

impl Stick {
    pub fn point_at(&mut self, target: Vec2<f32>) {
        self.position = target - vec2(0.0, constants::STICK_SIZE.y / 2.0);
    }
}
