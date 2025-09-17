use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct FpsCounter {
    last_time: Instant,
    frames: u32,
    fps: u32,
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            last_time: Instant::now(),
            frames: 0,
            fps: 0,
        }
    }

    pub fn tick(&mut self) -> Option<u32> {
        self.frames += 1;
        let now = Instant::now();

        if now.duration_since(self.last_time) >= Duration::from_secs(1) {
            self.fps = self.frames;
            self.frames = 0;
            self.last_time = now;
            Some(self.fps)
        } else {
            None
        }
    }
}
