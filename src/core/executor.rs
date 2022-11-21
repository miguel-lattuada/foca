use std::{str::FromStr, thread, time::Duration, sync::Arc};

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
    pub fn execute(&self) {
        if self.duration > 0 && self.rate > 0 {
            let thread_pool = ThreadPool::new(self.workers as usize);
            let mut sec_spent = 0;

            loop {
                if sec_spent < self.duration {
                    // TODO: if it make sense to use Arc
                    let rate = self.rate;
                    let url = String::from(&self.url);
                    let output = String::from(&self.output);

                    // TODO: check if we can use scoped threads
                    thread_pool.execute(move || {
                        let tasks = BatchHttpExecutor::new(url, rate).spawn().run();

                        let elements = tasks.iter().map(|task| ExecutionResult {
                            success: task.success,
                            status_code: task.status_code,
                        });

                        AggregatorExecutor::execute(
                            ExecutionResultOutputType::from_str(&output).unwrap(),
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
