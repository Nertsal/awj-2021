use super::*;

#[derive(geng::Assets)]
pub struct Assets {
    pub face: ugli::Texture,
    pub stick: ugli::Texture,
    pub crumb: ugli::Texture,
    #[asset(path = "teeth/*.png", range = "1..=10")]
    pub teeth: Vec<Rc<ugli::Texture>>,
    pub config: config::Config,
}

impl Assets {
    pub fn init(&mut self) {
        assert_eq!(
            self.teeth.len(),
            self.config.teeth_locations.len(),
            "There must be as many textures as there are locations in the config file!"
        );

        self.face.set_filter(ugli::Filter::Nearest);
        self.stick.set_filter(ugli::Filter::Nearest);
        self.crumb.set_filter(ugli::Filter::Nearest);
        for tooth in &mut self.teeth {
            Rc::get_mut(tooth)
                .unwrap()
                .set_filter(ugli::Filter::Nearest);
        }
    }
}
