use super::*;

impl GameState {
    pub fn update_impl(&mut self, delta_time: f32) {
        let mouse_world_pos = mouse_world_pos(&self.geng, &self.camera, self.framebuffer_size);

        // Control stick
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
                    self.poke(target);
                }
            }
            StickState::Retreating => {
                // Move back vertically and towards the mouse
                self.stick.position.y = move_towards_scalar(
                    self.stick.position.y,
                    self.assets.config.stick_height(),
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
                    .approx_eq(&self.assets.config.stick_height())
                {
                    self.stick.state = StickState::Moving;
                }
            }
        }

        // Move crumbs
        for crumb in &mut self.face.crumbs {
            match crumb.target {
                CrumbTarget::Local(target) => {
                    crumb.local_position = move_towards(
                        crumb.local_position,
                        target,
                        self.assets.config.crumb_speed * delta_time,
                    );

                    if reached(crumb.local_position, target) {
                        crumb.target = crumb_random_target();
                    }
                }
                CrumbTarget::ToothHorizontal { y, left } => {
                    let target_x = if left { 0.0 } else { 1.0 };
                    let target = vec2(target_x, y);
                    crumb.local_position = move_towards(
                        crumb.local_position,
                        target,
                        self.assets.config.crumb_speed * delta_time,
                    );

                    if reached(crumb.local_position, target) {
                        let row_len = self.face.teeth.get_row_len(crumb.tooth_position);
                        let new_pos = if left {
                            crumb.tooth_position.indexed(
                                crumb
                                    .tooth_position
                                    .get_index()
                                    .checked_sub(1)
                                    .unwrap_or(row_len - 1),
                            )
                        } else {
                            let mut index = crumb.tooth_position.get_index() + 1;
                            if index >= row_len {
                                index = 0;
                            }
                            crumb.tooth_position.indexed(index)
                        };
                        crumb.tooth_position = new_pos;
                        crumb.local_position.x = 1.0 - target_x;

                        crumb.target = crumb_random_target();
                    }
                }
                CrumbTarget::ToothVertical { x } => {
                    let target_y = match crumb.tooth_position {
                        ToothPosition::Top(_) => 0.0,
                        ToothPosition::Bottom(_) => 1.0,
                    };
                    let target = vec2(x, target_y);
                    crumb.local_position = move_towards(
                        crumb.local_position,
                        target,
                        self.assets.config.crumb_speed * delta_time,
                    );

                    if reached(crumb.local_position, target) {
                        let new_pos = crumb.tooth_position.opposite();
                        crumb.tooth_position = new_pos;
                        crumb.local_position.y = 1.0 - target_y;

                        crumb.target = crumb_random_target();
                    }
                }
            }
        }
    }

    fn poke(&mut self, poke_position: Vec2<f32>) {
        self.face.crumbs.retain(|crumb| {
            (poke_position - crumb.world_position(&self.face.teeth, &self.assets.config)).len()
                > self.assets.config.stick_hit_radius
        });
    }
}

fn crumb_random_target() -> CrumbTarget {
    let mut rng = global_rng();
    match rng.gen_range(1..=3) {
        1 => CrumbTarget::Local(vec2(rng.gen_range(0.0..=1.0), rng.gen_range(0.0..=1.0))),
        2 => CrumbTarget::ToothHorizontal {
            y: rng.gen_range(0.0..=1.0),
            left: rng.gen(),
        },
        3 => CrumbTarget::ToothVertical {
            x: rng.gen_range(0.0..=1.0),
        },
        _ => unreachable!(),
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
