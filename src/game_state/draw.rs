use geng::Draw2d;

use super::*;

impl GameState {
    pub fn draw_impl(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        // Face
        draw_2d::TexturedQuad::new(
            AABB::ZERO.extend_uniform(constants::FACE_SIZE),
            &self.assets.face,
        )
        .draw_2d(&self.geng, framebuffer, &self.camera);

        // Stick
        draw_2d::TexturedQuad::new(
            AABB::point(self.stick.position).extend_symmetric(constants::STICK_SIZE / 2.0),
            &self.assets.stick,
        )
        .draw_2d(&self.geng, framebuffer, &self.camera);
    }
}
