use super::*;

pub struct Stick {
    pub position: Vec2<f32>,
    pub state: StickState,
}

pub enum StickState {
    Moving,
    Poking { target: Vec2<f32> },
    Retreating,
}
