use geng::{Camera2d, MouseButton};

use crate::diagram::{BlockType, Diagram, Directions, SignalColor};

use super::*;

pub struct EditorState {
    geng: Geng,
    framebuffer_size: Vec2<f32>,
    mouse_position: Vec2<f32>,
    camera: Camera2d,
    diagram: Diagram,
    dragging: Option<Dragging>,
    selected_block: Option<BlockType>,
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
                fov: 30.0,
            },
            diagram: diagram_file
                .map(|file| Diagram::load_from_file(file).unwrap())
                .unwrap_or(Diagram::new(vec2(10, 10))),
            dragging: None,
            selected_block: None,
        }
    }

    fn drag_update(&mut self) {
        self.mouse_position = self.geng.window().mouse_pos().map(|x| x as f32);

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
                Dragging::Selection { .. } => (),
            },
            None => (),
        }
    }
}

pub enum Dragging {
    Selection {
        initial_mouse_pos: Vec2<f32>,
    },
    MoveCamera {
        initial_mouse_pos: Vec2<f32>,
        initial_camera_pos: Vec2<f32>,
    },
}

impl geng::State for EditorState {
    fn update(&mut self, delta_time: f64) {
        let delta_time = delta_time as f32;
        self.drag_update();
    }

    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size().map(|x| x as f32);
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        // Draw grid
        crate::draw::grid::draw_grid(
            self.diagram.map_width(),
            self.diagram.map_height(),
            &self.geng,
            framebuffer,
            &self.camera,
        );

        // Draw blocks
        crate::draw::diagram::draw_diagram(&self.diagram, &self.geng, framebuffer, &self.camera);
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
                MouseButton::Left => {
                    self.dragging = Some(Dragging::Selection {
                        initial_mouse_pos: position.map(|x| x as f32),
                    });
                }
                _ => (),
            },
            geng::Event::MouseUp { .. } => match self.dragging.take() {
                Some(dragging) => match dragging {
                    Dragging::Selection { initial_mouse_pos } => {
                        let delta = initial_mouse_pos - self.mouse_position;
                        if delta.len().approx_eq(&0.0) {
                            // Click
                            let world_pos = self
                                .camera
                                .screen_to_world(self.framebuffer_size, self.mouse_position);
                            if world_pos.x >= 0.0 && world_pos.y >= 0.0 {
                                let cell_pos = world_pos.map(|x| x.floor() as usize);
                                match self.selected_block.clone() {
                                    Some(block) => {
                                        self.diagram.insert_block_at(cell_pos, block);
                                    }
                                    None => {
                                        self.diagram.clear_at(cell_pos);
                                    }
                                }
                            }
                        }
                    }
                    Dragging::MoveCamera { .. } => (),
                },
                None => (),
            },
            geng::Event::MouseMove { .. } => {
                self.drag_update();
            }
            geng::Event::Wheel { delta } => {
                if self.geng.window().is_key_pressed(geng::Key::LCtrl) {
                    // Zoom
                    self.camera.fov = (self.camera.fov + delta as f32 * constants::ZOOM_SPEED)
                        .clamp(constants::ZOOM_MIN, constants::ZOOM_MAX);
                }
            }
            geng::Event::KeyDown { key } => match key {
                geng::Key::Num1 => {
                    self.selected_block = None;
                }
                geng::Key::Num2 => {
                    self.selected_block = Some(BlockType::Source {
                        signal_color: SignalColor::Green,
                        emit_directions: Directions::all(),
                    });
                }
                geng::Key::Num3 => {
                    self.selected_block = Some(BlockType::Wire {
                        connections: Directions::all(),
                        queued_signal: None,
                    });
                }
                _ => (),
            },
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
