use std::{
    io,
    process::exit,
    sync::Arc, thread::sleep, time::Duration,
};

use crossbeam::channel::Receiver;
use sysinfo::{self, ProcessesToUpdate};
pub mod autogui;
mod managers;
mod screenshots;
use crate::{managers::league, screenshots::frame::FrameData};

fn main() {
    let mut managers: Vec<fn(Receiver<Arc<FrameData>>)> = vec![];
    //this looks really ugly, but i dont want it to bother with checking if the game is already found lol
    // also since this is only screenreading i dont need to bother with multiple games open tbh
    // in retrospect a lot of this code can be refactored to take that into account lmao
    // gj me 
    check_for_league_and_return_managers(&mut managers);
    check_for_valorant_and_return_managers(&mut managers);
    check_for_star_rail_and_return_managers(&mut managers);


    if managers.len() == 0 {
        println!("No games found.");
        exit(0)
    }
        if managers.len() > 1 {
        println!("Multiple games found, continuing but its untested as to whether this works or not");
        sleep(Duration::from_secs(5));
        exit(0)
    }


    let raw_screenshot_recv = screenshots::capture::spawn_screenshotting_thread();
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

fn check_for_star_rail_and_return_managers(
    managers: &mut Vec<fn(Receiver<Arc<FrameData>>)>,
) -> Option<()> {
    //Valorant check
    let mut system = sysinfo::System::new();
    system.refresh_processes(ProcessesToUpdate::All, true);
    let mut x = system.processes_by_name("StarRail".as_ref()).peekable();
    match x.peek() {
        Some(_) => {
            println!("Star Rail found!");
            managers.push(managers::hsr::hsr::switcher_manual);
            return Some(());
        }
        None => {
            println!("Star rail is not open!");
            return None;
        }
    }

}

// oh hey it looks a lot better now
fn check_for_league_and_return_managers(
    managers: &mut Vec<fn(Receiver<Arc<FrameData>>)>,
) -> Option<()> {
    let mut system = sysinfo::System::new();
    system.refresh_processes(ProcessesToUpdate::All, true);

    let mut game_check = system
        .processes_by_name("League of Legends".as_ref())
        .peekable();
    if game_check.peek().is_some() {
        println!("League game found!");
        managers.push(league::minimap::process_map_data);
        return Some(());
    }
    println!("League game not found! Checking client...");
    let mut client_check = system.processes_by_name("LeagueClient".as_ref()).peekable();
    if client_check.peek().is_some() {
        println!("League Client found!");
        {
            let mut champion = String::new();
            println!(
                "When in the start queue menu, enter your champion below and press enter. \nWhen pressing enter, make sure that the \"START QUEUE\" button is completely visible! \nThe program might fail otherwise!\nChampion: "
            );
            let _ = io::stdin().read_line(&mut champion);
            if let Err(_) = managers::league::lock_in::start_queue_lock_in(&champion) {
                //shit happened, let user do it tehemslef
                println!(
                    "Failed to start queue. Please press enter once you have entered the game."
                );
                let mut rust_skill_issue = String::new();
                let _ = io::stdin().read_line(&mut rust_skill_issue);
            }

            managers.push(league::minimap::process_map_data);
            return Some(());
        }
    }
    println!("League client not found!");

    None
}
fn check_for_valorant_and_return_managers(
    managers: &mut Vec<fn(Receiver<Arc<FrameData>>)>,
) -> Option<()> {
    {
        //Valorant check
        let mut system = sysinfo::System::new();
        system.refresh_processes(ProcessesToUpdate::All, true);
        let mut x = system.processes_by_name("VALORANT".as_ref()).peekable();
        match x.peek() {
            Some(_) => {
                managers.push(managers::valorant::enemy_detection::process_valorant);
                return Some(());
            }
            None => {
                println!("Valorant is not open!");
                return None;
            }
        }
    }
}
