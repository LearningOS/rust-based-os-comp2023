#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

const TESTS: &[&str] = &[
    "ch2b_hello_world\0",
    "ch2b_power_3\0",
    "ch2b_power_5\0",
    "ch2b_power_7\0",
    "ch3b_sleep\0",
    "ch3b_sleep1\0",
    "ch3b_yield0\0",
    "ch3b_yield1\0",
    "ch3b_yield2\0",
    "ch5b_forktest2\0",
    "ch6b_filetest_simple\0",
    "ch7b_pipetest\0",
    "ch8_deadlock_mutex1\0",
    "ch8_deadlock_sem1\0",
    "ch8_deadlock_sem2\0",
    "ch8b_mpsc_sem\0",
    "ch8b_phil_din_mutex\0",
    "ch8b_race_adder_mutex_spin\0",
    "ch8b_sync_sem\0",
    "ch8b_test_condvar\0",
    "ch8b_threads\0",
    "ch8b_threads_arg\0",
];

const TEST_NUM: usize = TESTS.len();

use user_lib::{exec, fork, waitpid};

#[no_mangle]
pub fn main() -> i32 {
    let mut pids = [0; TEST_NUM];
    for (i, &test) in TESTS.iter().enumerate() {
        println!("Usertests: Running {}", test);
        let pid = fork();
        if pid == 0 {
            exec(&*test, &[core::ptr::null::<u8>()]);
            panic!("unreachable!");
        } else {
            pids[i] = pid;
        }
    }
    let mut xstate: i32 = Default::default();
    for (i, &test) in TESTS.iter().enumerate() {
        let wait_pid = waitpid(pids[i] as usize, &mut xstate);
        assert_eq!(pids[i], wait_pid);
        println!(
            "\x1b[32mUsertests: Test {} in Process {} exited with code {}\x1b[0m",
            test, pids[i], xstate
        );
    }
    println!("ch8 Usertests passed!");
    0
}
