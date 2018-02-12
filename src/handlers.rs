

use requests::{TelegramRequester, Update};

 #[derive(PartialEq)]
pub enum Reply {
    Alive,

    UnknownMessage,
}

impl Reply {
    fn new(message: &String) -> Self {
        match message.as_ref() {
            "ping" => Reply::Alive,
            _ => Reply::UnknownMessage,
        }
    }
    fn to_string(&self) -> String {
        match *self {
            Reply::Alive => String::from("I'm alive"),
            Reply::UnknownMessage => String::from("Unknown command"),
        }
    }
}


pub fn update_handler(requester: &TelegramRequester, update: &Update) {
    let (chat_id, message) = update.get_chat_message();

    let reply = Reply::new(&message);
    requester.send_message(chat_id, &reply.to_string())
    // run background job in different thread
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_reply_from_string() {
        let ping_text = String::from("ping");
        assert!(Reply::new(&ping_text) == Reply::Alive)
    }
}
