use std::time::Instant;

use crate::*;

pub struct FpsSystem {
    t0: Instant,
    fps: u32,
}

impl FpsSystem {
    pub fn new() -> FpsSystem {
        FpsSystem {
            t0: Instant::now(),
            fps: 0,
        }
    }
}

impl Default for FpsSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl System for FpsSystem {
    fn on_frame_update(&mut self, _world: &World) {
        self.fps += 1;

        if self.t0.elapsed().as_secs() >= 1 {
            self.t0 = Instant::now();
            eprintln!("FPS: {}", self.fps);
            self.fps = 0;
        }
    }
}
