mod bmp;
mod framebuffer;
mod line;
mod polygon;
mod polygon_renderer;
mod scanline;

use crate::bmp::write_bmp;
use crate::framebuffer::Framebuffer;
use crate::polygon::Polygon;
use crate::polygon_renderer::PolygonRenderer;
use crate::scanline::Scanline;

fn main() -> std::io::Result<()> {
    let mut framebuffer = Framebuffer::new(800, 500);

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    let polygon1 = Polygon::new(vec![
        (165, 380),
        (185, 360),
        (180, 330),
        (207, 345),
        (233, 330),
        (230, 360),
        (250, 380),
        (220, 385),
        (205, 410),
        (193, 383),
    ]);

    let polygon2 = Polygon::new(vec![(321, 335), (288, 286), (339, 251), (374, 302)]);

    let polygon3 = Polygon::new(vec![(377, 249), (411, 197), (436, 249)]);

    let polygon4 = Polygon::new(vec![
        (413, 177),
        (448, 159),
        (502, 88),
        (553, 53),
        (535, 36),
        (676, 37),
        (660, 52),
        (750, 145),
        (761, 179),
        (672, 192),
        (659, 214),
        (615, 214),
        (632, 230),
        (580, 230),
        (597, 215),
        (552, 214),
        (517, 144),
        (466, 180),
    ]);

    let polygon5 = Polygon::new(vec![(682, 175), (708, 120), (735, 148), (739, 170)]);

    Scanline::fill(&mut framebuffer, &[&polygon1], 0xFF0000);
    Scanline::fill(&mut framebuffer, &[&polygon2], 0x00FF00);
    Scanline::fill(&mut framebuffer, &[&polygon3], 0x0000FF);
    Scanline::fill(&mut framebuffer, &[&polygon4, &polygon5], 0xFFFF00);

    let border_color = 0xFFFFFF;

    PolygonRenderer::render(&mut framebuffer, &polygon1, border_color);
    PolygonRenderer::render(&mut framebuffer, &polygon2, border_color);
    PolygonRenderer::render(&mut framebuffer, &polygon3, border_color);
    PolygonRenderer::render(&mut framebuffer, &polygon4, border_color);
    PolygonRenderer::render(&mut framebuffer, &polygon5, border_color);

    write_bmp(&framebuffer, "out.bmp")?;
    println!("Se creó out.bmp");

    Ok(())
}
