use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use crate::screenshots::frame::{self, FrameData};
use crossbeam::channel::{Receiver, Sender, unbounded};

#[cfg(target_os = "windows")]
use windows_capture::{
    capture::{Context, GraphicsCaptureApiHandler},
    frame::Frame,
    graphics_capture_api::InternalCaptureControl,
    monitor::Monitor,
    settings::{
        CursorCaptureSettings, DirtyRegionSettings, DrawBorderSettings,
        MinimumUpdateIntervalSettings, SecondaryWindowSettings, Settings,
    },
};

#[cfg(target_os = "windows")]
pub struct Capture {
    start: Instant,

    sender: Sender<FrameData>,
}
#[cfg(target_os = "windows")]
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

        sleep(Duration::from_millis(1000 / 2)); //improvised delay cuz the other one isnt supported?
        // replace second value with desired frame rate
        result
    }

    fn on_closed(&mut self) -> Result<(), Self::Error> {
        println!("Ran for {}", self.start.elapsed().as_secs());
        Ok(())
    }
}

#[cfg(target_os = "windows")]
pub fn spawn_screenshotting_thread() -> Receiver<FrameData> {
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

    // let capture_thread = Capture::start_free_threaded(settings).unwrap();
    let _ = Capture::start_free_threaded(settings).unwrap();

    recv
}

#[cfg(target_os = "macos")]
pub fn spawn_screenshotting_thread() -> Receiver<FrameData> {
    use std::thread;

    use xcap::Monitor;

    let (send, recv) = unbounded();

    // let capture_thread = Capture::start_free_threaded(settings).unwrap();
    let monitors = Monitor::all().unwrap();
    let monitor = monitors
        .into_iter()
        .find(|m| m.is_primary().unwrap_or(false))
        .expect("No primary monitor found");
    let (video_recorder, sx) = monitor.video_recorder().unwrap();

    thread::spawn(move || {
        loop {
            match sx.recv() {
                Ok(frame) => {
                    println!("frame: {:?}", frame.width);
                    send.send(FrameData::from(frame)).unwrap(); // theres hopefully a way to frame rate limit this wthout using the screenshotting part of the library
                    // if not then il cope ig
                }
                _ => continue,
            }
        }
    });

    recv
}
