extern crate slack;

use std::collections::HashMap;

use regex::Regex;
use slack::Event;


pub struct SlackHandler<F> {
    caller: Option<Regex>,
    event_handler: F,
    user_from_id: HashMap<String, slack::User>
}


impl<F> SlackHandler<F> where F: Fn(&Message, &slack::RtmCLient) {
    pub fn new(event_handler: F) -> SlackHandler<F> {
        SlackHandler {
            caller: None,
            event_handler,
            user_from_id: HashMap::new()
        }
    }
}
