use raylib::prelude::{Color, Image};

use crate::framebuffer::Framebuffer;

pub fn write_png(framebuffer: &Framebuffer, filename: &str) {
    let mut image = Image::gen_image_color(
        framebuffer.width as i32,
        framebuffer.height as i32,
        Color::BLACK,
    );

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let pixel = framebuffer.buffer[y * framebuffer.width + x];
            let color = Color::new((pixel >> 16) as u8, (pixel >> 8) as u8, pixel as u8, 255);

            image.draw_pixel(x as i32, y as i32, color);
        }
    }

    image.export_image(filename);
}
