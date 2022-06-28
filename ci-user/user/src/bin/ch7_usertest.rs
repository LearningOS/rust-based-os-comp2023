#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

static TESTS: &[&str] = &[
];

use user_lib::{spawn, waitpid};

/// 辅助测例，运行所有其他测例。

#[no_mangle]
pub fn main() -> i32 {
    for test in TESTS {
        println!("Usertests: Running {}", test);
        let pid = spawn(*test);
        let mut xstate: i32 = Default::default();
        let wait_pid = waitpid(pid as usize, &mut xstate);
        assert_eq!(pid, wait_pid);
        println!(
            "\x1b[32mUsertests: Test {} in Process {} exited with code {}\x1b[0m",
            test, pid, xstate
        );
    }
    println!("ch7 Usertests passed!");
    0
}
