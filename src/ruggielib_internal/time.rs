use std::time::Instant;

/// Returns time since the application started running in seconds
pub fn get_time_since_initialization() -> u64{
    Instant::now().elapsed.as_secs()
}