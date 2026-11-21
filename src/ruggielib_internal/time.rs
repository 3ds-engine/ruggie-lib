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

    /// Returns time since the creation of the TimeHandler
    pub fn since_initialization(&self) -> Duration{
        self.initialization_instant.elapsed()
    }

    /// Returns the difference between the last checked instant and now. Then updates the last checked instant 
    /// to be now
    pub fn since_last_check(&mut self) -> Duration{
        let now = Instant::now();
        let secs = now.duration_since(self.last_checked_instant);
        self.last_checked_instant = now;
        secs
    }
}