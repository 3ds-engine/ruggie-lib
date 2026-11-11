use std::time::Instant;

/// Struct containing initialization and last time checked intants
pub struct TimeHandler{
    initialization_instant: Instant,
    last_checked_instant: Instant
}

impl TimeHandler{

    /// Creates the instants as Instant::now()
    pub fn new() -> TimeHandler{
        let now = Instant::now();
        TimeHandler { initialization_instant: now, 
                      last_checked_instant: now }
    }

    /// Returns time in seconds since the cration of the TimeHandler
    pub fn get_seconds_since_initialization(&self) -> f32{
        self.initialization_instant.elapsed().as_secs_f32()
    }

    /// Returns the differencen in seconds between the last checked instant and now. Then updates the last checked instant 
    /// to be now
    pub fn get_seconds_since_last_check(&mut self) -> f32{
        let now = Instant::now();
        let secs = now.duration_since(self.last_checked_instant);
        self.last_checked_instant = now;
        secs.as_secs_f32()
    }
}