use futures::Stream;
use notify::Watcher;
use std::sync::{mpsc, Arc, Mutex};

struct FolderWatcherState {
    waker: Option<std::task::Waker>,
}

pub struct FolderWatcher {
    rx: mpsc::Receiver<notify::Event>,
    state: Arc<Mutex<FolderWatcherState>>,
    watcher: notify::FsEventWatcher,
}

impl FolderWatcher {
    pub fn new() -> FolderWatcher {
        let state = Arc::new(Mutex::new(FolderWatcherState { waker: None }));
        let shared_state = state.clone();
        let (tx, rx) = mpsc::channel();
        let watcher = notify::RecommendedWatcher::new_immediate(move |res| match res {
            Ok(event) => {
                let mut state = shared_state.lock().unwrap();
                if let Some(waker) = state.waker.take() {
                    waker.wake();
                }
                tx.send(event).unwrap();
            }
            Err(e) => println!("watch error: {:?}", e),
        })
        .expect("Failed to create watcher.");
        FolderWatcher { rx, state, watcher }
    }

    pub fn watch(&mut self) {
        self.watcher
            .watch("./app/src", notify::RecursiveMode::Recursive)
            .expect("Failed to watch directory.");
        println!("Watching ./app/src");
    }
}

impl Stream for FolderWatcher {
    type Item = notify::Event;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let mut state = self.state.lock().unwrap();
        match self.rx.try_recv() {
            Ok(e) => std::task::Poll::Ready(Some(e)),
            Err(_) => {
                state.waker = Some(cx.waker().clone());
                std::task::Poll::Pending
            }
        }
    }
}
