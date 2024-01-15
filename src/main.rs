use app::app::App;

mod app;
mod game;
mod player;
mod ray;
mod timekeeper;
mod window;
extern crate sdl2;

pub fn main() -> Result<(), String> {
    let mut app = App::new()?;
    while app.is_running {
        app.process_input();
        app.update();
        app.render();
    }
    assert!(1 == 2);
    Ok(())
}
