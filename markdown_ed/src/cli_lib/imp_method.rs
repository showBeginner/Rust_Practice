use pulldown_cmark::{Parser, Options, html};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

pub(crate) fn file_read(file_name:&Path) ->std::io::Result<String>{
    let file = File::open(file_name)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;


    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&contents, options);

    // Write to String buffer.
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);


    println!("orignal: {:?}",contents);
    println!("html: {:?}",html_output);

    Ok(html_output)

}