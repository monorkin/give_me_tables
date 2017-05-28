extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate clap;
#[macro_use]
extern crate log;
extern crate log4rs;

mod inputs;
mod outputs;
mod table;
mod agregation_functions;
mod transformations;
mod utilities;
mod cli;

fn main() {
    let arguments = cli::parser::arguments();
    println!("{:?}", arguments);
}
