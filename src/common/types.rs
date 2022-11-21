use std::str::FromStr;

pub struct ExecutionResult {
    pub success: bool,
    pub status_code: u16,
}

pub enum ExecutionResultOutputType {
    Console,
    File,
}

impl FromStr for ExecutionResultOutputType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "console" => Ok(ExecutionResultOutputType::Console),
            "file" => Ok(ExecutionResultOutputType::File),
            _ => Err(()),
        }
    }
}
