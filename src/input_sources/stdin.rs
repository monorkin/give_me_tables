use std::io::prelude::*;
use std::io::BufReader;
use std::io::Read;
use std::io::Cursor;
use ::input_sources::Readable;
use ::input_sources::ReadableType;

pub fn process(content: String) -> Readable {
    Readable {
        kind: ReadableType::String,
        file: None,
        string: Some(BufReader::new(Cursor::new(content.into_bytes())))
    }
}
