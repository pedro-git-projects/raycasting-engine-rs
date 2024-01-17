use crate::app::texture_owner::TextureOwner;

mod app;
mod colorbuffer;
mod game;
mod player;
mod ray;
mod timekeeper;
mod window;
extern crate sdl2;

pub fn main() -> Result<(), String> {
    let mut owner = TextureOwner::new()?;
    let mut app = owner.build_app()?;

    while app.is_running {
        app.process_input();
        app.update();
        app.render();
    }
    Ok(())
}
