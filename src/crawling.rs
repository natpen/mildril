extern crate hyper;
extern crate url;

use std::{thread, time};
use std::collections::HashSet;

use self::hyper::Client;
use self::url::Url;

use fetching::fetch_url;
use parsing::{get_links, parse_html};

pub fn crawl(client: &Client,
             urls: &Vec<String>,
             mut visited_urls: &mut HashSet<String>,
             domain_blacklist: &HashSet<&str>) {
    for url in urls {
        match Url::parse(&url) {
            Ok(parsed_url) => {
                visited_urls.insert(parsed_url.into_string());
                ()
            }
            Err(_) => (),
        }
    }
    for url in urls {
        let one_second = time::Duration::new(1, 0);
        thread::sleep(one_second);
        let buf = fetch_url(&client, url);
        let dom = parse_html(buf);
        let links = get_links(dom.document);
        let mut unique_links: HashSet<String> = HashSet::new();
        for link in links {
            match Url::parse(&link) {
                Ok(url) => {
                    let url_ref = &url;
                    let domain = url_ref.domain().unwrap_or("");
                    if !domain_blacklist.contains(domain) {
                        unique_links.insert(url_ref.to_string());
                    }
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
        println!("{url} | {new_links_count} | {total_links_count}",
                 url = url,
                 new_links_count = &links.len(),
                 total_links_count = &unique_links.len());
        crawl(&client, &links, &mut visited_urls, &domain_blacklist);
    }
}
