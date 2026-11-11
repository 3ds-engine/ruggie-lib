use std::time::Instant;

pub struct TimeHandler{
    initialization_instant: Instant,
    last_checked_instant: Instant
}

impl TimeHandler{

    pub fn new() -> TimeHandler{
        let now = Instant::now();
        TimeHandler { initialization_instant: now, 
                      last_checked_instant: now }
    }

    pub fn get_seconds_since_initialization(&self) -> f32{
        self.initialization_instant.elapsed().as_secs_f32()
    }

    pub fn get_seconds_since_last_check(&mut self) -> f32{
        let now = Instant::now();
        let secs = now.duration_since(self.last_checked_instant);
        self.last_checked_instant = now;
        secs.as_secs_f32()
    }
}