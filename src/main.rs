use clap::{arg, command};
use futures::{
    future::BoxFuture,
    task::{waker_ref, ArcWake},
};
use std::{
    future::Future,
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc, Mutex,
    },
    task::Context,
    thread::sleep,
    time::Duration,
};
use surf::{Error, Response};

type RequestFuture = Result<Response, Error>;

struct HttpTask {
    future: Mutex<Option<BoxFuture<'static, RequestFuture>>>,
    notify: SyncSender<Arc<HttpTask>>,
}

impl HttpTask {
    pub fn from_future(
        future: impl Future<Output = RequestFuture> + 'static + Send,
        notify: SyncSender<Arc<HttpTask>>,
    ) -> Self {
        Self {
            future: Mutex::new(Some(Box::pin(future))),
            notify,
        }
    }
}

impl ArcWake for HttpTask {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self.notify.send(cloned);
    }
}

struct BatchHttpExecutor {
    batch_config: BatchHttpConfig,
    receiver: Receiver<Arc<HttpTask>>,
    sender: SyncSender<Arc<HttpTask>>,
}

impl BatchHttpExecutor {
    pub fn new(url: &String, number_of_requests: &u8) -> Self {
        let (sender, receiver) = sync_channel::<Arc<HttpTask>>(8);

        Self {
            batch_config: BatchHttpConfig {
                url: url.to_string(),
                number_of_requests: *number_of_requests,
            },
            receiver,
            sender,
        }
    }
    
    /**
     * Create tasks based on config batch
     * Send those tasks thorugh channel
     * Drop channel sender
     */
    pub fn spawn(self) -> Self {
        let mut tasks = (0..self.batch_config.number_of_requests).map(|_| {
            let url = self.batch_config.url.to_owned();
            let future = async { surf::get(url).send().await };
            HttpTask::from_future(future, self.sender.clone())
        });

        while let Some(task) = tasks.next() {
            let safe_task = Arc::new(task);
            self.sender.send(safe_task).ok();
        }

        // Do not expect more messages
        drop(&self.sender);

        // let url = &self.batch_config.url;

        // let future = async {
        //     println!("making request");
        //     surf::get("http://google.com").await
        // };

        // let task = HttpTask {
        //     future: Mutex::new(Some(Box::pin(future))),
        //     notify: self.sender.clone(),
        // };

        // self.sender.send(Arc::new(task)).ok();

        // drop(self.sender);

        // while let Ok(task) = self.receiver.recv() {
        //     let mut locked_future = task.future.lock().unwrap();
        //     if let Some(mut future) = locked_future.take() {
        //         let waker = waker_ref(&task);
        //         let context = &mut Context::from_waker(&*waker);

        //         if future.as_mut().poll(context).is_pending() {
        //             *locked_future = Some(future);
        //         }
        //     }
        // }
        self
    }

    /**
     * Loop over the received tasks through the channel
     * Take the future from the task
     * Poll the future
     * Put it back if it's still pending
     */
    pub fn run(self) -> Self {
        while let Ok(task) = self.receiver.recv() {
            let mut locked_future = task.future.lock().unwrap();
            if let Some(mut future) = locked_future.take() {
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);

                if future.as_mut().poll(context).is_pending() {
                    *locked_future = Some(future);
                }
            }
        }

        self
    }
}

struct BatchHttpConfig {
    url: String,
    number_of_requests: u8,
}

struct LoadTestBuilder {
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

    pub fn execute(self) -> Self {
        if self.duration > 0 && self.rate > 0 {
            let mut sec_spent = 0;

            loop {
                if sec_spent <= self.duration {
                    BatchHttpExecutor::new(&self.url, &self.rate).spawn();
                    // BatchHttpExecutor::new(&self.url, &self.rate).spawn().run();

                    sleep(Duration::from_secs(1));
                    sec_spent += 1;
                    continue;
                }
                break;
            }
        }

        self
    }
}

fn main() {
    let matches = command!()
        .arg(arg!(-u --url <URL> "define load test URL").required(true))
        .arg(arg!(-r --rate <RATE> "requests per second").default_value("5"))
        .arg(arg!(-d --duration <DURATION> "load test duration in seconds").default_value("10"))
        .get_matches();

    let mut builder = LoadTestBuilder::new();

    if let Some(url) = matches.get_one::<String>("url") {
        builder.url(url.to_owned());
    }

    if let Some(rate) = matches.get_one::<String>("rate") {
        builder.rate(rate.parse::<u8>().unwrap());
    }

    if let Some(duration) = matches.get_one::<String>("duration") {
        builder.duration(duration.parse::<u8>().unwrap());
    }

    builder.execute();
}
