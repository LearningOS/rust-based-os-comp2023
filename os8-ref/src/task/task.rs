//! Types related to task management & Functions for completely changing TCB

use super::id::TaskUserRes;
use super::{kstack_alloc, KernelStack, ProcessControlBlock, TaskContext};
use crate::trap::TrapContext;
use crate::{mm::PhysPageNum, sync::UPSafeCell};
use alloc::sync::{Arc, Weak};
use core::cell::RefMut;

/// Task control block structure
///
/// Directly save the contents that will not change during running
pub struct TaskControlBlock {
    // immutable
    pub process: Weak<ProcessControlBlock>,
    /// Kernel stack corresponding to TID
    pub kernel_stack: KernelStack,
    // mutable
    inner: UPSafeCell<TaskControlBlockInner>,
}

/// Structure containing more process content
///
/// Store the contents that will change during operation
/// and are wrapped by UPSafeCell to provide mutual exclusion
pub struct TaskControlBlockInner {
    /// The physical page number of the frame where the trap context is placed
    pub trap_cx_ppn: PhysPageNum,
    /// Save task context
    pub task_cx: TaskContext,
    /// Maintain the execution status of the current process
    pub task_status: TaskStatus,
    /// It is set when active exit or execution error occurs
    pub exit_code: Option<i32>,
    /// Tid and ustack will be deallocated when this goes None
    pub res: Option<TaskUserRes>,
}

/// Simple access to its internal fields
impl TaskControlBlockInner {
    /*
    pub fn get_task_cx_ptr2(&self) -> *const usize {
        &self.task_cx_ptr as *const usize
    }
    */
    pub fn get_trap_cx(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.get_mut()
    }

    #[allow(unused)]
    fn get_status(&self) -> TaskStatus {
        self.task_status
    }
}

impl TaskControlBlock {
    pub fn new(
        process: Arc<ProcessControlBlock>,
        ustack_base: usize,
        alloc_user_res: bool,
    ) -> Self {
        let res = TaskUserRes::new(Arc::clone(&process), ustack_base, alloc_user_res);
        let trap_cx_ppn = res.trap_cx_ppn();
        let kernel_stack = kstack_alloc();
        let kstack_top = kernel_stack.get_top();
        Self {
            process: Arc::downgrade(&process),
            kernel_stack,
            inner: unsafe {
                UPSafeCell::new(TaskControlBlockInner {
                    res: Some(res),
                    trap_cx_ppn,
                    task_cx: TaskContext::goto_trap_return(kstack_top),
                    task_status: TaskStatus::Ready,
                    exit_code: None,
                })
            },
        }
    }

    /// Get the mutex to get the RefMut TaskControlBlockInner
    pub fn inner_exclusive_access(&self) -> RefMut<'_, TaskControlBlockInner> {
        let inner = self.inner.exclusive_access();
        // if self.process.upgrade().unwrap().pid.0 > 1 {
        //     if let Some(res) = inner.res.as_ref() {
        //         println!("t{}i", res.tid);
        //     }
        // }
        inner
    }

    pub fn get_user_token(&self) -> usize {
        let process = self.process.upgrade().unwrap();
        let inner = process.inner_exclusive_access();
        inner.memory_set.token()
    }

    pub fn create_kthread(f: fn()) -> Self {
        use crate::mm::PhysAddr;
        let process = ProcessControlBlock::kernel_process();
        let process = Arc::downgrade(&process);

        let kernelstack = crate::task::id::KStack::new();
        let kstack_top = kernelstack.top();

        let mut context = TaskContext::zero_init();
        let context_addr = &context as *const TaskContext as usize;
        let pa = PhysAddr::from(context_addr);
        let context_ppn = pa.floor();

        context.ra = f as usize;
        context.sp = kstack_top;

        //println!("context ppn :{:#x?}", context_ppn);

        Self {
            process,
            kernel_stack: KernelStack(kstack_top),
            //kstack,
            inner: unsafe {
                UPSafeCell::new(TaskControlBlockInner {
                    res: None,
                    trap_cx_ppn: context_ppn,
                    task_cx: context,
                    task_status: TaskStatus::Ready,
                    exit_code: None,
                })
            },
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
/// task status: UnInit, Ready, Running, Exited
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Blocking,
}
