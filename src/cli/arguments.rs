#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum InputSource {
    Stdin(String),
    File(String),
    Url(String)
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum OutputSource {
    Stdout,
    File(String)
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Verbosity {
    Silent,
    Error,
    Warning,
    Info
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
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

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
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

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
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

    pub fn input_source(self) -> InputSource {
        self.input
    }

    pub fn output_source(self) -> OutputSource {
        self.output
    }

    pub fn verbosity(self) -> Verbosity {
        self.verbosity
    }

    pub fn aggregation_functions(self) -> Vec<AggregationFunction> {
        self.aggregation_functions
    }

    pub fn transformations(self) -> Vec<Transformation> {
        self.transformations
    }

    pub fn list_tables(self) -> bool {
        self.list_tables
    }

    pub fn list_available_aggregation_functions(self) -> bool {
        self.list_available_aggregation_functions
    }

    pub fn list_available_transformations(self) -> bool {
        self.list_available_transformations
    }

    pub fn interactive_mode(self) -> bool {
        self.interactive_mode
    }
}
