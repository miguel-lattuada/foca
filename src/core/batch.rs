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
    task::{Context, Poll},
};
use surf::{Error, Response};

type RequestFuture = Result<Response, Error>;

pub struct HttpTask {
    future: Mutex<Option<BoxFuture<'static, RequestFuture>>>,
    notify: SyncSender<Arc<HttpTask>>,
}

pub struct HttpTaskResult {
    pub success: bool,
    pub status_code: u16,
}

pub struct BatchHttpConfig {
    url: String,
    number_of_requests: u8,
}

pub struct BatchHttpExecutor {
    batch_config: BatchHttpConfig,
    receiver: Receiver<Arc<HttpTask>>,
    sender: SyncSender<Arc<HttpTask>>,
    results: Vec<HttpTask>,
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
        arc_self.notify.send(cloned).ok();
    }
}

impl BatchHttpExecutor {
    pub fn new(url: String, number_of_requests: u8) -> Self {
        let (sender, receiver) = sync_channel::<Arc<HttpTask>>(256);

        Self {
            batch_config: BatchHttpConfig {
                url,
                number_of_requests,
            },
            receiver,
            sender,
            results: vec![],
        }
    }

    /**
     * Create tasks based on config batch
     * Send those tasks thorugh channel
     * Drop channel sender
     */
    pub fn spawn(self) -> Self {
        for _ in 0..self.batch_config.number_of_requests {
            let url = self.batch_config.url.to_owned();

            let future = async {
                let res = surf::get(url).send().await;
                res
            };

            let task = Arc::new(HttpTask::from_future(future, self.sender.clone()));
            self.sender.send(task).ok();
        }

        self
    }

    /**
     * Loop over the received tasks through the channel
     * Take the locked future from the task
     * Poll the future
     * Put it back if it's still pending
     */
    pub fn run(self) -> Vec<HttpTaskResult> {
        // TODO: check how to deal with partially moved values
        // { self.sender; } = drop(self.sender) ?
        {
            self.sender;
        }

        let mut results: Vec<HttpTaskResult> = Vec::new();

        // TODO: please refactor
        while let Ok(task) = self.receiver.recv() {
            let mut locked_future = task.future.lock().unwrap();
            if let Some(mut future) = locked_future.take() {
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);

                let pin = future.as_mut().poll(context);

                if pin.is_pending() {
                    *locked_future = Some(future);
                } else {
                    // TODO: Check how to get future result from Polling it
                    let status_poll = pin.map(|future_result| future_result.unwrap().status());

                    if let Poll::Ready(status) = status_poll {
                        results.push(HttpTaskResult {
                            success: status.is_success(),
                            status_code: status as u16,
                        });
                    }
                }
            }
        }

        results
    }
}
