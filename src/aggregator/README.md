### Module components
- `aggregator_executor` is the entry point of this component, call it with `aggregator_executor(elements_to_aggregate) -> void`
- `aggregator_trait` interface with the following methods:
    - `aggregate(elements_to_aggregate) -> 'str`, aggregate input elements and return a string
    - `output(aggregated_string) -> void`, output implementation in charge of: displaying the result in console, create a `yaml` or `json` file, etc (Check specific implementation for more)
- `console_aggregator`: `aggregator_trait` implementation for console output