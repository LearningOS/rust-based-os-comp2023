#![no_std]
#![no_main]

extern crate user_lib;

static TESTS: &[&str] = &[
    "ch5_stride0\0",
    "ch5_stride1\0",
    "ch5_stride2\0",
    "ch5_stride3\0",
    "ch5_stride4\0",
    "ch5_stride5\0",
];


use user_lib::{spawn, waitpid, set_priority};

#[no_mangle]
pub fn main() -> i32 {
    let mut pid = [0; 6];
    let mut i = 0;
    for test in TESTS {
        pid[i] = spawn(*test);
        i += 1;
    }
    set_priority(4);
    for i in 0..6{
        let mut xstate: i32 = Default::default();
        let wait_pid = waitpid(pid[i] as usize, &mut xstate);
        assert_eq!(pid[i], wait_pid);
    }
    0
}
