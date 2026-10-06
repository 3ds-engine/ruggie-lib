use thiserror::Error;

#[derive(Error, Debug)]
pub enum InputError {
    #[error("Could not set 3DS accelerometer")]
    AccelerometerSet,
    #[error("Could not set 3DS gyroscope")]
    GyroscopeSet,
    #[error("Could not read 3DS accelerometer")]
    AccelerometerRead,
    #[error("Could not read 3DS gyroscope")]
    GyroscopeRead
}