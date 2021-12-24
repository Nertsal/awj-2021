use super::*;

impl GameState {
    pub fn update_impl(&mut self, delta_time: f32) {
        // Move stick under the mouse
        let mouse_pos = self.geng.window().mouse_pos().map(|x| x as f32);
        let mouse_world_pos = self
            .camera
            .screen_to_world(self.framebuffer_size, mouse_pos);
        self.stick.point_at(mouse_world_pos);
    }
}
