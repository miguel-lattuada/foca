## interface
A way to configure (interact) the load test (the "core" logic, at this point: `batch.rs` / `builder.rs` / `threading.rs` files)

### types of interfaces
- **cli**: we run the load test using command line options `foca cli --url someurl.io --duration 5 --rate 5`
- **file**: we run the load test using command line but providing options through `json` or `yaml` config files `foca file --json | --yaml <path to file>` (not implemented)

### interface executors
Interface executors will collect options and create proper core objects. They may share a `trait` to "execute" since the output core objects are the same. We should have an executor for each of the sub commands defined in `cli.rs:Commands` enum
- **cli**: will contain the logic to handle `cli` interface type, just read whatever `clap` already parsed for us, and construct the core objects
- **file**: will contain the logic to handle `file` interface type, will parse `json` or `yaml` and construct the core objects
