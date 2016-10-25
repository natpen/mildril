extern crate hyper;
extern crate url;

use std::env;
use std::collections::{HashMap, HashSet};

use self::hyper::Client;

use url::Url;

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

fn crawl(client: &Client, urls: &Vec<String>, mut visited_urls: &mut HashSet<String>) {
    for url in urls {
        // TODO: add standardized url to hash_set
        visited_urls.insert(url.to_string());
    }
    for url in urls {
        let buf = fetch_url(&client, url);
        let dom = parse_html(buf);
        let links = get_links(dom.document);
        let mut unique_links: HashSet<String> = HashSet::new();
        for link in links {
            // TODO: add standardized url to hash_set
            match Url::parse(&link) {
                Ok(_) => {
                    unique_links.insert(link);
                    ()
                }
                Err(_) => (),
            }
        }
        let mut links: Vec<String> = vec![];
        for link in &unique_links {
            if visited_urls.contains(link) {
                continue;
            }
            links.push(link.to_string());
        }
        println!("{url} ({new_links_count} new urls found)",
                 url = url,
                 new_links_count = &links.len());
        crawl(&client, &links, &mut visited_urls);
    }
}
