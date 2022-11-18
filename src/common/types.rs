pub struct ExecutionResult {
    pub success: bool,
    pub status_code: u16,
}

pub enum ExecutionResultOutputType {
    Console,
    File,
}
