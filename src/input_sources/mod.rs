pub mod stdin;
pub mod file;
pub mod url;

use ::cli::arguments::*;
use std::io::Read;
use std::io::BufReader;
use std::marker::Sized;
use std::fs::File;
use std::io::Cursor;

pub struct Readable {
    pub kind: ReadableType,
    pub file: Option<BufReader<File>>,
    pub string: Option<BufReader<Cursor<Vec<u8>>>>
}

pub enum ReadableType {
    File,
    String
}

pub fn read(input: InputSource) -> Readable {
    match input.to_owned() {
        InputSource::Stdin(content) => stdin::process(content),
        InputSource::File(path) => file::process(path),
        InputSource::Url(url) => url::process(url),
        _ => panic!("Unknown input source")
    }
}
