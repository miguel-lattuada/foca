use std::{thread, time::Duration};

use crate::aggregator::{
    aggregator_console::AggregatorConsole,
    aggregator_trait::{AggregateElement, Aggregator},
};

use super::{batch::BatchHttpExecutor, threading::ThreadPool};

pub struct LoadTestBuilder {
    url: String,
    rate: u8,
    duration: u8,
    workers: u8,
}

impl LoadTestBuilder {
    pub fn new() -> Self {
        Self {
            url: "".to_string(),
            rate: 0,
            duration: 0,
            workers: 0,
        }
    }

    pub fn url(mut self, url: String) -> Self {
        self.url = url;
        self
    }

    pub fn rate(mut self, rate: u8) -> Self {
        self.rate = rate;
        self
    }

    pub fn duration(mut self, duration: u8) -> Self {
        self.duration = duration;
        self
    }

    pub fn workers(mut self, workers: u8) -> Self {
        self.workers = workers;
        self
    }

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

                        // Use aggregator executor, with the -o (--output) option
                        let agg = AggregatorConsole::aggregate(
                            tasks
                                .into_iter()
                                .map(|task| AggregateElement {
                                    success: task.success,
                                    status_code: task.status_code,
                                })
                                .collect::<Vec<AggregateElement>>(),
                        );

                        AggregatorConsole::out(&agg);
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
