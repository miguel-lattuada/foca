use super::{aggregator_trait::Aggregator, aggregator_utils::count_results};
use crate::common::types::ExecutionResult;

const OUTPUT: &'static str =
    "Number of requests: {requests}\nSuccess requests: {success}\nError requests: {errors}\n";

pub struct AggregatorConsole;

impl Aggregator for AggregatorConsole {
    fn aggregate(elements: Vec<ExecutionResult>) -> String {
        let (total, success, errors) = count_results(elements);

        OUTPUT
            .replace("{requests}", &total.to_string())
            .replace("{success}", &success.to_string())
            .replace("{errors}", &errors.to_string())
    }
    fn out(input: &String) {
        print!("{}", input);
    }
}
