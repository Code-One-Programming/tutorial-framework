extern crate comrak;
use comrak::{parse_document, format_html, Arena, ComrakOptions};
use comrak::nodes::{AstNode, NodeValue};
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// The returned nodes are created in the supplied Arena, and are bound by its lifetime.
let arena = Arena::new();

// let root = parse_document(
//     &arena,
//     "This is my input.\n\n1. Also my input.\n2. Certainly my input.\n",
//     &ComrakOptions::default());

fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
    where F : Fn(&'a AstNode<'a>) {
    f(node);
    for c in node.children() {
        iter_nodes(c, f);
    }
}

fn format_plugins<'a>(root: &'a AstNode<'a>) {
    iter_nodes(root, &|node| {
        match &mut node.data.borrow_mut().value {
            &mut NodeValue::Text(ref mut text) => {
                let orig = std::mem::replace(text, vec![]);
                *text = String::from_utf8(orig).unwrap().replace("my", "your").as_bytes().to_vec();
            }
            _ => (),
        }
    });
}

// assert_eq!(
//     String::from_utf8(html).unwrap(),
//     "<p>This is your input.</p>\n\
//      <ol>\n\
//      <li>Also your input.</li>\n\
//      <li>Certainly your input.</li>\n\
//      </ol>\n");

#[wasm_bindgen]
extern "C" {
}

#[wasm_bindgen]
pub fn frameworkConvert(doc; &str) -> &str {
    let rawDocument = parse_document(&arena, doc, &ComrakOptions::default());
    let document = format_plugins(document);
    let mut html = vec![];
    let temp = format_html(root, &ComrakOptions::default(), &mut html).unwrap();
    String::from_utf8(html).unwrap()
}
