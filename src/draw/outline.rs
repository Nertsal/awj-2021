use geng::Draw2d;

use super::*;

pub fn aabb_outline(
    aabb: AABB<f32>,
    width: f32,
    color: Color<f32>,
    geng: &Geng,
    framebuffer: &mut ugli::Framebuffer,
    camera: &impl geng::AbstractCamera2d,
) {
    draw_2d::Segment::new(
        Segment::new(
            aabb.top_left() - vec2(width / 2.0, 0.0),
            aabb.top_right() + vec2(width / 2.0, 0.0),
        ),
        width,
        color,
    )
    .draw_2d(geng, framebuffer, camera);
    draw_2d::Segment::new(
        Segment::new(
            aabb.bottom_left() - vec2(width / 2.0, 0.0),
            aabb.bottom_right() + vec2(width / 2.0, 0.0),
        ),
        width,
        color,
    )
    .draw_2d(geng, framebuffer, camera);
    draw_2d::Segment::new(
        Segment::new(
            aabb.bottom_left() + vec2(0.0, width / 2.0),
            aabb.top_left() - vec2(0.0, width / 2.0),
        ),
        width,
        color,
    )
    .draw_2d(geng, framebuffer, camera);
    draw_2d::Segment::new(
        Segment::new(
            aabb.bottom_right() + vec2(0.0, width / 2.0),
            aabb.top_right() - vec2(0.0, width / 2.0),
        ),
        width,
        color,
    )
    .draw_2d(geng, framebuffer, camera);
}
