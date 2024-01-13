use sdl2::render::Canvas;

use crate::{timekeeper::timekeeper::TimeKeeper, window::window};

pub struct App {
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
        let width = window::calculate_width();
        let height = window::calculate_height();

        let window = video_subsystem
            .window("raycasting", width, height)
            .position_centered()
            .borderless()
            .build()
            .map_err(|err| format!("failed to build window with: {:?}", err))?;

        let canvas = window
            .into_canvas()
            .build()
            .map_err(|err| format!("Canvas build error: {:?}", err))?;

        let event_pump = sdl_context.event_pump()?;

        Ok(App {
            sdl_context,
            video_subsystem,
            canvas,
            event_pump,
            timekeeper: TimeKeeper::default(),
            is_running: true,
        })
    }
}
