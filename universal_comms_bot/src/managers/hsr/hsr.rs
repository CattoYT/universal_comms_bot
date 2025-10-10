use std::{sync::Arc, thread::sleep, time::Duration};


use crate::{managers::hsr::{divergent_universe_spammer, relic_farm}, screenshots::frame::FrameData};

pub fn switcher_manual(recv: crossbeam::channel::Receiver<Arc<FrameData>>) {
    // ok so basically i wanna run detection in here and switch between relics and divergent universe
    // but like automatically or something idfk i need hours for hackberry

    let mut user_buffer = String::new();

    loop {
        println!(
            "Welcome to the (worst and probably most undetectable) HSR autofarmer\n1. Divergent Universe\n2. Relics"
        );
        match std::io::stdin().read_line(&mut user_buffer) {
            Ok(_) => {
                println!("{user_buffer}");
                if user_buffer.trim() == "1".to_string() || user_buffer.trim() == "2".to_string() {
                    break;
                } else {
                    println!("Stop being dumb");
                    sleep(Duration::from_secs(1));
                    print!("{}[2J", 27 as char);
                }
            }
            Err(_) => {
                println!("")
            }
        }
    }

    match user_buffer.trim() {
        "1" => {
            println!("Starting divergent universe!");
            divergent_universe_spammer::spam_divergent_universe(recv);
        }
        "2" => {
            relic_farm::spam_relics(recv);
        }

        _ => panic!("Should have been caught earlier"),
    }
}

// pub fn switcher_auto(recv: crossbeam::channel::Receiver<Arc<FrameData>>) {
//     //TODO: take an ss then just compare whether its divergent universe or a calyx
    
// }