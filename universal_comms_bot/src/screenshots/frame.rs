#[allow(dead_code)]
pub struct FrameData {
    pub raw_buffer: Vec<u8>,
    pub height: u32,
    pub width: u32,
}

impl FrameData {
    pub fn new(raw_buffer: Vec<u8>, height: u32, width: u32) -> Self {
        FrameData {
            raw_buffer: raw_buffer,
            height: height,
            width: width,
        }
    }
}