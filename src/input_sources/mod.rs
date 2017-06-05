pub mod stdin;
pub mod file;
pub mod url;

use ::cli::arguments::*;

pub fn read(input: InputSource) -> String {
    match input {
        InputSource::Stdin(content) => stdin::process(content),
        InputSource::File(path) => file::process(path),
        InputSource::Url(url) => url::process(url),
        _ => panic!("Unknown input source")
    }
}
