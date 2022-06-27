use super::suspend_current_and_run_next;
use crate::task::{add_task, schedule, TaskContext, TaskControlBlock};
use alloc::sync::Arc;

// NOTE: This module is not required to finish the lab5, though you may run
// kernel_stackless_coroutine_test() in kernel main() to see what happens

#[no_mangle]
pub fn kthread_create(f: fn()) {
    println!("kthread_create");

    // create kernel thread
    let new_tcb = TaskControlBlock::create_kthread(f);
    // let kernel_stack = new_tcb.get_kernel_stack();
    let new_task = Arc::new(new_tcb);

    // add kernel thread into TASK_MANAGER
    // println!("add task");
    add_task(Arc::clone(&new_task));
}

#[no_mangle]
pub fn kernel_stackful_coroutine_test() {
    println!("kernel_stackful_coroutine_test");
    kthread_create(|| {
        let id = 1;
        println!("kernel thread {:?} STARTING", id);
        for i in 0..10 {
            println!("kernel thread: {} counter: {}", id, i);
        }
        println!("kernel thread {:?} FINISHED", id);
        kthread_stop();
    });
    kthread_create(|| {
        let id = 2;
        println!("kernel thread {:?} STARTING", id);
        for i in 0..10 {
            println!("kernel thread: {} counter: {}", id, i);
            kthread_yield();
        }
        println!("kernel thread {:?} FINISHED", id);
        kthread_stop();
    });
    kthread_create(|| {
        let id = 3;
        println!("kernel thread {:?} STARTING", id);
        for i in 0..10 {
            println!("kernel thread: {} counter: {}", id, i);
            kthread_yield();
        }
        println!("kernel thread {:?} FINISHED", id);
        kthread_stop();
    });
}

pub fn kthread_stop() {
    do_exit();
}
#[no_mangle]
pub fn do_exit() {
    println!("kthread do exit");
    exit_kthread_and_run_next(0);
    panic!("Unreachable in sys_exit!");
}

pub fn kthread_yield() {
    suspend_current_and_run_next();
}

#[no_mangle]
pub fn exit_kthread_and_run_next(exit_code: i32) {
    println!("exit_kthread_and_run_next with code: {}", exit_code);
    // we do not have to save task context
    let mut _unused = TaskContext::zero_init();
    schedule(&mut _unused as *mut _);
}
