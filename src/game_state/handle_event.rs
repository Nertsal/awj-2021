use geng::MouseButton;

use super::*;

pub enum Dragging {
    MoveCamera {
        initial_mouse_pos: Vec2<f32>,
        initial_camera_pos: Vec2<f32>,
    },
}

impl GameState {
    pub fn handle_event_impl(&mut self, event: geng::Event) {
        match event {
            geng::Event::MouseDown { position, button } => {
                self.drag_start(position, button);
            }
            geng::Event::MouseUp { .. } => {
                self.drag_stop();
            }
            geng::Event::MouseMove { position, .. } => {
                self.drag_update_pos(position);
            }
            geng::Event::Wheel { delta } => {
                if self.geng.window().is_key_pressed(geng::Key::LCtrl) {
                    // Zoom
                    self.camera.fov = (self.camera.fov + delta as f32 * constants::ZOOM_SPEED)
                        .clamp(constants::ZOOM_MIN, constants::ZOOM_MAX);
                }
            }
            _ => (),
        }
    }

    fn drag_start(&mut self, mouse_pos: Vec2<f64>, mouse_button: MouseButton) {
        match mouse_button {
            button
                if button == MouseButton::Left
                    && self.geng.window().is_key_pressed(geng::Key::LCtrl)
                    || button == MouseButton::Right =>
            {
                self.dragging = Some(Dragging::MoveCamera {
                    initial_mouse_pos: mouse_pos.map(|x| x as f32),
                    initial_camera_pos: self.camera.center,
                })
            }
            _ => (),
        }
    }

    fn drag_stop(&mut self) {
        self.dragging = None;
    }

    fn drag_update_pos(&mut self, mouse_pos: Vec2<f64>) {
        self.mouse_position = mouse_pos.map(|x| x as f32);
    }

    pub fn drag_update(&mut self) {
        match &mut self.dragging {
            Some(dragging) => match dragging {
                Dragging::MoveCamera {
                    initial_mouse_pos,
                    initial_camera_pos,
                } => {
                    let initial_world_pos = self
                        .camera
                        .screen_to_world(self.framebuffer_size, *initial_mouse_pos);
                    let current_world_pos = self
                        .camera
                        .screen_to_world(self.framebuffer_size, self.mouse_position);
                    let delta = initial_world_pos - current_world_pos;
                    self.camera.center = *initial_camera_pos + delta;
                }
            },
            None => (),
        }
    }
}
