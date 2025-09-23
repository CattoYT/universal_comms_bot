pub struct FrameData {
    raw_buffer: Vec<u8>,
    height: u32,
    width: u32,
}

impl FrameData {
    pub fn new(raw_buffer: Vec<u8>, height: u32, width: u32) -> Self{
        FrameData {
            raw_buffer: raw_buffer,
            height: height,
            width: width
        }
    }
}