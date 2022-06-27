任务切换
================================


本节我们将见识操作系统的核心机制—— **任务切换** ，
即应用在运行中主动或被动地交出 CPU 的使用权，内核可以选择另一个程序继续执行。
内核需要保证用户程序两次运行期间，任务上下文（如寄存器、栈等）保持一致。

任务切换的设计与实现
---------------------------------

任务切换与上一章提及的 Trap 控制流切换相比，有如下异同：

- 与 Trap 切换不同，它不涉及特权级切换，部分由编译器完成；
- 与 Trap 切换相同，它对应用是透明的。

事实上，任务切换是来自两个不同应用在内核中的 Trap 控制流之间的切换。
当一个应用 Trap 到 S 态 OS 内核中进行进一步处理时，
其 Trap 控制流可以调用一个特殊的 ``__switch`` 函数。
在 ``__switch`` 返回之后，Trap 控制流将继续从调用该函数的位置继续向下执行。
而在调用 ``__switch`` 之后到返回前的这段时间里，
原 Trap 控制流 ``A`` 会先被暂停并被切换出去， CPU 转而运行另一个应用的 Trap 控制流 ``B`` 。
``__switch`` 返回之后，原 Trap 控制流 ``A`` 才会从某一条 Trap 控制流 ``C`` 切换回来继续执行。

我们需要在 ``__switch`` 中保存 CPU 的某些寄存器，它们就是 **任务上下文** (Task Context)。

下面我们给出 ``__switch`` 的实现：

.. code-block:: riscv
    :linenos:

    # os/src/task/switch.S

    .altmacro
    .macro SAVE_SN n
        sd s\n, (\n+2)*8(a0)
    .endm
    .macro LOAD_SN n
        ld s\n, (\n+2)*8(a1)
    .endm
        .section .text
        .globl __switch
    __switch:
        # __switch(
        #     current_task_cx_ptr: *mut TaskContext,
        #     next_task_cx_ptr: *const TaskContext
        # )
        # save kernel stack of current task
        sd sp, 8(a0)
        # save ra & s0~s11 of current execution
        sd ra, 0(a0)
        .set n, 0
        .rept 12
            SAVE_SN %n
            .set n, n + 1
        .endr
        # restore ra & s0~s11 of next execution
        ld ra, 0(a1)
        .set n, 0
        .rept 12
            LOAD_SN %n
            .set n, n + 1
        .endr
        # restore kernel stack of next task
        ld sp, 8(a1)
        ret

它的两个参数分别是当前和即将被切换到的 Trap 控制流的 ``task_cx_ptr`` ，从 RISC-V 调用规范可知，它们分别通过寄存器 ``a0/a1`` 传入。

内核先把 ``current_task_cx_ptr`` 中包含的寄存器值逐个保存，再把 ``next_task_cx_ptr`` 中包含的寄存器值逐个恢复。

``TaskContext`` 里包含的寄存器有：

.. code-block:: rust
    :linenos:

    // os/src/task/context.rs
    #[repr(C)]
    pub struct TaskContext {
        ra: usize,
        sp: usize,
        s: [usize; 12],
    }

``s0~s11`` 是被调用者保存寄存器， ``__switch`` 是用汇编编写的，编译器不会帮我们处理这些寄存器。
保存 ``ra`` 很重要，它记录了 ``__switch`` 函数返回之后应该跳转到哪里继续执行。

我们将这段汇编代码 ``__switch`` 解释为一个 Rust 函数：

.. code-block:: rust
    :linenos:

    // os/src/task/switch.rs

    core::arch::global_asm!(include_str!("switch.S"));

    extern "C" {
        pub fn __switch(
            current_task_cx_ptr: *mut TaskContext,
            next_task_cx_ptr: *const TaskContext);
    }

我们会调用该函数来完成切换功能，而不是直接跳转到符号 ``__switch`` 的地址。
因此在调用前后，编译器会帮我们保存和恢复调用者保存寄存器。
