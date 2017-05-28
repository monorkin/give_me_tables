use clap::{Arg, App};
use serde_json::{Value};

pub fn arguments() -> Value {
    let matches =
        App::new("Give Me Tables")
        .version("0.1")
        .author("Stanko K. R. <stanko.krtalic@gmail.com>")
        .about("Extract tabular data from documents")
        .arg(Arg::with_name("INPUT")
           .help("Raw input data")
           .required(false))
        .arg(Arg::with_name("verbose")
           .short("v")
           .long("verbose")
           .multiple(true)
           .overrides_with_all(&["interactive"])
           .help("Sets the level of verbosity"))
        .arg(Arg::with_name("list-aggregation-functions")
           .short("A")
           .long("list-aggregation-functions")
           .multiple(false)
           .overrides_with_all(&[
                "interactive", "list-tables"
           ])
           .help("Lists all available aggregation functions"))
        .arg(Arg::with_name("list-transformations")
           .short("T")
           .long("list-transformations")
           .multiple(false)
           .overrides_with_all(&[
                "interactive", "list-tables"
           ])
           .help("Lists all available transformations"))
        .arg(Arg::with_name("list-tables")
           .short("l")
           .long("list-tables")
           .multiple(false)
           .overrides_with_all(&[
                "interactive", "list-aggregation-functions",
                "list-transformations"
           ])
           .help("Lists all available tables in a document"))
        .arg(Arg::with_name("output-file")
           .short("o")
           .long("output-file")
           .value_name("FILE")
           .help("Sets the desired output file")
           .takes_value(true))
        .arg(Arg::with_name("input-file")
           .short("f")
           .long("input-file")
           .value_name("FILE")
           .help("Sets the desired input file")
           .takes_value(true))
        .arg(Arg::with_name("url")
           .short("u")
           .long("url")
           .value_name("URL")
           .help("Sets the input data from URL")
           .takes_value(true))
        .arg(Arg::with_name("interactive")
           .short("i")
           .long("interactive")
           .help("Runs the application in interactive mode"))
        .arg(Arg::with_name("aggregation-function")
           .short("a")
           .long("aggregation-function")
           .value_name("COMMAND")
           .help("Apply an aggregation function. Syntax \"FUNCTION_NAME [, col X [-Y]] [, row X [-Y]]\"")
           .takes_value(true)
           .overrides_with_all(&["interactive"])
           .multiple(true))
        .arg(Arg::with_name("transformation")
           .short("t")
           .long("transformation")
           .value_name("COMMAND")
           .help("Apply a transformation. Syntax \"TRANSFORMATION_NAME [, col X [-Y]] [, row X [-Y]]\"")
           .takes_value(true)
           .overrides_with_all(&["interactive"])
           .multiple(true))
        .get_matches();

    println!("{:?}", matches);

    json!({})
}
