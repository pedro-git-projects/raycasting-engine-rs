use sdl2::render::Canvas;

use crate::{
    game::game::Game,
    timekeeper::timekeeper::TimeKeeper,
    window::window::{WINDOW_HEIGHT, WINDOW_WIDTH},
};

pub struct App {
    pub game: Game,
    pub sdl_context: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub canvas: Canvas<sdl2::video::Window>,
    pub event_pump: sdl2::EventPump,
    pub timekeeper: TimeKeeper,
    pub is_running: bool,
}

impl App {
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
        let game = Game::default();

        Ok(App {
            game,
            sdl_context,
            video_subsystem,
            canvas,
            event_pump,
            timekeeper: TimeKeeper::default(),
            is_running: true,
        })
    }
}
