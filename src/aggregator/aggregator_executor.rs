use crate::common::types::{ExecutionResult, ExecutionResultOutputType};

use super::{
    aggregagor_file::AggregatorFile, aggregator_console::AggregatorConsole,
    aggregator_trait::Aggregator,
};

pub struct AggregatorExecutor;

impl AggregatorExecutor {
    pub fn execute(output_type: ExecutionResultOutputType, elements: Vec<ExecutionResult>) {
        match output_type {
            ExecutionResultOutputType::File => {
                let result = AggregatorFile::aggregate(elements);
                AggregatorFile::out(&result);
            }
            ExecutionResultOutputType::Console => {
                let result = AggregatorConsole::aggregate(elements);
                AggregatorConsole::out(&result);
            }
        }
    }
}
