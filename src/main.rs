extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate clap;
#[macro_use]
extern crate log;
extern crate log4rs;

mod input_sources;
mod output_sources;
mod input_formats;
mod output_formats;
mod table;
mod agregation_functions;
mod transformations;
mod utilities;
mod cli;
mod license;

use std::io::Read;

fn main() {
    let arguments = cli::parser::arguments();
    println!("{:?}", arguments);

    if arguments.clone().print_license() {
        license::print();
    }

    let reader = input_sources::read(arguments.input_source());

    let mut content = String::new();

    match reader.string {
        Some(mut buffer) => {
            buffer.read_to_string(&mut content).unwrap();
            ()
        },
        _ => {
            content = "".to_string();
            ()
        }
    };

    println!("INPUT:\n{}", content);
}
