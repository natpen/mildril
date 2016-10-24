extern crate hyper;

use std::env;

use self::hyper::Client;

mod fetching;
mod parsing;

use fetching::fetch_url;
use parsing::{get_urls, parse_html};

fn main() {
    let client = Client::new();
    let args = env::args().collect();
    let url = parse_cl_args(&args);

    let buf = fetch_url(&client, url);

    let dom = parse_html(buf);

    let urls = get_urls(dom.document);
    for url in urls {
        println!("{}", url);
    }
}

fn parse_cl_args(args: &Vec<String>) -> &String {
    // for arg in args {
    //     println!("{}", arg);
    // }

    if args.len() < 2 {
        panic!("Please pass in a url as a command line argument");
    }

    return &args[1];
}
