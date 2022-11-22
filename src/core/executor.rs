use std::{str::FromStr, sync::Arc, thread, time::Duration};

use crate::{
    aggregator::aggregator_executor::AggregatorExecutor,
    common::types::{ExecutionResult, ExecutionResultOutputType},
};

use super::{batch::BatchHttpExecutor, threading::ThreadPool};

pub struct Executor {
    pub url: String,
    pub rate: u8,
    pub duration: u8,
    pub workers: u8,
    pub output: String,
}

impl Executor {
    pub fn execute(self) {
        let url = Arc::new(self.url.to_string());
        let output = Arc::new(self.output.to_string());
        let rate = Arc::new(self.rate);

        if self.duration > 0 && self.rate > 0 {
            let thread_pool = ThreadPool::new(self.workers as usize);
            let mut sec_spent = 0;

            loop {
                let shared_url = url.clone();
                let shared_output = output.clone();
                let shared_rate = rate.clone();

                if sec_spent < self.duration {
                    // TODO: check if we can use scoped threads
                    thread_pool.execute(move || {
                        let tasks = BatchHttpExecutor::new(&shared_url, *shared_rate)
                            .spawn()
                            .run();

                        let elements = tasks.iter().map(|task| ExecutionResult {
                            success: task.success,
                            status_code: task.status_code,
                        });

                        AggregatorExecutor::execute(
                            ExecutionResultOutputType::from_str(&shared_output).unwrap(),
                            elements.collect::<Vec<ExecutionResult>>(),
                        );
                    });

                    thread::sleep(Duration::from_secs(1));

                    sec_spent += 1;
                    continue;
                }
                break;
            }
        }
    }
}
