
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::env;

mod requests;
mod handlers;

const TOKEN_ENV: &str = "TELEGRAM_API_TOKEN";
const UPDATES_TIMEOUT: u8 = 10;


fn main() {
    let token = match env::var(TOKEN_ENV) {
        Ok(val) => val,
        Err(_) => panic!("Could not initialize config from env. {} var not found", TOKEN_ENV),
    };
    let requester = requests::TelegramRequester::new(&token, 0);
    loop {
        let resp = requester.get_updates(UPDATES_TIMEOUT);
        for update in resp.get_updates() {
            handlers::update_handler(&requester, update);
        }
    }
}

// TODO:
// + make API request for updates
// + reply to the message
// + add timeout to get_updates and run long polling
// - extract link from message text
// - start downloading file from a link
