use super::*;

impl GameState {
    pub fn tick(&mut self) {
        self.diagram.tick();
    }
}
