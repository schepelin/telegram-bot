
extern crate reqwest;

use serde_json::value::Value;
use self::reqwest::header::ContentType;
use self::reqwest::Url;

pub const HOST_URL: &'static str = "https://api.telegram.org/";


#[derive(Deserialize)]
pub struct Update {
    update_id: usize,
}

#[derive(Deserialize)]
pub struct Response {
    ok: bool,
    result: Vec<Update>,
}


pub struct TelegramRequester {
    host: String,
    token: String,
    last_update_id: usize,
}


impl TelegramRequester {
    pub fn new(host: &str, token: &str) -> Self {
        TelegramRequester {
            host: String::from(host),
            token: String::from(token),
            last_update_id: 0,
        }
    }

    fn construct_url(&self, method_name: &str) -> String {
        let mut url = Url::parse(&self.host).unwrap();
        url.set_path(&format!("{}/{}", self.token, method_name));
        url.into_string()
    }

    fn request_api_method(&self, method_name: &str, params: Option<&Value>) -> reqwest::Result<reqwest::Response> {
        let client = reqwest::Client::new();
        let url = &self.construct_url(method_name);
        let mut builder = client.post(url);

        match params {
            Some(p) => {
                let params_as_str = p.to_string();
                builder.body(params_as_str);
            },
            None => (),
        };
        builder
        .header(ContentType::json())
        .send()
    }

    pub fn get_updates(&self) -> Response {
        let params = json!({"offset": self.last_update_id + 1});
        let result = self.request_api_method("getUpdates", Some(&params));

        match result {
            Ok(mut resp) => {
                match resp.json() {
                    Ok(data) => data,
                    Err(_) => panic!("JSON parse error"),
                }
            },
            Err(_) => panic!("Request failed"),
        }
    }
}

#[cfg(test)]
mod test {
    extern crate mockito;

    use super::*;
    use self::mockito::{mock, SERVER_URL};

    const TOKEN: &str = "/bot100500:topsecrethash";

    #[test]
    fn initialzie_requester() {
        let requester = TelegramRequester::new("host", TOKEN);
        assert!(requester.host == String::from("host"));
        assert!(requester.token == TOKEN);
    }

    #[test]
    fn requester_get_updates_makes_http_request() {
        let mock = mock("POST", "/bot100500:topsecrethash/getUpdates")
            .with_status(200)
            .match_header("content-type", "application/json")
            .match_body("{\"offset\":1}")
            .with_body("{\"ok\":true,\"result\":[]}")
            .expect(1)
            .create();
        let requester = TelegramRequester::new(SERVER_URL, TOKEN);
        let _ = requester.get_updates();
        mock.assert();
    }

    #[test]
    fn construct_url_for_result() {
        let requester = TelegramRequester::new("https://host", "token");
        let result = requester.construct_url("getUpdates");
        assert!(result == String::from("https://host/token/getUpdates"));
    }

    #[test]
    fn request_api_method_makes_http_request() {
        let requester = TelegramRequester::new(SERVER_URL, "secret");
        let mock = mock("POST", "/secret/method")
            .with_status(200)
            .match_header("content-type", "application/json")
            .match_body(r#"{"foo":"bar"}"#)
            .expect(1)
            .create();
        let params = json!({"foo": "bar"});
        requester.request_api_method(
            "method",
            Some(&params)
        ).unwrap();
        mock.assert();
    }
}
