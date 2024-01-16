use super::app::App;
use sdl2::{event::Event, keyboard::Keycode};

impl<'a> App<'a> {
    pub fn process_input(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.is_running = false,
                _ => {}
            }
        }
    }
}
