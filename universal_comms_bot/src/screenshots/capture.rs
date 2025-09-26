use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use crossbeam::channel::{Receiver, Sender, unbounded};
use windows_capture::{
    capture::{CaptureControl, Context, GraphicsCaptureApiHandler},
    frame::Frame,
    graphics_capture_api::InternalCaptureControl,
    monitor::Monitor,
    settings::{
        CursorCaptureSettings, DirtyRegionSettings, DrawBorderSettings,
        MinimumUpdateIntervalSettings, SecondaryWindowSettings, Settings,
    },
};

use crate::screenshots::frame::{self, FrameData};

pub struct Capture {
    start: Instant,

    sender: Sender<FrameData>,
}

impl GraphicsCaptureApiHandler for Capture {
    type Flags = Sender<FrameData>;

    // The type of error that can be returned from `CaptureControl` and `start`
    // functions.
    type Error = Box<dyn std::error::Error + Send + Sync>;

    // Function that will be called to create a new instance. The flags can be
    // passed from settings.
    fn new(ctx: Context<Self::Flags>) -> Result<Self, Self::Error> {
        println!("Created with Flags: {:?}", ctx.flags);

        Ok(Self {
            start: Instant::now(),
            sender: ctx.flags,
        })
    }

    fn on_frame_arrived(
        &mut self,
        frame: &mut Frame,
        _capture_control: InternalCaptureControl,
    ) -> Result<(), Self::Error> {
        // Send the frame to the video encoder
        // self.encoder.as_mut().unwrap().send_frame(frame)?;

        // Note: The frame has other uses too, for example, you can save a single frame
        // to a file, like this: frame.save_as_image("frame.png", ImageFormat::Png)?;
        // Or get the raw data like this so you have full
        // control: let data = frame.buffer()?;
        let mut binding = frame.buffer()?;
        let data = binding.as_raw_buffer();
        let frame_data: frame::FrameData =
            FrameData::new(data.to_vec(), frame.height(), frame.width());

        let result = self
            .sender
            .send(frame_data)
            .map_err(|e: crossbeam::channel::SendError<FrameData>| Box::new(e) as Self::Error);

        sleep(Duration::from_millis(1000 / 10)); //improvised delay cuz the other one isnt supported?
        // replace second value with desired frame rate
        result
    }

    fn on_closed(&mut self) -> Result<(), Self::Error> {
        println!("Ran for {}", self.start.elapsed().as_secs());
        Ok(())
    }
}

pub fn spawn_screenshotting_thread() -> (
    Receiver<FrameData>,
    CaptureControl<Capture, Box<dyn std::error::Error + Send + Sync>>,
) {
    let (send, recv) = unbounded();

    let settings = Settings::new(
        Monitor::primary().expect("There is no primary monitor"),
        CursorCaptureSettings::Default,
        DrawBorderSettings::WithoutBorder,
        SecondaryWindowSettings::Default,
        // MinimumUpdateIntervalSettings::Custom(Duration::from_millis(67)),
        MinimumUpdateIntervalSettings::Default,
        DirtyRegionSettings::Default,
        windows_capture::settings::ColorFormat::Rgba8,
        send.clone(),
    );

    // let capturer: Result<Capture, Box<dyn Error + Send + Sync>> = Capture::new(settings);

    let capture_thread = Capture::start_free_threaded(settings).unwrap();

    (recv, capture_thread)
}
