use sdl2::pixels::Color;

use super::app::App;

impl<'a> App<'a> {
    pub fn render(&mut self) {
        self.color_buffer.clear(Color::RGBA(0, 0, 0, 255));
        self.canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
        self.canvas.clear();
        self.canvas.present();
    }
}
