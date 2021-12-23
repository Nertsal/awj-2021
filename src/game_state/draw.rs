use super::*;

impl GameState {
    pub fn draw_impl(&mut self, framebuffer: &mut ugli::Framebuffer) {
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
}
