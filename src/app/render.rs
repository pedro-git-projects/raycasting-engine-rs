use sdl2::pixels::Color;

use super::app::App;

impl App {
    pub fn render(&mut self) {
        self.canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
        self.canvas.clear();
        self.canvas.present();
    }
}
