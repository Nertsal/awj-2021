mod draw;
mod handle_event;
mod update;
mod fixed_update;

use geng::Camera2d;

use super::*;

pub struct GameState {
    geng: Geng,
    framebuffer_size: Vec2<usize>,
    camera: Camera2d,
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
        }
    }
}

impl geng::State for GameState {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size();
        self.draw_impl(framebuffer);
    }

    fn update(&mut self, delta_time: f64) {
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
