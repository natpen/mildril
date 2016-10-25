extern crate hyper;

use std::io::Read;

use self::hyper::Client;
use self::hyper::header::{Headers, UserAgent};

const MILDRIL_USER_AGENT: &'static str = "";

pub fn fetch_url(client: &Client, url: &String) -> String {
    let mut headers = Headers::new();
    headers.set(UserAgent(MILDRIL_USER_AGENT.to_owned()));
    let mut response = match client.get(url).headers(headers).send() {
        Ok(response) => response,
        Err(_) => return String::new(),
    };
    let mut buf = String::new();
    match response.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(_) => return String::new(),
    };
    buf
}
