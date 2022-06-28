#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

static TESTS: &[&str] = &[
    "ch2b_hello_world\0",
    "ch2b_power_3\0",
    "ch2b_power_5\0",
    "ch2b_power_7\0",
    "ch3b_yield0\0",
    "ch3b_yield1\0",
    "ch3b_yield2\0",
    "ch3b_sleep\0",
    "ch3b_sleep1\0",
    "ch4_mmap0\0",
    "ch4_mmap1\0",
    "ch4_mmap2\0",
    "ch4_mmap3\0",
    "ch4_unmap\0",
    "ch4_unmap2\0",
    "ch5_spawn0\0",
    "ch5_spawn1\0",
    "ch5_setprio\0",
    // "ch5_stride\0",
];
static STEST: &str = "ch5_stride\0";

use user_lib::{spawn, waitpid};

/// 辅助测例，运行所有其他测例。

#[no_mangle]
pub fn main() -> i32 {
    let mut pid = [0; 20];
    for (i, &test) in TESTS.iter().enumerate() {
        println!("Usertests: Running {}", test);
        pid[i] = spawn(test);
    }
    let mut xstate: i32 = Default::default();
    for (i, &test) in TESTS.iter().enumerate() {
        let wait_pid = waitpid(pid[i] as usize, &mut xstate);
        assert_eq!(pid[i], wait_pid);
        println!(
            "\x1b[32mUsertests: Test {} in Process {} exited with code {}\x1b[0m",
            test, pid[i], xstate
        );
    }
    println!("Usertests: Running {}", STEST);
    let spid = spawn(STEST);
    xstate = Default::default();
    let wait_pid = waitpid(spid as usize, &mut xstate);
    assert_eq!(spid, wait_pid);
    println!(
        "\x1b[32mUsertests: Test {} in Process {} exited with code {}\x1b[0m",
        STEST, spid, xstate
    );
    println!("ch5 Usertests passed!");
    0
}
