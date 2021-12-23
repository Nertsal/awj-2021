mod draw;
mod fixed_update;
mod handle_event;
mod tick;
mod update;

use geng::Camera2d;

use crate::diagram::*;

use super::*;

pub struct GameState {
    geng: Geng,
    framebuffer_size: Vec2<usize>,
    camera: Camera2d,
    tick_updater: FixedUpdater,
    diagram: Diagram,
}

impl GameState {
    pub fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            framebuffer_size: vec2(1, 1),
            camera: Camera2d {
                center: Vec2::ZERO,
                rotation: 0.0,
                fov: 100.0,
            },
            tick_updater: FixedUpdater::new(1.0, 0.0),
            diagram: Diagram::new(vec2(10, 10)),
        }
    }
}

impl geng::State for GameState {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size();
        self.draw_impl(framebuffer);
    }

    fn update(&mut self, delta_time: f64) {
        for _ in 0..self.tick_updater.update(delta_time) {
            self.tick();
        }

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
}
