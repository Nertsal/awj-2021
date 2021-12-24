use geng::Draw2d;

use super::*;

impl GameState {
    pub fn draw_impl(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        // Face
        let face = AABB::ZERO.extend_uniform(self.assets.config.face_radius);
        draw_2d::TexturedQuad::new(face, &self.assets.face).draw_2d(
            &self.geng,
            framebuffer,
            &self.camera,
        );

        // Teeth
        for tooth in self.face.teeth.iter() {
            draw_2d::TexturedQuad::new(tooth.poke_box(&self.assets.config), tooth.texture.clone())
                .draw_2d(&self.geng, framebuffer, &self.camera);
        }

        // Crumbs
        for crumb in &self.face.crumbs {
            draw_2d::TexturedQuad::new(
                AABB::point(crumb.world_position(&self.face.teeth, &self.assets.config))
                    .extend_symmetric(self.assets.config.crumb_size),
                &self.assets.crumb,
            )
            .draw_2d(&self.geng, framebuffer, &self.camera);
        }

        // Stick
        draw_2d::TexturedQuad::new(
            AABB::point(self.stick.position - vec2(0.0, self.assets.config.stick_size.y / 2.0))
                .extend_symmetric(self.assets.config.stick_size / 2.0),
            &self.assets.stick,
        )
        .draw_2d(&self.geng, framebuffer, &self.camera);
    }
}
