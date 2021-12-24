use super::*;

pub struct Tooth {
    pub texture: Rc<ugli::Texture>,
    pub position: Vec2<f32>,
    pub state: ToothState,
}

impl Tooth {
    pub fn poke_box(&self, config: &Config) -> AABB<f32> {
        AABB::point(self.position).extend_symmetric(config.tooth_size / 2.0)
    }

    pub fn hurt_box(&self, config: &Config) -> AABB<f32> {
        self.poke_box(config).extend_symmetric(vec2(-config.tooth_edge_size, 0.0))
    }
}

pub enum ToothState {
    Healthy,
}
