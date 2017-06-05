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

fn main() {
    let arguments = cli::parser::arguments();
    println!("{:?}", arguments);

    let content = input_sources::read(arguments.input_source());

    println!("INPUT:\n{}", content);
}
