- move `batch`, `builder`, `threading` to this folder
- put definitions on the top and then implementations
- file explanations:
    - `batch`: execute http requests by batch using custom async executor from https://rust-lang.github.io/async-book/02_execution/04_executor.html
    - `threading`: threadpool custom implementation from https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch20-04-storing-threads.html
    - `builder`: load test builder/executor (will rename the class, split it won't make sense), read the options (provided by the interface) creates the threadpool and run http request batches inside each thread