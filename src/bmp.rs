use std::fs::File;
use std::io::{BufWriter, Write};

use crate::framebuffer::Framebuffer;

pub fn write_bmp(framebuffer: &Framebuffer, filename: &str) -> std::io::Result<()> {
    let width = framebuffer.width;
    let height = framebuffer.height;
    let row_size = (width * 3 + 3) & !3;
    let pixel_size = row_size * height;
    let file_size = pixel_size + 54;

    let mut file = BufWriter::new(File::create(filename)?);

    file.write_all(b"BM")?;
    file.write_all(&(file_size as u32).to_le_bytes())?;
    file.write_all(&0u32.to_le_bytes())?;
    file.write_all(&54u32.to_le_bytes())?;

    file.write_all(&40u32.to_le_bytes())?;
    file.write_all(&(width as u32).to_le_bytes())?;
    file.write_all(&(height as u32).to_le_bytes())?;
    file.write_all(&1u16.to_le_bytes())?;
    file.write_all(&24u16.to_le_bytes())?;
    file.write_all(&0u32.to_le_bytes())?;
    file.write_all(&(pixel_size as u32).to_le_bytes())?;
    file.write_all(&0u32.to_le_bytes())?;
    file.write_all(&0u32.to_le_bytes())?;
    file.write_all(&0u32.to_le_bytes())?;
    file.write_all(&0u32.to_le_bytes())?;

    let padding_size = (4 - (width * 3) % 4) % 4;
    let padding = [0u8; 3];

    for y in (0..height).rev() {
        for x in 0..width {
            let pixel = framebuffer.buffer[y * width + x];
            let bgr = [pixel as u8, (pixel >> 8) as u8, (pixel >> 16) as u8];
            file.write_all(&bgr)?;
        }
        file.write_all(&padding[..padding_size])?;
    }

    Ok(())
}
