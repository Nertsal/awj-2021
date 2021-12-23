use super::*;

pub fn draw_grid(
    width: usize,
    height: usize,
    geng: &Geng,
    framebuffer: &mut ugli::Framebuffer,
    camera: &impl geng::AbstractCamera2d,
) {
    for y in 0..height {
        for x in 0..width {
            let aabb = AABB::point(vec2(x as f32, y as f32))
                .extend_positive(vec2(1.0, 1.0))
                .extend_uniform(-constants::GRID_WIDTH / 2.0);
            outline::aabb_outline(
                aabb,
                constants::GRID_WIDTH,
                constants::GRID_COLOR,
                geng,
                framebuffer,
                camera,
            );
        }
    }
}
