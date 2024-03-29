use sdl2::{
    pixels::Color,
    pixels::PixelFormatEnum,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

use crate::window::window::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct ColorBuffer<'a> {
    pub buffer: Vec<u32>,
    pub texture_creator: &'a TextureCreator<WindowContext>,
    pub texture: Texture<'a>,
}

impl<'a> ColorBuffer<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Result<Self, String> {
        let buffer = vec![0; (WINDOW_WIDTH * WINDOW_HEIGHT) as usize];

        let texture = match texture_creator.create_texture_streaming(
            PixelFormatEnum::ARGB8888,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        ) {
            Ok(texture) => texture,
            Err(_) => return Err(String::from("Failed to create texture")),
        };

        Ok(Self {
            buffer,
            texture_creator,
            texture,
        })
    }

    pub fn clear(&mut self, color: Color) -> Result<(), String> {
        let pixel_format = PixelFormatEnum::ARGB8888
            .try_into()
            .map_err(|e| format!("Error converting to PixelFormatEnum: {}", e))?;

        for pixel in self.buffer.iter_mut() {
            *pixel = color.to_u32(&pixel_format);
        }

        let color_bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(self.buffer.as_ptr() as *const u8, self.buffer.len() * 4)
        };

        self.texture
            .update(None, color_bytes, WINDOW_WIDTH as usize * 4)
            .map_err(|e| format!("Error updating texture: {}", e))?;

        Ok(())
    }
}
