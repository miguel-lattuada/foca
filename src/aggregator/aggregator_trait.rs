pub trait Aggregator {
    fn aggregate(elements: Vec<AggregateElement>) -> String;
    fn out(input: &String);
}

pub struct AggregateElement {
    pub success: bool,
    pub status_code: u16,
}
