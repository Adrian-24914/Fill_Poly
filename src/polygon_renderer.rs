use crate::framebuffer::Framebuffer;
use crate::polygon::Polygon;

pub struct PolygonRenderer;

impl PolygonRenderer {
    pub fn render(framebuffer: &mut Framebuffer, polygon: &Polygon, color: u32) {
        framebuffer.set_current_color(color);

        Self::draw_contour(framebuffer, &polygon.vertices);

        for hole in &polygon.holes {
            Self::draw_contour(framebuffer, hole);
        }
    }

    fn draw_contour(framebuffer: &mut Framebuffer, vertices: &[(i32, i32)]) {
        if vertices.len() < 2 {
            return;
        }

        for index in 0..vertices.len() {
            let current = vertices[index];
            let next = vertices[(index + 1) % vertices.len()];

            framebuffer.draw_line(current.0, current.1, next.0, next.1);
        }
    }
}
