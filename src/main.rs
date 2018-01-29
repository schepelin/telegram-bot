
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::env;

mod requests;
mod handlers;

const TOKEN_ENV: &str = "TELEGRAM_API_TOKEN";

fn main() {
    let token = match env::var(TOKEN_ENV) {
        Ok(val) => val,
        Err(_) => panic!("Could not initialize config from end. {} var not found", TOKEN_ENV),
    };


    let requester = requests::TelegramRequester::new(&token, 0);
    let resp = requester.get_updates();
    println!("here is the response: {:?}", resp);
}

// TODO:
// + make API request for updates
// - reply to the message
// - add timeout to get_updates and rum long polling
// - extract link from message text
// - start downloading file from a link
