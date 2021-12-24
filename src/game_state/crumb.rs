use super::*;

pub struct Crumb {
    pub tooth_position: usize,
    /// Local position in range (0..=1, 0..=1)
    pub local_position: Vec2<f32>,
}

impl Crumb {
    pub fn world_position(&self, teeth: &Vec<Tooth>, config: &Config) -> Vec2<f32> {
        self.local_position * config.tooth_size + teeth[self.tooth_position].position
    }
}
