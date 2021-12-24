use super::*;

#[derive(geng::Assets)]
pub struct Assets {
    pub face: ugli::Texture,
    pub stick: ugli::Texture,
    #[asset(path = "teeth/*.png", range = "1..=10")]
    pub teeth: Vec<Rc<ugli::Texture>>,
    #[asset(path = "teeth_config.json")]
    pub teeth_config: String,
}

impl Assets {
    pub fn init(&mut self) {
        self.face.set_filter(ugli::Filter::Nearest);
        self.stick.set_filter(ugli::Filter::Nearest);
        for tooth in &mut self.teeth {
            Rc::get_mut(tooth)
                .unwrap()
                .set_filter(ugli::Filter::Nearest);
        }
    }
}
