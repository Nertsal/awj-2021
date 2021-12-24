use super::*;

pub struct Crumb {
    pub tooth_position: ToothPosition,
    /// Local position in range (0..=1, 0..=1)
    pub local_position: Vec2<f32>,
    pub target: CrumbTarget,
}

impl Crumb {
    pub fn world_position(&self, teeth: &Teeth, config: &Config) -> Vec2<f32> {
        self.local_position * config.tooth_size
            + teeth
                .get_tooth(self.tooth_position)
                .unwrap()
                .poke_box(config)
                .bottom_left()
    }
}

pub enum CrumbTarget {
    Local(Vec2<f32>),
    ToothHorizontal { y: f32, left: bool },
    ToothVertical { x: f32 },
}
