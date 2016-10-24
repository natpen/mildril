extern crate html5ever;
extern crate tendril;

use self::html5ever::tokenizer::Attribute;
use self::html5ever::driver;
use self::html5ever::rcdom::{Element, ElementEnum, Handle, NodeEnum, RcDom};

use self::tendril::TendrilSink;

pub fn parse_html(source_str: String) -> RcDom {
    driver::parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut source_str.as_bytes())
        .unwrap()
}

pub fn get_links(handle: Handle) -> Vec<String> {
    let mut urls = vec![];

    let mut anchor_tags = vec![];
    get_elements_by_name(handle, "a", &mut anchor_tags);

    for node in anchor_tags {
        if let Element(_, _, ref attrs) = node {
            for attr in attrs.iter() {
                let Attribute { ref name, ref value } = *attr;
                if &name.local == "href" {
                    urls.push(value.to_string());
                }
            }
        }
    }

    urls
}

fn get_elements_by_name(handle: Handle, element_name: &str, out: &mut Vec<NodeEnum>) {
    let node = handle.borrow();

    if let Element(ref name, _, ref attrs) = node.node {
        if &name.local == element_name {
            out.push(Element(name.clone(), ElementEnum::Normal, attrs.clone()));
        }
    }

    for child in &node.children {
        get_elements_by_name(child.clone(), element_name, out);
    }
}
