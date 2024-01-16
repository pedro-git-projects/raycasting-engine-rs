use std::rc::Rc;

use sdl2::{
    render::{Canvas, TextureCreator},
    video::WindowContext,
};

use crate::{
    colorbuffer::colorbuffer::ColorBuffer,
    game::game::Game,
    player::player::Player,
    timekeeper::timekeeper::TimeKeeper,
    window::window::{WINDOW_HEIGHT, WINDOW_WIDTH},
};

pub struct App<'a> {
    pub game: Game,
    pub player: Player,
    pub sdl_context: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub texture_creator: Rc<TextureCreator<WindowContext>>,
    pub color_buffer: ColorBuffer<'a>,
    pub canvas: Canvas<sdl2::video::Window>,
    pub event_pump: sdl2::EventPump,
    pub timekeeper: TimeKeeper,
    pub is_running: bool,
}

impl<'a> App<'a> {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("raycasting", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .borderless()
            .build()
            .map_err(|err| format!("failed to build window with: {:?}", err))?;

        let canvas = window
            .into_canvas()
            .build()
            .map_err(|err| format!("Canvas build error: {:?}", err))?;

        let event_pump = sdl_context.event_pump()?;

        let texture_creator = Rc::new(canvas.texture_creator());
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
