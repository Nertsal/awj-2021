use super::*;

impl GameState {
    pub fn handle_event_impl(&mut self, event: geng::Event) {
        let mouse_world_pos = mouse_world_pos(&self.geng, &self.camera, self.framebuffer_size);

        match event {
            geng::Event::KeyDown { key } => match key {
                geng::Key::Space => match self.stick.state {
                    StickState::Moving => {
                        self.stick.state = StickState::Poking {
                            target: mouse_world_pos,
                        }
                    }
                    StickState::Poking { .. } => self.stick.state = StickState::Retreating,
                    StickState::Retreating => (),
                },
                _ => (),
            },
            _ => (),
        }
    }
}
