#![no_std]
#![no_main]
#![allow(clippy::println_empty_string)]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use user_lib::{
    enable_deadlock_detect, exit, semaphore_create, semaphore_down, semaphore_up, sleep,
};
use user_lib::{gettid, thread_create, waittid};

// sem 0: used to sync child thread with main
// sem 1-3: representing some kind of resource

// 理想结果：检测到死锁，至少有一个子线程返回值不为 0

const SEM_BARRIER: usize = 0;
const THREAD_N: usize = 3;
const RES_TYPE: usize = 3;
const RES_NUM: [usize; RES_TYPE] = [1, 2, 1];
const REQUEST: [Option<usize>; THREAD_N] = [Some(1), Some(3), Some(2)];

fn try_sem_down(sem_id: usize) {
    if semaphore_down(sem_id) == -0xdead {
        sem_dealloc(gettid() as usize);
        println!("Deadlock detected. Test 08_sem1 failed!");
        exit(-1);
    }
}

fn sem_alloc(tid: usize) {
    match tid {
        1 => assert_eq!(semaphore_down(2), 0),
        2 => {
            assert_eq!(semaphore_down(1), 0);
            assert_eq!(semaphore_down(2), 0);
        }
        3 => assert_eq!(semaphore_down(3), 0),
        _ => exit(1),
    }
    semaphore_down(SEM_BARRIER);
}

fn sem_dealloc(tid: usize) {
    semaphore_up(SEM_BARRIER);
    match tid {
        1 => semaphore_up(2),
        2 => {
            semaphore_up(1);
            semaphore_up(2);
        }
        3 => semaphore_up(3),
        _ => exit(1),
    }
}

fn deadlock_test() {
    let tid = gettid() as usize;
    println!("thread {} running", tid);
    sem_alloc(tid);
    if let Some(sem_id) = REQUEST[tid - 1] {
        try_sem_down(sem_id);
        semaphore_up(sem_id);
    }
    sem_dealloc(tid);
    println!("thread {} exited", tid);
    exit(0);
}

#[no_mangle]
pub fn main() -> i32 {
    enable_deadlock_detect(true);
    assert_eq!(semaphore_create(THREAD_N) as usize, SEM_BARRIER);
    for _ in 0..THREAD_N {
        semaphore_down(SEM_BARRIER);
    }

    for n in RES_NUM {
        semaphore_create(n);
    }
    let mut tids = [0; THREAD_N];

    for i in 0..THREAD_N {
        tids[i] = thread_create(deadlock_test as usize, 0) as usize;
    }

    sleep(500);
    for _ in 0..THREAD_N {
        semaphore_up(SEM_BARRIER);
    }
    let mut failed = 0;
    for tid in tids {
        if waittid(tid) != 0 {
            failed += 1;
        }
    }
    assert!(failed > 0);
    println!("deadlock test semaphore 1 OK!");
    0
}
