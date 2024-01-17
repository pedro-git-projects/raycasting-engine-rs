use sdl2::{
    render::{Canvas, TextureCreator},
    video::WindowContext,
};

use crate::{
    colorbuffer::colorbuffer::ColorBuffer, game::game::Game, player::player::Player,
    timekeeper::timekeeper::TimeKeeper,
};

pub struct App<'a> {
    pub game: Game,
    pub player: Player,
    pub sdl_context: &'a sdl2::Sdl,
    pub video_subsystem: &'a sdl2::VideoSubsystem,
    pub texture_creator: &'a TextureCreator<WindowContext>,
    pub color_buffer: ColorBuffer<'a>,
    pub canvas: &'a mut Canvas<sdl2::video::Window>,
    pub event_pump: sdl2::EventPump,
    pub timekeeper: TimeKeeper,
    pub is_running: bool,
}

impl<'a> App<'a> {
    pub fn new(
        sdl_context: &'a sdl2::Sdl,
        video_subsystem: &'a sdl2::VideoSubsystem,
        canvas: &'a mut Canvas<sdl2::video::Window>,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<Self, String> {
        let event_pump = sdl_context.event_pump()?;
        let color_buffer = ColorBuffer::new(&texture_creator)?;

        Ok(App {
            game: Game::default(),
            player: Player::default(),
            sdl_context,
            video_subsystem,
            canvas,
            texture_creator,
            color_buffer,
            event_pump,
            timekeeper: TimeKeeper::default(),
            is_running: true,
        })
    }
}
