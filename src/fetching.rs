extern crate hyper;

use std::io::Read;

use self::hyper::Client;

pub fn fetch_url(client: &Client, url: &String) -> String {
    let mut response = match client.get(url).send() {
        Ok(response) => response,
        Err(_) => panic!("Error getting {}", url),
    };
    let mut buf = String::new();
    match response.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(_) => panic!("Error reading response from {}", url),
    };
    buf
}
