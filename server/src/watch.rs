use futures::Stream;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::pin::Pin;
use std::sync::{mpsc, Arc, Mutex};
use std::task::{Context, Poll, Waker};

struct FolderWatcherState {
    waker: Option<Waker>,
}

pub struct FolderWatcher {
    rx: mpsc::Receiver<Event>,
    state: Arc<Mutex<FolderWatcherState>>,
    watcher: RecommendedWatcher,
}

impl FolderWatcher {
    pub fn new() -> FolderWatcher {
        let state = Arc::new(Mutex::new(FolderWatcherState { waker: None }));
        let shared_state = state.clone();
        let (tx, rx) = mpsc::channel();
        let watcher = RecommendedWatcher::new_immediate(move |res| match res {
            Ok(event) => {
                let mut state = shared_state.lock().expect("Failed to lock watcher state.");
                if let Some(waker) = state.waker.take() {
                    waker.wake();
                    tx.send(event).expect("Failed to forward event.");
                }
            }
            Err(e) => eprintln!("Watch error: {:?}", e),
        })
        .expect("Failed to create watcher.");
        FolderWatcher { rx, state, watcher }
    }

    pub fn watch(&mut self) {
        self.watcher
            .watch("./app/src", RecursiveMode::Recursive)
            .expect("Failed to watch directory.");
        println!("Watching ./app/src");
    }
}

impl Stream for FolderWatcher {
    type Item = notify::Event;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut state = self.state.lock().expect("Failed to lock watcher state.");
        match self.rx.try_recv() {
            Ok(e) => Poll::Ready(Some(e)),
            Err(_) => {
                state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}
