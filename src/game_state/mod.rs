mod draw;
mod fixed_update;
mod handle_event;
mod tick;
mod update;

use geng::Camera2d;

use crate::diagram::*;

use self::handle_event::Dragging;

use super::*;

pub struct GameState {
    geng: Geng,
    framebuffer_size: Vec2<f32>,
    mouse_position: Vec2<f32>,
    camera: Camera2d,
    tick_updater: FixedUpdater,
    diagram: Diagram,
    dragging: Option<Dragging>,
}

impl GameState {
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
            tick_updater: FixedUpdater::new(1.0, 0.0),
            diagram: diagram_file
                .map(|file| Diagram::load_from_file(file).unwrap())
                .unwrap_or(Diagram::new(vec2(10, 10))),
            dragging: None,
        }
    }
}

impl geng::State for GameState {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size().map(|x| x as f32);
        self.draw_impl(framebuffer);
    }

    fn update(&mut self, delta_time: f64) {
        for _ in 0..self.tick_updater.update(delta_time) {
            self.tick();
        }

        self.drag_update();

        let delta_time = delta_time as f32;
        self.update_impl(delta_time);
    }

    fn fixed_update(&mut self, delta_time: f64) {
        let delta_time = delta_time as f32;
        self.fixed_update_impl(delta_time);
    }

    fn handle_event(&mut self, event: geng::Event) {
        self.handle_event_impl(event);
    }

    fn transition(&mut self) -> Option<geng::Transition> {
        if self.geng.window().is_key_pressed(geng::Key::E) {
            Some(geng::Transition::Switch(Box::new(
                editor_state::EditorState::new(&self.geng, Some(constants::DIAGRAM_FILE)),
            )))
        } else {
            None
        }
    }
}
