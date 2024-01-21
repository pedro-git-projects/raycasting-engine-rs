const FPS: u64 = 60;

const FRAME_TIME_TARGET: u64 = 1000 / FPS;

#[derive(Default)]
pub struct TimeKeeper {
    ticks_last_frame: u64,
    delta_time: f64,
    frame_time_target: u64,
    wait_time: u64,
}

impl TimeKeeper {
    pub fn default() -> Self {
        Self {
            ticks_last_frame: 0,
            frame_time_target: FRAME_TIME_TARGET,
            ..Default::default()
        }
    }

    pub fn set_ticks(&mut self, ticks: u64) {
        self.ticks_last_frame = ticks;
    }

    pub fn delta_time(&self) -> f64 {
        self.delta_time
    }

    pub fn calculate_delta(&mut self) {
        unsafe {
            let d = (sdl2::sys::SDL_GetTicks64() - self.ticks_last_frame) as f64 / 1000.0;
            self.delta_time = d;
        }
    }

    pub fn calculate_wait_time(&mut self) {
        unsafe {
            let current_ticks = sdl2::sys::SDL_GetTicks64();
            if let Some(w) = current_ticks.checked_sub(self.ticks_last_frame) {
                self.wait_time = self.frame_time_target.saturating_sub(w);
            } else {
                self.wait_time = u64::MAX;
            }
        }
    }

    pub fn frame_time(&self) -> u64 {
        self.frame_time_target
    }

    pub fn wait_time(&self) -> u64 {
        self.wait_time
    }
}
