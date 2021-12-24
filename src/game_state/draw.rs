use geng::Draw2d;

use super::*;

impl GameState {
    pub fn draw_impl(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        // Face
        let face = AABB::ZERO.extend_uniform(constants::FACE_SIZE);
        draw_2d::TexturedQuad::new(face, &self.assets.face).draw_2d(
            &self.geng,
            framebuffer,
            &self.camera,
        );

        // Teeth
        for tooth in &self.face.teeth {
            draw_2d::TexturedQuad::new(
                AABB::point(tooth.position * face.size() + face.bottom_left())
                    .extend_symmetric(constants::TOOTH_SIZE / 2.0),
                tooth.texture.clone(),
            )
            .draw_2d(&self.geng, framebuffer, &self.camera);
        }

        // Stick
        draw_2d::TexturedQuad::new(
            AABB::point(self.stick.position - vec2(0.0, constants::STICK_SIZE.y / 2.0))
                .extend_symmetric(constants::STICK_SIZE / 2.0),
            &self.assets.stick,
        )
        .draw_2d(&self.geng, framebuffer, &self.camera);
    }
}
