use crate::framebuffer::Framebuffer;
use crate::polygon::Polygon;

pub struct Scanline;

impl Scanline {
    pub fn fill(framebuffer: &mut Framebuffer, polygons: &[&Polygon], color: u32) {
        if framebuffer.width == 0 {
            return;
        }

        framebuffer.set_current_color(color);

        for y in 0..framebuffer.height as i32 {
            let mut intersections = Vec::new();

            for polygon in polygons {
                Self::find_intersections(&polygon.vertices, y, &mut intersections);
            }

            intersections.sort_by(f64::total_cmp);

            for pair in intersections.chunks_exact(2) {
                let start_x = (pair[0].ceil() as i32).max(0);
                let end_x = (pair[1].floor() as i32).min(framebuffer.width as i32 - 1);

                if start_x <= end_x {
                    framebuffer.draw_line(start_x, y, end_x, y);
                }
            }
        }
    }

    fn find_intersections(vertices: &[(i32, i32)], y: i32, result: &mut Vec<f64>) {
        if vertices.len() < 3 {
            return;
        }

        for index in 0..vertices.len() {
            let (x0, y0) = vertices[index];
            let (x1, y1) = vertices[(index + 1) % vertices.len()];

            let crosses = (y0 <= y && y < y1) || (y1 <= y && y < y0);

            if crosses {
                let x = x0 as f64 + (y - y0) as f64 * (x1 - x0) as f64 / (y1 - y0) as f64;

                result.push(x);
            }
        }
    }
}
