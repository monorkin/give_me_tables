use clap::{Arg, App, ArgMatches};
use ::cli::arguments::*;

pub fn arguments() -> Arguments {
    let matches =
        App::new("Give Me Tables")
        .version("0.1")
        .author("Stanko K. R. <stanko.krtalic@gmail.com>")
        .about("Extract tabular data from documents")
        .arg(Arg::with_name("INPUT")
            .help("Raw input data")
            .required_unless_one(&[
                "input-file",
                "url"
            ])
            .index(1))
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
                "interactive",
                "list-tables"
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
                "interactive",
                "list-aggregation-functions",
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
           .required_unless_one(&[
                "INPUT",
                "url"
            ])
           .takes_value(true))
        .arg(Arg::with_name("url")
           .short("u")
           .long("url")
           .value_name("URL")
           .help("Sets the input data from URL")
           .required_unless_one(&[
                "INPUT",
                "input-file"
            ])
           .takes_value(true))
        .arg(Arg::with_name("interactive")
           .short("i")
           .long("interactive")
           .help("Runs the application in interactive mode"))
        .arg(Arg::with_name("aggregation-function")
           .short("a")
           .long("aggregation-function")
           .value_name("COMMAND")
           .help("Apply an aggregation function. Syntax \"FUNCTION_NAME [, column] [, row]\"")
           .takes_value(true)
           .overrides_with_all(&["interactive"])
           .multiple(true)
           .number_of_values(1)
           .value_terminator(" "))
        .arg(Arg::with_name("transformation")
           .short("t")
           .long("transformation")
           .value_name("COMMAND")
           .help("Apply a transformation. Syntax \"TRANSFORMATION_NAME [, column] [, row]\"")
           .takes_value(true)
           .overrides_with_all(&["interactive"])
           .multiple(true)
           .number_of_values(1)
           .value_terminator(" "))
        .get_matches();

    convert_to_arguments(matches)
}

fn convert_to_arguments(matches: ArgMatches) -> Arguments {
    let input =
        if matches.occurrences_of("input-file") == 1 {
            InputSource::File(
                matches.value_of("input-file")
                .expect("No input file specified")
                .to_string()
            )
        }
        else if matches.occurrences_of("url") == 1 {
            InputSource::Url(
                matches.value_of("url")
                .expect("No URL specified")
                .to_string()
            )
        }
        else {
            InputSource::Stdin(
                matches.value_of("INPUT").unwrap_or("").to_string()
            )
        };

    let output =
        if matches.occurrences_of("output-file") == 1 {
            OutputSource::File(
                matches.value_of("output-file")
                .unwrap_or("output")
                .to_string()
            )
        }
        else {
            OutputSource::Stdout
        };

    let verbosity =
        match matches.occurrences_of("verbose") {
            1 => Verbosity::Error,
            2 => Verbosity::Warning,
            3 => Verbosity::Info,
            _ => Verbosity::Silent
        };

    let aggregation_functions = parse_aggregation_functions(&matches);

    let transformations = parse_transformations(&matches);

    let list_tables = matches.occurrences_of("list-tables") == 1;

    let list_available_aggregation_functions =
        matches.occurrences_of("list-aggregation-functions") == 1;

    let list_available_transformations =
        matches.occurrences_of("list-transformations") == 1;

    let interactive_mode =
        matches.occurrences_of("interactive") == 1;

    Arguments::new(
        input,
        output,
        verbosity,
        aggregation_functions,
        transformations,
        list_tables,
        list_available_aggregation_functions,
        list_available_transformations,
        interactive_mode
    )
}

fn parse_aggregation_functions(matches: &ArgMatches) -> Vec<AggregationFunction>
{
    if matches.occurrences_of("aggregation-function") == 0 {
        return vec![];
    }

    let values = matches.values_of("aggregation-function")
        .expect("No aggregation functions found");

    fn map_function(value: &str) -> AggregationFunction {
        let (name, column, row) = parse_function_command(value);

        AggregationFunction::new(
            name,
            column,
            row
        )
    }

    values.into_iter().map(map_function).collect::<Vec<AggregationFunction>>()
}

fn parse_transformations(matches: &ArgMatches) -> Vec<Transformation> {
    if matches.occurrences_of("transformation") == 0 {
        return vec![];
    }

    let values = matches.values_of("transformation")
        .expect("No transformations found");

    fn map_function(value: &str) -> Transformation {
        let (name, column, row) = parse_function_command(value);

        Transformation::new(
            name,
            column,
            row
        )
    }

    values.into_iter().map(map_function).collect::<Vec<Transformation>>()
}

fn parse_function_command(value: &str)
    -> (String, Option<String>, Option<i64>) {
    let command = value.to_string();
    let mut components = command.split(",")
        .into_iter()
        .map(|string| string.trim().to_string())
        .collect::<Vec<String>>();

    components.reverse();

    let name = match components.pop() {
        Some(name) => name.to_string(),
        _ => panic!("No name componenct specified for command '{:?}'", command)
    };

    let column = match components.pop() {
        Some(column) => Some(column.to_string()),
        _ => None
    };

    let row = match components.pop() {
        Some(row) => {
            let row = row
                .to_string()
                .parse::<i64>()
                .expect("Could not parse row integer for command");

            Some(row)
        },
        _ => None
    };

    (name, column, row)
}
