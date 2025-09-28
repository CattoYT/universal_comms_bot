use std::{io::{self, Read, Stdin}, sync::Arc};

use sysinfo::{self, ProcessesToUpdate};
mod managers;
mod screenshots;

use crate::managers::league::minimap::process_map_data;

fn main() {
    println!("Hello, world!");
    let mut system = sysinfo::System::new();
    system.refresh_processes(ProcessesToUpdate::All, true);
    let mut x = system
        .processes_by_name("League of Legends".as_ref())
        .peekable();

    if x.peek().is_none() {
        println!("Initiating lock in. Please enter queue.");
        {
            let mut champion = String::new();
            let _ = io::stdin().read_line(&mut champion);
            match managers::league::lock_in::start_queue_lock_in(&champion) {
                Ok(_) => {},
                Err(_) => {
                    println!("Failed to start queue. Please press enter once you have entered the game.");
                    let mut rust_skill_issue = String::new();
                    io::stdin().read_line(&mut rust_skill_issue);
                }
            }

            

        }
    }

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
