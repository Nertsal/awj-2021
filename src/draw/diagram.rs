use geng::Draw2d;

use crate::diagram::{Block, Diagram};

use super::*;

pub fn draw_diagram(
    diagram: &Diagram,
    geng: &Geng,
    framebuffer: &mut ugli::Framebuffer,
    camera: &impl geng::AbstractCamera2d,
) {
    for block in diagram.blocks() {
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
                    .draw_2d(geng, framebuffer, camera);
                }
            }
            Block::Source {
                position,
                signal_color,
                ..
            } => {
                draw_2d::Quad::new(position.map(|x| x as f32), signal_color.color_f32()).draw_2d(
                    geng,
                    framebuffer,
                    camera,
                );
            }
        }
    }
}
