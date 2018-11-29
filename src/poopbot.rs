extern crate slack_api;

use std::collections::VecDeque;
use std::env;

pub struct Encouragement {
    quote: String
}

pub struct PoopBot {
    // temporary
    line: VecDeque<u64>
}

impl PoopBot {

    // create poopbot instance
    pub fn new() -> Self {
        PoopBot {
            line: VecDeque::new()
        }
    }

    fn get_token() -> String {
       match env::var("SLACK_TOKEN") {
            Ok(token) => token,
            Err(_) => panic!("Pls set SLACK_TOKEN for poopbot. :(")
       }
    }

    pub fn connect(&self) {
        let token: String = PoopBot::get_token();
        PoopBot::speak();
    }

    pub fn add_pooper(&self) -> u8 {
        42
    }

    pub fn remove_pooper() -> Encouragement {
        Encouragement {
            quote: "Congrats!".to_string()
        }
    }

    pub fn speak() {
        println!("Please poop freely!");
    }
}