use opendsp_sys::{opendspAudioClose, opendspAudioStream};

pub struct StreamedSound{
    pub(crate)music: *mut opendspAudioStream
}

impl StreamedSound{
    pub fn new(music: *mut opendspAudioStream) -> Self{
        Self { music }
    }
}

impl Drop for StreamedSound{
    fn drop(&mut self) {
        unsafe{
            opendspAudioClose(&mut self.music);
        }
    }
}