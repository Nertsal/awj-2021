use geng::Draw2d;

use super::*;

impl GameState {
    pub fn draw_impl(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        draw_2d::TexturedQuad::new(
            AABB::ZERO.extend_uniform(constants::FACE_SIZE),
            &self.assets.face,
        )
        .draw_2d(&self.geng, framebuffer, &self.camera);
    }
}
