use geng::{Camera2d, MouseButton};

use crate::diagram::Diagram;

use super::*;

pub struct EditorState {
    geng: Geng,
    framebuffer_size: Vec2<f32>,
    mouse_position: Vec2<f32>,
    camera: Camera2d,
    diagram: Diagram,
    dragging: Option<Dragging>,
}

impl EditorState {
    pub fn new(geng: &Geng, diagram_file: Option<&str>) -> Self {
        Self {
            geng: geng.clone(),
            framebuffer_size: vec2(1.0, 1.0),
            mouse_position: vec2(0.0, 0.0),
            camera: Camera2d {
                center: Vec2::ZERO,
                rotation: 0.0,
                fov: 100.0,
            },
            diagram: diagram_file
                .map(|file| Diagram::load_from_file(file).unwrap())
                .unwrap_or(Diagram::new(vec2(10, 10))),
            dragging: None,
        }
    }
}

pub enum Dragging {
    MoveCamera {
        initial_mouse_pos: Vec2<f32>,
        initial_camera_pos: Vec2<f32>,
    },
}

impl geng::State for EditorState {
    fn update(&mut self, delta_time: f64) {
        let delta_time = delta_time as f32;
    }

    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);
    }

    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::MouseDown { position, button } => match button {
                button
                    if button == MouseButton::Left
                        && self.geng.window().is_key_pressed(geng::Key::LCtrl)
                        || button == MouseButton::Right =>
                {
                    self.dragging = Some(Dragging::MoveCamera {
                        initial_mouse_pos: position.map(|x| x as f32),
                        initial_camera_pos: self.camera.center,
                    })
                }
                _ => (),
            },
            geng::Event::MouseUp { .. } => {
                self.dragging = None;
            }
            geng::Event::MouseMove { position, .. } => {
                self.mouse_position = position.map(|x| x as f32);

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

    fn transition(&mut self) -> Option<geng::Transition> {
        if self.geng.window().is_key_pressed(geng::Key::R) {
            self.diagram.save_to_file(constants::DIAGRAM_FILE).unwrap();
            Some(geng::Transition::Switch(Box::new(
                game_state::GameState::new(&self.geng, Some(constants::DIAGRAM_FILE)),
            )))
        } else {
            None
        }
    }
}
