use askama::Template;
use pulldown_cmark::{Parser, Options, html};
use walkdir::WalkDir;

use std::fs::File;
use std::io::prelude::*;
use std::ffi::OsStr;
use std::path::Path;


#[derive(Template)]
#[template(path = "default.html")]
pub struct ContentTemplate<'a> {
    content: &'a str,
}

pub fn generate() {
    let files = WalkDir::new("./base")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension() == Some(OsStr::new("md")));

    for entry in files {
        let file_path = entry.path();
        let mut file = File::open(&file_path).expect("Couldn't open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Could not read file");

        let parser = Parser::new_ext(&contents, Options::empty());
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        let content = ContentTemplate { content: &html_output };

        let html_file_name = Path::new("./build").join(file_path.file_stem().expect("missing file stem")).with_extension("html");

        let mut file = File::create(html_file_name).expect("Couldn't create file");
        file.write_all(content.render().unwrap().as_bytes()).expect("couldn't write file");
    }
}
