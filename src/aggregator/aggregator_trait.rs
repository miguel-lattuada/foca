use crate::common::types::ExecutionResult;

pub trait Aggregator {
    fn aggregate(elements: Vec<ExecutionResult>) -> String;
    fn out(input: &String);
}
