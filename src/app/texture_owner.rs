use sdl2::{
    render::{Canvas, TextureCreator},
    video::WindowContext,
    VideoSubsystem,
};

use crate::window::window::{WINDOW_HEIGHT, WINDOW_WIDTH};

use super::app::App;

pub struct TextureOwner {
    pub sdl_context: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub texture_creator: sdl2::render::TextureCreator<WindowContext>,
}

impl TextureOwner {
    pub fn new() -> Result<Self, String> {
        let sdl_context = Self::build_context()?;
        let video_subsystem = Self::build_video_subsystem(&sdl_context)?;
        let canvas = Self::build_canvas(&video_subsystem)?;
        let texture_creator = Self::build_texture_creator(&canvas);
        Ok(Self {
            sdl_context,
            video_subsystem,
            canvas,
            texture_creator,
        })
    }

    pub fn build_app<'b>(&'b mut self) -> Result<App<'b>, String> {
        let app = App::new(
            &self.sdl_context,
            &self.video_subsystem,
            &mut self.canvas,
            &self.texture_creator,
        )?;
        Ok(app)
    }

    fn build_context() -> Result<sdl2::Sdl, String> {
        sdl2::init()
    }

    fn build_video_subsystem(sdl_context: &sdl2::Sdl) -> Result<VideoSubsystem, String> {
        sdl_context.video()
    }

    fn build_canvas(
        video_subsystem: &VideoSubsystem,
    ) -> Result<Canvas<sdl2::video::Window>, String> {
        let window = video_subsystem
            .window("raycasting", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .borderless()
            .build()
            .map_err(|err| format!("failed to build window with: {:?}", err))?;

        window
            .into_canvas()
            .build()
            .map_err(|err| format!("Canvas build error: {:?}", err))
    }

    fn build_texture_creator(
        canvas: &Canvas<sdl2::video::Window>,
    ) -> TextureCreator<WindowContext> {
        canvas.texture_creator()
    }
}
