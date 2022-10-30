use crate::{threading::ThreadPool, batch::BatchHttpExecutor};

pub struct LoadTestBuilder {
    url: String,
    rate: u8,
    duration: u8,
}

impl LoadTestBuilder {
    pub fn new() -> Self {
        Self {
            url: String::from(""),
            rate: 0,
            duration: 0,
        }
    }

    pub fn url(&mut self, url: String) -> &mut Self {
        self.url = url;
        self
    }

    pub fn rate(&mut self, rate: u8) -> &mut Self {
        self.rate = rate;
        self
    }

    pub fn duration(&mut self, duration: u8) -> &mut Self {
        self.duration = duration;
        self
    }

    pub fn execute(&self) {
        if self.duration > 0 && self.rate > 0 {
            let thread_pool = ThreadPool::new(4);
            let mut sec_spent = 0;

            loop {
                if sec_spent < self.duration {
                    // TODO: check how to avoid creating a copy for each thread
                    let rate = self.rate;
                    let url = String::from(&self.url);

                    thread_pool.execute(move || {
                        BatchHttpExecutor::new(url, rate).spawn().run();
                    });

                    sec_spent += 1;
                    continue;
                }
                break;
            }
        }
    }
}