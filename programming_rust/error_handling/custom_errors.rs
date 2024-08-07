use std::fmt;

#[derive(Debug, Clone)]
struct JsonError {
    message: String,
    line: usize,
    column: usize,
}

// So that the error can be printed
impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} ({}:{})", self.message, self.line, self.column)
    }
}

// Implement the std::error::Error trait, use default method definitions
impl std::error::Error for JsonError {}

fn failure() -> Result<(), JsonError> {
    Err(JsonError {
        message: "Json Error".to_string(),
        line: 21,
        column: 17,
    })
}

fn main() {
    if let Err(json_err) = failure() {
        eprintln!("{}", json_err);
        std::process::exit(1);
    }
}
