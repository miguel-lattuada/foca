use super::aggregator_trait::{AggregateElement, Aggregator};

const OUTPUT: &'static str =
    "Number of requests: {requests}\nSuccess requests: {success}\nError requests: {errors}";

pub struct AggregatorConsole;

impl Aggregator for AggregatorConsole {
    fn aggregate(elements: Vec<AggregateElement>) -> String {
        let total = elements.len();
        let success = elements
            .iter()
            .filter(|agg_element| agg_element.success)
            .collect::<Vec<_>>()
            .len();

        let errors = total - success;

        OUTPUT
            .replace("{requests}", &total.to_string())
            .replace("{success}", &success.to_string())
            .replace("{errors}", &errors.to_string())
    }
    fn out(input: &String) {
        print!("{}", input);
    }
}
