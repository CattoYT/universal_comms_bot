
use std::sync::Arc;

mod managers;
mod screenshots;

use crate::managers::league::process_map_data;

fn main() {
    println!("Hello, world!");

    // highgui::named_window("Demo", WINDOW_NORMAL).expect("ONO");

    #[allow(unused_variables)]
    let (raw_screenshot_recv, screenshot_controller) =
        screenshots::capture::spawn_screenshotting_thread();

    let managers = vec![
        // add managers here
        process_map_data,
    ];

    let mut channels = vec![];

    for x in managers.iter() {
        let (consumer_sender, consumer_recv) = crossbeam::channel::unbounded();
        channels.push(consumer_sender);
        x(consumer_recv);
    }

    std::thread::spawn(move || {
        loop {
            let screenshot = Arc::new(raw_screenshot_recv.recv().unwrap());
            for channel in &channels {
                let _ = channel.send(screenshot.clone());
            }
        }
    });
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
