use std::time::Instant;

pub struct Timer {
    start_time: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            start_time: Instant::now(),
        }
    }

    pub fn reset(&mut self) {
        self.start_time = Instant::now();
    }

    pub fn get_elapsed_milis(&self) -> u128 {
        (Instant::now() - self.start_time).as_millis()
    }

    pub fn get_elapsed_seconds(&self) -> f32 {
        (Instant::now() - self.start_time).as_secs_f32()
    }
}
