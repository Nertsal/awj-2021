use geng::Draw2d;

use crate::diagram::{BlockType, Diagram};

use super::*;

pub fn draw_diagram(
    diagram: &Diagram,
    geng: &Geng,
    framebuffer: &mut ugli::Framebuffer,
    camera: &impl geng::AbstractCamera2d,
) {
    for block in diagram.blocks() {
        match &block.block_type {
            BlockType::Wire {
                connections,
                queued_signal,
            } => {
                let center = block.position.map(|x| x as f32 + 0.5);
                for delta in connections
                    .deltas()
                    .map(|(delta, _)| delta.map(|x| x as f32 / 2.0))
                {
                    draw_2d::Segment::new(
                        Segment::new(center, center + delta),
                        constants::WIRE_WIDTH,
                        queued_signal
                            .map(|(color, _)| color.color_f32())
                            .unwrap_or(constants::WIRE_COLOR),
                    )
                    .draw_2d(geng, framebuffer, camera);
                }
            }
            BlockType::Source { signal_color, .. } => {
                draw_2d::Quad::new(
                    AABB::point(block.position.map(|x| x as f32)).extend_positive(vec2(1.0, 1.0)),
                    signal_color.color_f32(),
                )
                .draw_2d(geng, framebuffer, camera);
            }
        }
    }
}
