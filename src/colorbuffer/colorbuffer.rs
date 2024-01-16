use std::rc::Rc;

use sdl2::{
    pixels::Color,
    pixels::PixelFormatEnum,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

use crate::window::window::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct ColorBuffer<'a> {
    color: Vec<u32>,
    texture_creator: &'a TextureCreator<WindowContext>,
    texture: Texture<'a>,
}

impl<'a> ColorBuffer<'a> {
    pub fn new(texture_creator: &'a Rc<TextureCreator<WindowContext>>) -> Result<Self, String> {
        let color = vec![0; (WINDOW_WIDTH * WINDOW_HEIGHT) as usize];

        let texture = match texture_creator.create_texture_streaming(
            PixelFormatEnum::ARGB8888,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        ) {
            Ok(texture) => texture,
            Err(_) => return Err(String::from("Failed to create texture")),
        };

        Ok(Self {
            color,
            texture_creator,
            texture,
        })
    }

    fn clear(&mut self, color: Color) {
        for pixel in self.color.iter_mut() {
            *pixel = color.to_u32(&PixelFormatEnum::ARGB8888.try_into().unwrap());
        }

        let color_bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(self.color.as_ptr() as *const u8, self.color.len() * 4)
        };

        self.texture
            .update(None, color_bytes, WINDOW_WIDTH as usize * 4)
            .unwrap();
    }
}
