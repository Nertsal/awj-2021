use geng::Draw2d;

use super::*;

impl GameState {
    pub fn draw_impl(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        // Draw grid
        for y in 0..self.diagram.map_height() {
            for x in 0..self.diagram.map_width() {
                let aabb = AABB::point(vec2(x as f32, y as f32))
                    .extend_positive(vec2(1.0, 1.0))
                    .extend_uniform(-constants::GRID_WIDTH / 2.0);
                draw_2d::Chain::new(
                    Chain::new(vec![
                        aabb.bottom_left(),
                        aabb.top_left(),
                        aabb.top_right(),
                        aabb.bottom_right(),
                        aabb.bottom_left(),
                    ]),
                    constants::GRID_WIDTH,
                    constants::GRID_COLOR,
                    0,
                )
                .draw_2d(&self.geng, framebuffer, &self.camera);
            }
        }

        // Draw blocks
        for block in self.diagram.blocks() {
            match block {
                Block::Wire {
                    position,
                    connections,
                    queued_signal,
                } => {
                    let center = position.map(|x| x as f32);
                    for delta in connections
                        .deltas()
                        .map(|(delta, _)| delta.map(|x| x as f32))
                    {
                        draw_2d::Segment::new(
                            Segment::new(center, center + delta),
                            constants::WIRE_WIDTH,
                            queued_signal
                                .map(|(color, _)| color.color_f32())
                                .unwrap_or(constants::WIRE_COLOR),
                        )
                        .draw_2d(&self.geng, framebuffer, &self.camera);
                    }
                }
                Block::Source {
                    position,
                    signal_color,
                    ..
                } => {
                    draw_2d::Quad::new(position.map(|x| x as f32), signal_color.color_f32())
                        .draw_2d(&self.geng, framebuffer, &self.camera);
                }
            }
        }
    }
}
