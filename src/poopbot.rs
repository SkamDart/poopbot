extern crate slack_api as slack;

use std::collections::VecDeque;
use std::env;

const SLACK_TOKEN: &str = "SLACK_TOKEN";
const NO_SLACK_TOKEN: &str = "Pls set SLACK_TOKEN for poopbot. :(";

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

    fn get_slack_token() -> String {
       match env::var(SLACK_TOKEN) {
            Ok(token) => token,
            Err(_) => panic!(NO_SLACK_TOKEN)
       }
    }

    pub fn connect(&self) {
        let token: String = PoopBot::get_slack_token();
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


#[cfg(test)]
mod tests {
    use super::*;
    const TOKEN: &str = "FOO";

    #[test]
    fn test_get_existing_slack_token() {
        env::set_var(SLACK_TOKEN, TOKEN);
        assert_eq!(PoopBot::get_slack_token(), TOKEN);
    }

    #[test]
    #[should_panic]
    fn test_get_nonexistant_slack_token() {
        env::set_var(SLACK_TOKEN, TOKEN);
        env::remove_var(SLACK_TOKEN);
        PoopBot::get_slack_token();
    }

}