chapter3练习
=======================================

Lab1 编程作业
--------------------------------------

获取任务信息
++++++++++++++++++++++++++

ch3 中，我们的系统已经能够支持多个任务分时轮流运行，我们希望引入一个新的系统调用 ``sys_task_info`` 以获取当前任务的信息，定义如下：

.. code-block:: rust

    fn sys_task_info(ti: *mut TaskInfo) -> isize

- syscall ID: 410
- 查询当前正在执行的任务信息，任务信息包括任务控制块相关信息（任务状态）、任务使用的系统调用及调用次数、任务总运行时长（单位ms）。

.. code-block:: rust

    struct TaskInfo {
        status: TaskStatus,
        syscall_times: [u32; MAX_SYSCALL_NUM],
        time: usize
    }

- 参数：
    - ti: 待查询任务信息
- 返回值：执行成功返回0，错误返回-1
- 说明：
    - 相关结构已在框架中给出，只需添加逻辑实现功能需求即可。
    - 在我们的实验中，系统调用号一定小于 500，所以直接使用一个长为 ``MAX_SYSCALL_NUM=500`` 的数组做桶计数。
    - 运行时间 time 返回系统调用时刻距离任务第一次被调度时刻的时长，也就是说这个时长可能包含该任务被其他任务抢占后的等待重新调度的时间。
    - 由于查询的是当前任务的状态，因此 TaskStatus 一定是 Running。（助教起初想设计根据任务 id 查询，但是既不好定义任务 id 也不好写测例，遂放弃 QAQ）
    - 调用 ``sys_task_info`` 也会对本次调用计数。
- 提示：
    - 大胆修改已有框架！除了配置文件，你几乎可以随意修改已有框架的内容。
    - 程序运行时间可以通过调用 ``get_time()`` 获取，注意任务运行总时长的单位是 ms。
    - 系统调用次数可以考虑在进入内核态系统调用异常处理函数之后，进入具体系统调用函数之前维护。
    - 阅读 TaskManager 的实现，思考如何维护内核控制块信息（可以在控制块可变部分加入需要的信息）。


实验要求
+++++++++++++++++++++++++++++++++++++++++

- `lab1(os3)参考框架: <https://github.com/LearningOS/rust-based-os-comp2022/tree/main/os3-ref>`_

- 实验目录要求

.. code-block::

   ├── os3(内核实现)
   │   ├── Cargo.toml(配置文件)
   │   └── src(所有内核的源代码放在 os/src 目录下)
   │       ├── main.rs(内核主函数)
   │       └── ...
   ├── reports (不是 report)
   │   ├── lab1.md/pdf
   │   └── ...
   ├── ...


- 通过所有测例：

   CI 使用的测例与本地相同，测试中，user 文件夹及其它与构建相关的文件将被替换，请不要试图依靠硬编码通过测试。

   在 ``os3`` 目录下，默认情况下，makefile 仅编译基础测例 (``BASE=1``)，即无需修改框架即可正常运行的测例。
   你需要在编译时指定 ``BASE=0`` 控制框架仅编译实验测例（在 os 目录执行 ``make run BASE=0``），
   或指定 ``BASE=2`` 控制框架同时编译基础测例和实验测例。

.. note::

    你的实现只需且必须通过测例，建议读者感到困惑时先检查测例。


简答作业
--------------------------------------------

1. 正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。
   请同学们可以自行测试这些内容 (运行 `Rust 三个 bad 测例 (ch2b_bad_*.rs) <https://github.com/LearningOS/rust-based-os-comp2022/tree/main/user/src/bin>`_ ，
   注意在编译时至少需要指定 ``LOG=ERROR`` 才能观察到内核的报错信息) ，
   描述程序出错行为，同时注意注明你使用的 sbi 及其版本。

2. 深入理解 `trap.S <https://github.com/LearningOS/rust-based-os-comp2022/blob/main/os3-ref/src/trap/trap.S>`_
   中两个函数 ``__alltraps`` 和 ``__restore`` 的作用，并回答如下问题:

   1. L40：刚进入 ``__restore`` 时，``a0`` 代表了什么值。请指出 ``__restore`` 的两种使用情景。

   2. L46-L51：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。

      .. code-block:: riscv

         ld t0, 32*8(sp)
         ld t1, 33*8(sp)
         ld t2, 2*8(sp)
         csrw sstatus, t0
         csrw sepc, t1
         csrw sscratch, t2

   3. L53-L59：为何跳过了 ``x2`` 和 ``x4``？

      .. code-block:: riscv

         ld x1, 1*8(sp)
         ld x3, 3*8(sp)
         .set n, 5
         .rept 27
            LOAD_GP %n
            .set n, n+1
         .endr

   4. L63：该指令之后，``sp`` 和 ``sscratch`` 中的值分别有什么意义？

      .. code-block:: riscv

         csrrw sp, sscratch, sp

   5. ``__restore``：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？

   6. L13：该指令之后，``sp`` 和 ``sscratch`` 中的值分别有什么意义？

      .. code-block:: riscv

         csrrw sp, sscratch, sp

   7. 从 U 态进入 S 态是哪一条指令发生的？

报告要求
-------------------------------

- 简单总结你实现的功能（200字以内，不要贴代码）。
- 完成问答题。
- (optional) 你对本次实验设计及难度/工作量的看法，以及有哪些需要改进的地方，欢迎畅所欲言。
