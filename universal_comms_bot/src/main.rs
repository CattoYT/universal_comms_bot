use std::{
    io::{self, Read, Stdin},
    sync::Arc,
};

use crossbeam::channel::Receiver;
use sysinfo::{self, ProcessesToUpdate};
mod managers;
mod screenshots;

use crate::{managers::league, screenshots::frame::FrameData};

fn main() {
    println!("Hello, world!");

    let mut managers: Vec<fn(Receiver<Arc<FrameData>>)> = vec![league::minimap::process_map_data];
    // TODO: actually make sure this doesnt error cuz idk rust well enough to tell if having 2 mut refs like that will piss off the borrow checker
    if check_for_league_and_return_managers(&mut managers).is_none() {
        // check_for_valorant_and_return_managers(&mut managers);
    }

    let (raw_screenshot_recv, screenshot_controller) =
        screenshots::capture::spawn_screenshotting_thread();

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

// both of these funcs are very shitcoded and tbh need rewrites (i havent even finished writing them yet)

fn check_for_league_and_return_managers(
    managers: &mut Vec<fn(Receiver<Arc<FrameData>>)>,
) -> Option<()> {
    //TODO: NEEDS REFACTOR -> check league game first you fucking buffoon

    let mut system = sysinfo::System::new();
    system.refresh_processes(ProcessesToUpdate::All, true);
    //league check
    let mut x = system.processes_by_name("LeagueClient".as_ref()).peekable();
    if x.peek().is_none() {
        //league client not open
        println!("The League client is not open! Checking League game...");
        x = system
            .processes_by_name("League of Legends".as_ref())
            .peekable(); //check if game is already open

        if x.peek().is_none() {
            // * league client closed, game closed
            return None;
        } else {
            // * league client closed game open

            //league game is already open
            //this branch will almost never be reached cuz most people dont turn on the setting where the league client closes when the game is open
            // may as well account for it tho
            managers.push(league::minimap::process_map_data);
            return Some(());
        }
    } else {
        x = system
            .processes_by_name("League of Legends".as_ref())
            .peekable();

        if x.peek().is_none() {
            // * league client open game closed
            println!("League Game not found. Initiating lock in...");
            {
                let mut champion = String::new();
                println!(
                    "when in the start queue menu, enter your champion below and press enter: \n"
                );
                let _ = io::stdin().read_line(&mut champion);
                match managers::league::lock_in::start_queue_lock_in(&champion) {
                    Ok(_) => {
                        // locking in was successful
                    }
                    Err(_) => {
                        //shit happened, let user do it tehemslef
                        println!(
                            "Failed to start queue. Please press enter once you have entered the game."
                        );
                        let mut rust_skill_issue = String::new();
                        io::stdin().read_line(&mut rust_skill_issue);
                    }
                }
                managers.push(league::minimap::process_map_data);
                return Some(());
            }
        } else {
            // * league client and game is open
            println!("League game found. ");
            managers.push(league::minimap::process_map_data);
            return Some(());
        }
    }
}
// fn check_for_valorant_and_return_managers(managers: &mut Vec<fn(Receiver<Arc<FrameData>>)>) -> Option<()> {
//     {
//         //Valorant check
//         let mut system = sysinfo::System::new();
//         system.refresh_processes(ProcessesToUpdate::All, true);
//         let mut x = system.processes_by_name("LeagueClient".as_ref()).peekable();
//         match x.peek() {
//             Some(_) => {
//                 todo!("Add valorant managers and modules");
//                 managers.push();
//                 return Some(())
//             }
//             None => {
//                 println!("Valorant is not open!");
//                 return None
//             }
//         }

//     }
// }
