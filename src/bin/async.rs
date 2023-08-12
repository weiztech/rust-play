use tokio::time::Duration;
use tokio::time::sleep;
use tokio::macros::support::Poll;
use std::{
    sync::{Arc, Mutex},
    task::{Context, Waker},
    thread,
};
use tokio::macros::support::Pin;
use core::future::Future;
use tokio::task;


#[derive(Debug)]
pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

#[derive(Debug)]
struct SharedLock {
    is_lock: bool,
}

#[derive(Debug)]
struct AsyncData {
    id: u32,
    name: &'static str,
    state: Arc<Mutex<SharedLock>>,
}

impl Future for AsyncData {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        //println!("Running AsyncData {:?}", self);
        let mut state = self.state.lock().unwrap();
        // println!("Block mutex lock call {:?}", self);
        if !state.is_lock{
            state.is_lock = true;
            println!("Succeed lock AsyncData {:?} {}", self, state.is_lock);
            return return Poll::Ready(())
        }
        // println!("Re-pool AsyncData {:?} {}", self, state.is_lock);
        cx.waker().clone().wake();
        Poll::Pending
    }
}

/// Shared state between the future and the waiting thread
#[derive(Debug)]
struct SharedState {
    /// Whether or not the sleep time has elapsed
    completed: bool,

    /// The waker for the task that `TimerFuture` is running on.
    /// The thread can use this after setting `completed = true` to tell
    /// `TimerFuture`'s task to wake up, see that `completed = true`, and
    /// move forward.
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Running");
        // Look at the shared state to see if the timer has already completed.
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            println!("Done");
            Poll::Ready(())
        } else {
            // Set waker so that the thread can wake up the current task
            // when the timer has completed, ensuring that the future is polled
            // again and sees that `completed = true`.
            //
            // It's tempting to do this once rather than repeatedly cloning
            // the waker each time. However, the `TimerFuture` can move between
            // tasks on the executor, which could cause a stale waker pointing
            // to the wrong task, preventing `TimerFuture` from waking up
            // correctly.
            //
            // N.B. it's possible to check for this using the `Waker::will_wake`
            // function, but we omit that here to keep things simple.
            shared_state.waker = Some(cx.waker().clone());
            println!("Not complete set waker");
            Poll::Pending
        }
    }
}

impl TimerFuture {
    /// Create a new `TimerFuture` which will complete after the provided
    /// timeout.
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));
        println!("New 1");
        // Spawn the new thread
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            println!("Start Thread");
            let mut shared_state = thread_shared_state.lock().unwrap();
            thread::sleep(duration);
            println!("Start Thread Sleeping Done");
            // let mut shared_state = thread_shared_state.lock().unwrap();
            // Signal that the timer has completed and wake up the last
            // task on which the future was polled, if one exists.
            shared_state.completed = true;
            println!("Thread 2");
            if let Some(waker) = shared_state.waker.take() {
                println!("Set Wake");
                waker.wake()
            }
        });

        println!("New 2");

        TimerFuture { shared_state }
    }
}


struct  MyCounterFuture {
    cnt : u32,
    cnt_final : u32
}

impl MyCounterFuture {
    pub fn new(final_value : u32) -> Self {
        Self {
            cnt : 0,
            cnt_final : final_value
        }
    }
}

impl Future for MyCounterFuture {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32>{
        self.cnt += 1;
        if self.cnt >= self.cnt_final {
            println!("Counting finished");
            return Poll::Ready(self.cnt_final);
        }

        println!("Counting {}", self.cnt);
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}


async fn test() -> u8{
    println!("testing 123");
    1
}

fn bar() -> impl Future<Output = u8> {
    // This `async` block results in a type that implements
    // `Future<Output = u8>`.
    println!("Bar init");
    async {
        let x: u8 = test().await;
        println!("Bar Proceed");
        x + 5
    }
}


async fn proceed_data(data: AsyncData){
    println!("Proceed {:?}", data);
    let id = data.id;
    let state = data.state.clone();
    data.await;
    sleep(Duration::from_secs(2)).await;
    let mut state = state.lock().unwrap();
    state.is_lock = false;
    println!("Open Lock {id}");
}


#[tokio::main]
async fn main(){

    let test_bar = bar().await;
    let testing = test().await;
    println!("Start! {}", testing);
    sleep(Duration::from_secs(1)).await;
    println!("End after 1 second");

    let my_counter = MyCounterFuture::new(42);
    let final_value = my_counter.await;
    println!("Final value: {}", final_value);

    let timer = TimerFuture::new(Duration::from_secs(5));
    println!("Calling await {:?}", timer);
    let timer_value = timer.await;

    let lock = Arc::new(Mutex::new(SharedLock{is_lock: false}));
    let async_datas = [
        AsyncData{id: 1, name: "first", state: lock.clone()},
        AsyncData{id: 2, name: "two", state: lock.clone()},
        AsyncData{id: 3, name: "three", state: lock.clone()},
        AsyncData{id: 4, name: "four", state: lock.clone()},
        AsyncData{id: 5, name: "five", state: lock.clone()},
    ];

    for async_data in async_datas{
        task::spawn(async move {proceed_data(async_data).await});
    }

    sleep(Duration::from_secs(12)).await;
}
