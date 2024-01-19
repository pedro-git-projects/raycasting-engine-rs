use sdl2::pixels::Color;

use super::app::App;

impl<'a> App<'a> {
    pub fn render(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        self.canvas.clear();

        self.generate3d_projection()?;

        self.render_color_buffer()?;
        self.color_buffer.clear(Color::RGBA(0, 0, 0, 255))?;

        self.render_map()?;
        self.render_rays()?;
        self.render_player()?;
        self.canvas.present();

        Ok(())
    }
}
