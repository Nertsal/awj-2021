use super::*;

pub struct Teeth {
    pub top: Vec<Option<Tooth>>,
    pub bottom: Vec<Option<Tooth>>,
}

impl Teeth {
    pub fn get_tooth(&self, position: ToothPosition) -> Option<&Tooth> {
        match position {
            ToothPosition::Top(index) => self.top.get(index),
            ToothPosition::Bottom(index) => self.bottom.get(index),
        }
        .and_then(|t| t.as_ref())
    }

    pub fn get_row_len(&self, position: ToothPosition) -> usize {
        match position {
            ToothPosition::Top(_) => self.top.len(),
            ToothPosition::Bottom(_) => self.bottom.len(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Tooth> {
        self.top
            .iter()
            .filter_map(|t| t.as_ref())
            .chain(self.bottom.iter().filter_map(|t| t.as_ref()))
    }

    pub fn iter_raw_mut(&mut self) -> impl Iterator<Item = &mut Option<Tooth>> {
        self.top.iter_mut().chain(self.bottom.iter_mut())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ToothPosition {
    Top(usize),
    Bottom(usize),
}

impl ToothPosition {
    pub fn get_index(self) -> usize {
        match self {
            ToothPosition::Top(i) => i,
            ToothPosition::Bottom(i) => i,
        }
    }

    pub fn indexed(self, index: usize) -> Self {
        match self {
            ToothPosition::Top(_) => Self::Top(index),
            ToothPosition::Bottom(_) => Self::Bottom(index),
        }
    }

    pub fn opposite(self) -> Self {
        match self {
            ToothPosition::Top(i) => Self::Bottom(i),
            ToothPosition::Bottom(i) => Self::Top(i),
        }
    }
}

pub struct Tooth {
    pub texture: Rc<ugli::Texture>,
    pub position: Vec2<f32>,
    pub is_loose: bool,
    pub is_ill: bool,
}

impl Tooth {
    pub fn poke_box(&self, config: &Config) -> AABB<f32> {
        AABB::point(self.position).extend_symmetric(config.tooth_size / 2.0)
    }

    pub fn hurt_box(&self, config: &Config) -> AABB<f32> {
        self.poke_box(config)
            .extend_symmetric(vec2(-config.tooth_edge_size, 0.0))
    }
}
