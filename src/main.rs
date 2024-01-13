use app::app::App;

mod app;
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
    Ok(())
}
