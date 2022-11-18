use std::{thread, time::Duration};

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
}

impl Executor {
    pub fn execute(&self) {
        if self.duration > 0 && self.rate > 0 {
            let thread_pool = ThreadPool::new(self.workers as usize);
            let mut sec_spent = 0;

            loop {
                if sec_spent < self.duration {
                    // TODO: check how to avoid creating a copy for each thread
                    let rate = self.rate;
                    let url = String::from(&self.url);

                    thread_pool.execute(move || {
                        let tasks = BatchHttpExecutor::new(url, rate).spawn().run();

                        let elements = tasks.iter().map(|task| ExecutionResult {
                            success: task.success,
                            status_code: task.status_code,
                        });

                        // TODO: map option -o (--output) to execution result output type enum
                        AggregatorExecutor::execute(
                            ExecutionResultOutputType::Console,
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
