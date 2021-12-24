use super::*;

#[derive(geng::Assets)]
pub struct Assets {
    pub face: ugli::Texture,
    pub stick: ugli::Texture,
    pub crumb: ugli::Texture,
    #[asset(path = "teeth/top/*.png", range = "1..=5")]
    pub top_teeth: Vec<Rc<ugli::Texture>>,
    #[asset(path = "teeth/bottom/*.png", range = "1..=5")]
    pub bottom_teeth: Vec<Rc<ugli::Texture>>,
    pub config: config::Config,
}

impl Assets {
    pub fn init(&mut self) {
        assert_eq!(
            self.top_teeth.len(),
            self.config.top_teeth_locations.len(),
            "There must be as many textures as there are locations in the config file!"
        );
        assert_eq!(
            self.bottom_teeth.len(),
            self.config.bottom_teeth_locations.len(),
            "There must be as many textures as there are locations in the config file!"
        );

        self.face.set_filter(ugli::Filter::Nearest);
        self.stick.set_filter(ugli::Filter::Nearest);
        self.crumb.set_filter(ugli::Filter::Nearest);
        for tooth in self
            .top_teeth
            .iter_mut()
            .chain(self.bottom_teeth.iter_mut())
        {
            Rc::get_mut(tooth)
                .unwrap()
                .set_filter(ugli::Filter::Nearest);
        }
    }
}
