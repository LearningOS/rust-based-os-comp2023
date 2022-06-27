//! Implementation of process management mechanism
//!
//! Here is the entry for process scheduling required by other modules
//! (such as syscall or clock interrupt).
//! By suspending or exiting the current process, you can
//! modify the process state, manage the process queue through TASK_MANAGER,
//! and switch the control flow through PROCESSOR.
//!
//! Be careful when you see [`__switch`]. Control flow around this function
//! might not be what you expect.

mod context;
mod id;
pub mod kthread;
mod manager;
mod process;
mod processor;
pub mod stackless_coroutine;
mod switch;
#[allow(clippy::module_inception)]
mod task;

pub use crate::syscall::process::TaskInfo;
use crate::{
    fs::{open_file, OpenFlags},
    task::id::TaskUserRes,
};
use alloc::{sync::Arc, vec::Vec};
pub use context::TaskContext;
pub use id::{kstack_alloc, pid_alloc, KernelStack, PidHandle};
pub use kthread::kernel_stackful_coroutine_test;
use lazy_static::*;
pub use manager::add_task;
use manager::fetch_task;
use process::ProcessControlBlock;
pub use processor::{
    current_process, current_task, current_trap_cx, current_trap_cx_user_va, current_user_token,
    run_tasks, schedule, take_current_task,
};
pub use stackless_coroutine::kernel_stackless_coroutine_test;
use switch::__switch;
pub use task::{TaskControlBlock, TaskStatus};

pub fn block_current_and_run_next() {
    let task = take_current_task().unwrap();
    let mut task_inner = task.inner_exclusive_access();
    let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
    task_inner.task_status = TaskStatus::Blocking;
    drop(task_inner);
    schedule(task_cx_ptr);
}

/// Make current task suspended and switch to the next task
pub fn suspend_current_and_run_next() {
    // There must be an application running.
    let task = take_current_task().unwrap();

    // ---- access current TCB exclusively
    let mut task_inner = task.inner_exclusive_access();

    let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
    // Change status to Ready
    task_inner.task_status = TaskStatus::Ready;
    drop(task_inner);
    // ---- release current PCB

    // push back to ready queue.
    add_task(task);
    // jump to scheduling cycle
    schedule(task_cx_ptr);
}

/// Exit current task, recycle process resources and switch to the next task
pub fn exit_current_and_run_next(exit_code: i32) {
    // take from Processor
    let task = take_current_task().unwrap();
    // **** access current TCB exclusively
    let mut task_inner = task.inner_exclusive_access();
    let process = task.process.upgrade().unwrap();
    let tid = task_inner.res.as_ref().unwrap().tid;
    // Record exit code
    task_inner.exit_code = Some(exit_code);
    task_inner.res = None;

    // here we do not remove the thread since we are still using the kstack
    // it will be deallocated when sys_waittid is called
    drop(task_inner);
    drop(task);
    // debug!("task {} dropped", tid);

    if tid == 0 {
        let mut process_inner = process.inner_exclusive_access();
        // mark this process as a zombie process
        process_inner.is_zombie = true;
        // record exit code of main process
        process_inner.exit_code = exit_code;

        // do not move to its parent but under initproc
        // debug!("reparent");

        // ++++++ access initproc PCB exclusively
        {
            let mut initproc_inner = INITPROC.inner_exclusive_access();
            for child in process_inner.children.iter() {
                child.inner_exclusive_access().parent = Some(Arc::downgrade(&INITPROC));
                initproc_inner.children.push(child.clone());
            }
        }
        let mut recycle_res = Vec::<TaskUserRes>::new();

        // debug!("deallocate user res");
        // deallocate user res (including tid/trap_cx/ustack) of all threads
        // it has to be done before we dealloc the whole memory_set
        // otherwise they will be deallocated twice
        for task in process_inner.tasks.iter().filter(|t| t.is_some()) {
            let task = task.as_ref().unwrap();
            let mut task_inner = task.inner_exclusive_access();
            if let Some(res) = task_inner.res.take() {
                recycle_res.push(res);
            }
        }
        drop(process_inner);
        recycle_res.clear();
        let mut process_inner = process.inner_exclusive_access();
        // debug!("deallocate pcb res");
        process_inner.children.clear();
        // deallocate other data in user space i.e. program code/data section
        process_inner.memory_set.recycle_data_pages();
        // drop file descriptors
        process_inner.fd_table.clear();
    }
    // debug!("pcb dropped");

    // ++++++ release parent PCB
    drop(process);

    // we do not have to save task context
    let mut _unused = TaskContext::zero_init();
    schedule(&mut _unused as *mut _);
}

lazy_static! {
    /// Creation of initial process
    ///
    /// the name "initproc" may be changed to any other app name like "usertests",
    /// but we have user_shell, so we don't need to change it.
    pub static ref INITPROC: Arc<ProcessControlBlock> = {
        let inode = open_file("ch8b_initproc", OpenFlags::RDONLY).unwrap();
        let v = inode.read_all();
        ProcessControlBlock::new(v.as_slice())
    };
}

pub fn add_initproc() {
    // INITPROC must be referenced at least once so that it can be initialized
    // through lazy_static
    let _initproc = INITPROC.clone();
}
