use super::*;

impl GameState {
    pub fn update_impl(&mut self, delta_time: f32) {
        let mouse_world_pos = mouse_world_pos(&self.geng, &self.camera, self.framebuffer_size);

        match self.stick.state {
            StickState::Moving => {
                // Move stick under the mouse
                self.stick.position.x = move_towards_scalar(
                    self.stick.position.x,
                    mouse_world_pos.x,
                    self.assets.config.stick_move_speed * delta_time,
                );
            }
            StickState::Poking { target } => {
                // Move towards target
                self.stick.position = move_towards(
                    self.stick.position,
                    target,
                    self.assets.config.stick_move_speed * delta_time,
                );

                if reached(self.stick.position, target) {
                    self.stick.state = StickState::Retreating;
                }
            }
            StickState::Retreating => {
                // Move back vertically and towards the mouse
                self.stick.position.y = move_towards_scalar(
                    self.stick.position.y,
                    self.assets.config.stick_height,
                    self.assets.config.stick_move_speed * delta_time,
                );
                self.stick.position.x = move_towards_scalar(
                    self.stick.position.x,
                    mouse_world_pos.x,
                    self.assets.config.stick_move_speed * delta_time,
                );

                if self
                    .stick
                    .position
                    .y
                    .approx_eq(&self.assets.config.stick_height)
                {
                    self.stick.state = StickState::Moving;
                }
            }
        }
    }
}

fn reached(position: Vec2<f32>, target: Vec2<f32>) -> bool {
    (position - target).len().approx_eq(&0.0)
}

fn move_towards(position: Vec2<f32>, target: Vec2<f32>, speed: f32) -> Vec2<f32> {
    let delta = target - position;
    let move_delta = delta.clamp_len(..=speed);
    position + move_delta
}

fn move_towards_scalar(current: f32, target: f32, speed: f32) -> f32 {
    let delta = target - current;
    let move_delta = delta.clamp(-speed, speed);
    current + move_delta
}
