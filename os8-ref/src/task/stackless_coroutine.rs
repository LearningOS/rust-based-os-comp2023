// https://blog.aloni.org/posts/a-stack-less-rust-coroutine-100-loc/
// https://github.com/chyyuu/example-coroutine-and-thread/tree/stackless-coroutine-x86

// NOTE: This module is not required to finish the lab5, though you may run
// kernel_stackless_coroutine_test() in kernel main() to see what happens

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use core::task::{RawWaker, RawWakerVTable, Waker};

extern crate alloc;
use alloc::collections::VecDeque;

use alloc::boxed::Box;

enum State {
    Halted,
    Running,
}

struct Task {
    state: State,
}

impl Task {
    fn waiter<'a>(&'a mut self) -> Waiter<'a> {
        Waiter { task: self }
    }
}

struct Waiter<'a> {
    task: &'a mut Task,
}

impl<'a> Future for Waiter<'a> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
        match self.task.state {
            State::Halted => {
                self.task.state = State::Running;
                Poll::Ready(())
            }
            State::Running => {
                self.task.state = State::Halted;
                Poll::Pending
            }
        }
    }
}

struct Executor {
    tasks: VecDeque<Pin<Box<dyn Future<Output = ()>>>>,
}

impl Executor {
    fn new() -> Self {
        Executor {
            tasks: VecDeque::new(),
        }
    }

    fn push<C, F>(&mut self, closure: C)
    where
        F: Future<Output = ()> + 'static,
        C: FnOnce(Task) -> F,
    {
        let task = Task {
            state: State::Running,
        };
        self.tasks.push_back(Box::pin(closure(task)));
    }

    fn run(&mut self) {
        let waker = create_waker();
        let mut context = Context::from_waker(&waker);

        while let Some(mut task) = self.tasks.pop_front() {
            match task.as_mut().poll(&mut context) {
                Poll::Pending => {
                    self.tasks.push_back(task);
                }
                Poll::Ready(()) => {}
            }
        }
    }
}

pub fn create_waker() -> Waker {
    // Safety: The waker points to a vtable with functions that do nothing. Doing
    // nothing is memory-safe.
    unsafe { Waker::from_raw(RAW_WAKER) }
}

const RAW_WAKER: RawWaker = RawWaker::new(core::ptr::null(), &VTABLE);
const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

unsafe fn clone(_: *const ()) -> RawWaker {
    RAW_WAKER
}
unsafe fn wake(_: *const ()) {}
unsafe fn wake_by_ref(_: *const ()) {}
unsafe fn drop(_: *const ()) {}

#[no_mangle]
pub fn kernel_stackless_coroutine_test() {
    println!("kernel stackless coroutine Begin..");
    let mut exec = Executor::new();
    println!(" Create futures");
    for instance in 1..=3 {
        exec.push(move |mut task| async move {
            println!("  Kernel  Task {}: begin state", instance);
            task.waiter().await;
            println!("  Kernel  Task {}: next state", instance);
            task.waiter().await;
            println!("  Kernel  Task {}: end state", instance);
        });
    }

    println!(" Running");
    exec.run();
    println!(" Done");
    println!("kernel stackless coroutine PASSED");
}
