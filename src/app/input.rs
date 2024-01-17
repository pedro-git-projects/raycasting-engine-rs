use super::app::App;
use sdl2::{event::Event, keyboard::Keycode};

impl<'a> App<'a> {
    pub fn process_input(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.is_running = false,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Escape => self.is_running = false,
                    Keycode::Up => self.player.set_walk_direction("forward").unwrap(),
                    Keycode::Down => self.player.set_walk_direction("backward").unwrap(),
                    Keycode::Right => self.player.set_turn_direction("right").unwrap(),
                    Keycode::Left => self.player.set_turn_direction("left").unwrap(),
                    _ => {}
                },
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Up | Keycode::Down => {
                        self.player.set_walk_direction("neutral").unwrap()
                    }
                    Keycode::Right | Keycode::Left => {
                        self.player.set_turn_direction("neutral").unwrap()
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
