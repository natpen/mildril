extern crate hyper;

use std::env;
use std::collections::{HashMap, HashSet};

use self::hyper::Client;

mod fetching;
mod parsing;

use fetching::fetch_url;
use parsing::{get_links, parse_html};

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
    crawl(&client, &starting_urls);
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

fn crawl(client: &Client, urls: &Vec<String>) {

    for url in urls {
        println!("\n{}\n---------------------------", url);
        let mut unique_links = HashSet::new();

        let buf = fetch_url(&client, url);

        let dom = parse_html(buf);

        let links = get_links(dom.document);

        println!("{} links", &links.len());

        for link in links {
            unique_links.insert(link);
        }

        println!("{} unique links", unique_links.len());
    }
}
