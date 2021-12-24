use super::*;

pub struct Tooth {
    pub texture: Rc<ugli::Texture>,
    pub position: Vec2<f32>,
    pub state: ToothState,
}

pub enum ToothState {
    Healthy,
}
