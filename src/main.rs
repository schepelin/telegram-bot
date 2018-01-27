#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::env;

mod requests;

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
// - extract link from message text
// - start downloading file from a link
