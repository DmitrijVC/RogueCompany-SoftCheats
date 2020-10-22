/*

    Very raw, I have to optimize it

*/

use discord_rpc_client::Client;
use crate::VERSION;
use std::{thread, time};

pub struct RPC;

impl RPC {

    pub fn run() { thread::spawn(|| {

        // Feel free to use this app, I don't really care much.
        let mut drpc = Client::new(763874319762718760).unwrap();

        drpc.start();

        loop {

            match drpc.set_activity(|act| {
                act.state("Cheating in RogueCompany")

                    .details(format!("RC Edition [v{}]", VERSION))
                    // .timestamps(move |timestamp| {
                    //     timestamp.start(time::SystemTime::now().elapsed().unwrap().as_secs() as u64)
                    // })
                    .assets(|assets| {
                        assets.large_image("icon_huge")
                            .small_image("rc")
                            .small_text("RogueCompany edition")
                            .large_text("Made with <3 by FssAy")
                    })
            }) {
                Ok(_) => {}
                Err(_) => {}
            }

            thread::sleep(time::Duration::from_millis(500));
        }

    } ); }

}
