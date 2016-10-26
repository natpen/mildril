extern crate hyper;
#[macro_use]
extern crate lazy_static;

use self::hyper::Client;

use std::env;
use std::collections::{HashMap, HashSet};

mod crawling;
mod fetching;
mod parsing;

use crawling::crawl;

fn main() {
    let mut starting_urls: Vec<String> = vec![];
    let cl_args = env::args().collect();
    let cl_opts = parse_cl_args(&cl_args);
    if cl_opts.is_empty() {
        panic!("Please provide a starting argument");
    }
    if cl_opts.contains_key("url") && cl_opts.contains_key("file") {
        panic!("Please specify a starting url or file, but not both");
    }
    if cl_opts.contains_key("url") {
        starting_urls.push(cl_opts.get("url").unwrap().to_string());
    }
    let client = Client::new();
    let mut visited_urls: HashSet<String> = HashSet::new();
    crawl(&client, &starting_urls, &mut visited_urls);
}

fn parse_cl_args(args: &Vec<String>) -> HashMap<&str, &String> {
    let mut cl_opts = HashMap::new();
    let mut should_skip_next_i = false;
    for i in 0..args.len() {
        if should_skip_next_i {
            should_skip_next_i = false;
            continue;
        }
        should_skip_next_i = true;
        if args.len() > i + 1 && (args[i] == "-u" || args[i] == "--url") {
            cl_opts.insert("url", &args[i + 1]);
        } else {
            should_skip_next_i = false;
        }
        // match args[i] {
        //     "-u" | "--url" => {
        //         if args.len() > i + 1 {
        //             cl_opts.insert("url", &args[i + 1]);
        //         } else {
        //             should_skip_next_i = false;
        //         }
        //         ()
        //     }
        //     _ => {
        //         should_skip_next_i = false;
        //         ()
        //     }
        // }
    }
    return cl_opts;
}
