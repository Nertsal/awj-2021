use super::*;

#[derive(geng::Assets)]
pub struct Assets {
    pub face: ugli::Texture,
    pub stick: ugli::Texture,
}

impl Assets {
    pub fn init(&mut self) {
        self.face.set_filter(ugli::Filter::Nearest);
        self.stick.set_filter(ugli::Filter::Nearest);
    }
}
