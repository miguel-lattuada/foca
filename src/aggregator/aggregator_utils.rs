use crate::common::types::ExecutionResult;

// count_results -> (total, success, error)
pub fn count_results(elements: Vec<ExecutionResult>) -> (usize, usize, usize) {
    let total = elements.len();
    let success = elements
        .iter()
        .filter(|agg_element| agg_element.success)
        .collect::<Vec<_>>()
        .len();

    let errors = total - success;

    (total, success, errors)
}
