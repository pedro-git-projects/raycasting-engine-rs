use super::app::App;

impl<'a> App<'a> {
    pub fn update(&mut self) {
        self.timekeeper.calculate_wait_time();
        if self.timekeeper.wait_time() > 0
            && self.timekeeper.wait_time() <= self.timekeeper.frame_time()
        {
            unsafe { sdl2::sys::SDL_Delay(self.timekeeper.wait_time() as u32) }
        }

        self.timekeeper.calculate_delta();
        unsafe { self.timekeeper.set_ticks(sdl2::sys::SDL_GetTicks64()) }
    }
}
