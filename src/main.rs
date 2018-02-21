#[macro_use]
extern crate clap;
#[macro_use]
extern crate html5ever;

use std::fs::File;
use std::io::{self, BufReader, Write};

use html5ever::parse_document;
use html5ever::rcdom::{Handle, NodeData, RcDom};
use html5ever::tendril::TendrilSink;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const ATTRS: &[&str] = &["href", "src"];

fn create_config(handle: Handle, prefix: &str, exts: &[&str]) {
    let node = handle;
    if let NodeData::Element {
        ref name,
        ref attrs,
        ..
    } = node.data
    {
        assert!(name.ns == ns!(html));
        for attr in attrs.borrow().iter() {
            let name = format!("{}", attr.name.local);
            if ATTRS.iter().any(|a| name == *a) && !attr.value.starts_with("//")
                && !attr.value.contains("://")
                && exts.iter().any(|e| attr.value.ends_with(e))
            {
                print!("    http2_push ");
                if !attr.value.starts_with('/') {
                    print!("{}/", prefix);
                }
                if attr.value.starts_with("./") {
                    print!("{}", &attr.value[2..]);
                } else {
                    print!("{}", attr.value);
                }
                println!(";");
            }
        }
    }

    for child in node.children.borrow().iter() {
        create_config(child.clone(), prefix, exts);
    }
}

fn main() {
    let matches = clap_app!(nginx_http2_push_generator =>
                            (version: VERSION.unwrap_or("unknown"))
                            (author: "Valentin B. <mail@vbrandl.net>")
                            (about: "Parse a HTML file and generate a HTTP2 push configuration for nginx")
                            (@arg PREFIX: -p --prefix +takes_value "Set a path prefix. Should be the path where the file is located. Defaults to \"/\"")
                            (@arg FILE: -i --input +takes_value "Input value. If not set or if set to \"-\", data is read from stdin")
                            (@arg EXT: ... +required "File extensions to include")
                           ).get_matches();
    let input = matches.value_of("FILE").unwrap_or("-");
    let pref = matches.value_of("PREFIX").unwrap_or("");
    let exts: Vec<_> = matches.values_of("EXT").unwrap().collect();
    let dom = parse_document(RcDom::default(), Default::default()).from_utf8();
    let stdin = io::stdin();
    let dom = if input == "-" {
        dom.read_from(&mut stdin.lock())
    } else {
        dom.read_from(&mut BufReader::new(File::open(input).unwrap()))
    }.unwrap();
    let stderr = io::stderr();
    let mut stderr = stderr.lock();
    if !dom.errors.is_empty() {
        writeln!(
            stderr,
            "The following errors occurred when parsing the HTML file:"
        ).ok();
        dom.errors
            .iter()
            .for_each(|e| writeln!(stderr, "{}", e).unwrap());
    } else {
        println!(
            "location = {}/{} {{",
            pref,
            if input == "-" { "file.html" } else { input }
        );
        create_config(dom.document, pref, &exts);
        println!("}}");
    }
}
