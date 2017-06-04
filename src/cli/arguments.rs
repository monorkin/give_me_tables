#[derive(Debug)]
pub enum InputSource {
    Stdin(String),
    File(String),
    Url(String)
}

#[derive(Debug)]
pub enum OutputSource {
    Stdout,
    File(String)
}

#[derive(Debug)]
pub enum Verbosity {
    Silent,
    Error,
    Warning,
    Info
}

#[derive(Debug)]
pub struct AggregationFunction {
    name: String,
    column: Option<String>,
    row: Option<i64>
}

impl AggregationFunction {
    pub fn new(name: String, column: Option<String>, row: Option<i64>)
        -> AggregationFunction {
        AggregationFunction {
            name: name,
            column: column,
            row: row
        }
    }
}

#[derive(Debug)]
pub struct Transformation {
    name: String,
    column: Option<String>,
    row: Option<i64>
}

impl Transformation {
    pub fn new(name: String, column: Option<String>, row: Option<i64>)
        -> Transformation {
        Transformation {
            name: name,
            column: column,
            row: row
        }
    }
}

#[derive(Debug)]
pub struct Arguments {
    input: InputSource,
    output: OutputSource,
    verbosity: Verbosity,
    aggregation_functions: Vec<AggregationFunction>,
    transformations: Vec<Transformation>,
    list_tables: bool,
    list_available_aggregation_functions: bool,
    list_available_transformations: bool,
    interactive_mode: bool
}

impl Arguments {
    pub fn new(
        input: InputSource,
        output: OutputSource,
        verbosity: Verbosity,
        aggregation_functions: Vec<AggregationFunction>,
        transformations: Vec<Transformation>,
        list_tables: bool,
        list_available_aggregation_functions: bool,
        list_available_transformations: bool,
        interactive_mode: bool
    ) -> Self {
        Self {
            input: input,
            output: output,
            verbosity: verbosity,
            aggregation_functions: aggregation_functions,
            transformations: transformations,
            list_tables: list_tables,
            list_available_aggregation_functions:
                list_available_aggregation_functions,
            list_available_transformations: list_available_transformations,
            interactive_mode: interactive_mode
        }
    }
}
