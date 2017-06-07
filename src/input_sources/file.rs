use std::io::prelude::*;
use std::io::BufReader;
use std::io::Read;
use std::fs::File;
use ::input_sources::Readable;
use ::input_sources::ReadableType;

pub fn process(path: String) -> Readable {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("Input file doesn't exist or isn't readable")
    };

    Readable {
        kind: ReadableType::File,
        file: Some(BufReader::new(file)),
        string: None
    }
}
