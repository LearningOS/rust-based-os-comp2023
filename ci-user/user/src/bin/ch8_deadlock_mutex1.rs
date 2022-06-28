#![no_std]
#![no_main]
#![allow(clippy::println_empty_string)]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use user_lib::{enable_deadlock_detect, mutex_blocking_create, mutex_lock, mutex_unlock};

// 理想结果：检测到死锁

#[no_mangle]
pub fn main() -> i32 {
    enable_deadlock_detect(true);
    let mid = mutex_blocking_create() as usize;
    assert_eq!(mutex_lock(mid), 0);
    assert_eq!(mutex_lock(mid), -0xdead);
    mutex_unlock(mid);
    println!("deadlock test mutex 1 OK!");
    0
}
