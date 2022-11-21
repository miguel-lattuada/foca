use super::{aggregator_trait::Aggregator, aggregator_utils::count_results};
use crate::common::types::ExecutionResult;
use std::{fs::{OpenOptions}, io::Write};

const OUTPUT: &'static str =
    "Number of requests: {requests}\nSuccess requests: {success}\nError requests: {errors}\n";

pub struct AggregatorFile;

impl Aggregator for AggregatorFile {
    fn aggregate(elements: Vec<ExecutionResult>) -> String {
        let (total, success, errors) = count_results(elements);

        OUTPUT
            .replace("{requests}", &total.to_string())
            .replace("{success}", &success.to_string())
            .replace("{errors}", &errors.to_string())
    }
    fn out(input: &String) {
        let mut file_ref = OpenOptions::new().append(true).open("./static/output.txt").unwrap();
        file_ref.write_all(input.as_bytes()).unwrap();
    }
}
