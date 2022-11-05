- put definitions on the top and then implementations
- file explanations:
    - `batch`: execute http requests by batch using custom async executor from https://rust-lang.github.io/async-book/02_execution/04_executor.html
    - `threading`: threadpool custom implementation from https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch20-04-storing-threads.html
    - `builder`: load test builder/executor (will rename the class, split it won't make sense), read the options (provided by the interface) creates the threadpool and run http request batches inside each thread, it will also read each batch http result and send it to the aggregator (more in the aggregator README.md)
### TODO
- change the builder name to `test_executor` or `executor`, also remove the builder interface and create the object with new fn directly