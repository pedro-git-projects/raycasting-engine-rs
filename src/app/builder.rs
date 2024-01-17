// use sdl2::{
//     render::{Canvas, TextureCreator},
//     video::WindowContext,
//     VideoSubsystem,
// };
//
// use crate::{
//     colorbuffer::colorbuffer::ColorBuffer,
//     game::game::Game,
//     player::player::Player,
//     timekeeper::timekeeper::TimeKeeper,
//     window::window::{WINDOW_HEIGHT, WINDOW_WIDTH},
// };
//
// use super::app::App;
//
// pub struct AppBuilder<'a> {
//     pub sdl_context: Option<sdl2::Sdl>,
//     pub video_subsystem: Option<sdl2::VideoSubsystem>,
//     pub window: Option<sdl2::video::Window>,
//     pub canvas: Option<sdl2::render::Canvas<sdl2::video::Window>>,
//     texture_creator: Option<sdl2::render::TextureCreator<WindowContext>>,
//     color_buffer: Option<ColorBuffer<'a>>,
//     event_pump: Option<sdl2::EventPump>,
//     game: Option<Game>,
//     player: Option<Player>,
//     timekeeper: Option<TimeKeeper>,
//     is_running: Option<bool>,
// }
//
// impl Default for AppBuilder<'_> {
//     fn default() -> Self {
//         Self {
//             sdl_context: None,
//             video_subsystem: None,
//             window: None,
//             canvas: None,
//             texture_creator: None,
//             color_buffer: None,
//             event_pump: None,
//             game: None,
//             player: None,
//             timekeeper: None,
//             is_running: None,
//         }
//     }
// }
//
// impl<'a> AppBuilder<'a> {
//     pub fn build(&mut self) -> Result<App, String> {
//         if self.sdl_context.is_none()
//             || self.video_subsystem.is_none()
//             || self.canvas.is_none()
//             || self.texture_creator.is_none()
//             || self.color_buffer.is_none()
//             || self.event_pump.is_none()
//             || self.game.is_none()
//             || self.player.is_none()
//             || self.timekeeper.is_none()
//             || self.is_running.is_none()
//         {
//             return Err("Missing required fields".to_string());
//         }
//
//         if self.window.is_none() {
//             let window = self
//                 .video_subsystem
//                 .as_ref()
//                 .unwrap()
//                 .window("raycasting", WINDOW_WIDTH, WINDOW_HEIGHT)
//                 .position_centered()
//                 .borderless()
//                 .build()
//                 .map_err(|err| format!("failed to build window with: {:?}", err))?;
//
//             self.window = Some(window);
//         }
//
//         let app = App {
//             sdl_context: self.sdl_context.take().unwrap(),
//             video_subsystem: self.video_subsystem.take().unwrap(),
//             canvas: self.canvas.take().unwrap(),
//             texture_creator: &self.texture_creator.take().unwrap(),
//             color_buffer: self.color_buffer.take().unwrap(),
//             event_pump: self.event_pump.take().unwrap(),
//             game: self.game.take().unwrap(),
//             player: self.player.take().unwrap(),
//             timekeeper: self.timekeeper.take().unwrap(),
//             is_running: self.is_running.take().unwrap(),
//         };
//
//         Ok(app)
//     }
//
//     pub fn with_sdl_context(&mut self, sdl_context: sdl2::Sdl) -> &mut Self {
//         self.sdl_context = Some(sdl_context);
//         self
//     }
//
//     pub fn with_game(&mut self, game: Game) -> &mut Self {
//         self.game = Some(game);
//         self
//     }
//
//     pub fn with_player(&mut self, player: Player) -> &mut Self {
//         self.player = Some(player);
//         self
//     }
//
//     pub fn with_context(&mut self, sdl_context: Result<sdl2::Sdl, String>) -> &mut Self {
//         self.sdl_context = match sdl_context {
//             Ok(context) => Some(context),
//             Err(err) => {
//                 eprintln!("Error initializing SDL context: {}", err);
//                 None
//             }
//         };
//         self
//     }
//
//     pub fn with_video_subsystem(
//         &mut self,
//         video_subsystem: Result<VideoSubsystem, String>,
//     ) -> &mut Self {
//         self.video_subsystem = match video_subsystem {
//             Ok(subsystem) => Some(subsystem),
//             Err(err) => {
//                 eprintln!("Error initializing video subsystem: {}", err);
//                 None
//             }
//         };
//         self
//     }
//
//     pub fn with_canvas(
//         &mut self,
//         canvas: Result<Canvas<sdl2::video::Window>, String>,
//     ) -> &mut Self {
//         self.canvas = match canvas {
//             Ok(canvas) => Some(canvas),
//             Err(err) => {
//                 eprintln!("Error building canvas: {}", err);
//                 None
//             }
//         };
//         self
//     }
//
//     pub fn with_texture_creator(
//         &mut self,
//         texture_creator: TextureCreator<WindowContext>,
//     ) -> &mut Self {
//         self.texture_creator = Some(texture_creator);
//         self
//     }
//
//     pub fn with_color_buffer(
//         &mut self,
//         color_buffer: Result<ColorBuffer<'a>, String>,
//     ) -> &mut Self {
//         self.color_buffer = match color_buffer {
//             Ok(buffer) => Some(buffer),
//             Err(err) => {
//                 // Handle error appropriately
//                 println!("Error creating color buffer: {}", err);
//                 None
//             }
//         };
//         self
//     }
//
//     pub fn with_event_pump(&mut self, event_pump: Result<sdl2::EventPump, String>) -> &mut Self {
//         self.event_pump = match event_pump {
//             Ok(pump) => Some(pump),
//             Err(err) => {
//                 eprintln!("Error creating event pump: {}", err);
//                 None
//             }
//         };
//         self
//     }
//
//     pub fn with_time_keeper(&mut self, time_keeper: TimeKeeper) -> &mut Self {
//         self.timekeeper = Some(time_keeper);
//         self
//     }
// }
