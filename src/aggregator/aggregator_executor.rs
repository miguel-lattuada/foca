use crate::common::types::{ExecutionResultOutputType, ExecutionResult};

use super::{aggregator_console::AggregatorConsole, aggregator_trait::Aggregator};

pub struct AggregatorExecutor;

impl AggregatorExecutor {
    pub fn execute(output_type: ExecutionResultOutputType, elements: Vec<ExecutionResult>) {
        match output_type {
            ExecutionResultOutputType::File => {}
            ExecutionResultOutputType::Console => {
                let result = AggregatorConsole::aggregate(elements);
                AggregatorConsole::out(&result);
            }
        }
    }
}
