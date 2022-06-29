引言
========================================

本章导读
--------------------------


本章的目标是实现分时多任务系统，它能并发地执行多个用户程序，并调度这些程序。为此需要实现

- 一次性加载所有用户程序，减少任务切换开销；
- 支持任务切换机制，保存切换前后程序上下文；
- 支持程序主动放弃处理器，实现 yield 系统调用；
- 以时间片轮转算法调度用户程序，实现资源的时分复用。


实践体验
-------------------------------------

.. note::

   基于github classroom的开发方式
   
   基于github classroom，可方便建立开发用的git repository，并可基于github的 codespace （在线版ubuntu +vscode）在线开发使用。整个开发环境仅仅需要一个网络浏览器。

   1. 在网络浏览器中用自己的 github id 登录 github.com
   2. 接收 `第一个实验(os3)的github classroom在线邀请 <https://classroom.github.com/a/s1v7GyJM>`_  ，根据提示一路选择OK即可。
   3. 完成第二步后，你的第一个实验的 github repository 会被自动建立好，点击此github repository的链接，就可看到你要完成的第一个实验了。
   4. 在你的第一个实验的网页的中上部可以看到一个醒目的 `code`  绿色按钮，点击后，可以进一步看到  `codespace` 标签和醒目的 `create codesapce on main` 绿色按钮。请点击这个绿色按钮，就可以进入到在线的ubuntu +vscode环境中
   5. 再按照下面的环境安装提示在vscode的 `console` 中安装配置开发环境：rustc，qemu等工具。注：也可在vscode的 `console` 中执行 ``make codespaces_setenv`` 来自动安装配置开发环境（执行``sudo``需要root权限，仅需要执行一次）。
   6. 在vscode的 `console` 中执行 `make setupclassroom_test3`  （该命令仅执行一次）配置githubclassroom 自动评分功能。
   7. 然后就可以基于在线vscode进行开发、运行、提交等完整的实验过程了。

   上述的3，4，5步不是必须的，你也可以线下本地开发。

   注：如果是本地的ubuntu中建立开发环境，可在shell中执行 ``make ubuntu_local_setenv`` 来自动安装配置开发环境（执行``sudo``需要root权限，仅需要执行一次）。

.. code-block:: console

   $ git clone https://github.com/LearningOS/rust-based-os-comp2022.git
   $ cd rust-based-os-comp2022/
   $ make setupclassroom_test3  //注意：这一步很重要，是用于github classroom自动评测你的工作。这一步只需在首次克隆项目仓库时执行一次，以后一般就不用执行了，除非 .github/workflows/classroom.yml发生了变化。

在 qemu 模拟器上运行 `lab1(os3)参考框架: <https://github.com/LearningOS/rust-based-os-comp2022/tree/main/os3-ref>`_ ：

.. code-block:: console

   $ cd os3-ref
   $ make run

运行代码，看到用户程序交替输出信息：

.. code-block::

   [rustsbi] RustSBI version 0.2.0-alpha.4
   .______       __    __      _______.___________.  _______..______   __
   |   _  \     |  |  |  |    /       |           | /       ||   _  \ |  |
   |  |_)  |    |  |  |  |   |   (----`---|  |----`|   (----`|  |_)  ||  |
   |      /     |  |  |  |    \   \       |  |      \   \    |   _  < |  |
   |  |\  \----.|  `--'  |.----)   |      |  |  .----)   |   |  |_)  ||  |
   | _| `._____| \______/ |_______/       |__|  |_______/    |______/ |__|

   [rustsbi] Implementation: RustSBI-QEMU Version 0.0.1
   [rustsbi-dtb] Hart count: cluster0 with 1 cores
   [rustsbi] misa: RV64ACDFIMSU
   [rustsbi] mideleg: ssoft, stimer, sext (0x222)
   [rustsbi] medeleg: ima, ia, bkpt, la, sa, uecall, ipage, lpage, spage (0xb1ab)
   [rustsbi] pmp0: 0x80000000 ..= 0x800fffff (rwx)
   [rustsbi] pmp1: 0x80000000 ..= 0x807fffff (rwx)
   [rustsbi] pmp2: 0x0 ..= 0xffffffffffffff (---)
   [rustsbi] enter supervisor 0x80200000
   [kernel] Hello, world!
   power_3 [10000/200000]
   power_3 [20000/200000]
   power_3 [30000/200000]
   power_3 [40000/200000]
   power_3 [50000/200000]
   power_3 [60000/200000]
   power_3 [70000/200000]
   power_3 [80000/200000]
   power_3 [90000/200000]
   power_3 [100000/200000]
   power_3 [110000/200000]
   power_3 [120000/200000]
   power_3 [130000/200000]
   power_3 [140000/200000]
   power_3 [150000/200000]
   power_3 [160000/200000]
   power_3 [170000/200000]
   power_3 [180000/200000]
   power_3 [190000/200000]
   power_3 [200000/200000]
   3^200000 = 871008973(MOD 998244353)
   Test power_3 OK!
   power_5 [10000/140000]
   power_5 [20000/140000]
   power_5 [30000/140000]
   power_5 [40000/140000]
   power_5 [50000/140000]
   power_5 [60000/140000]
   power_7 [10000/160000]
   power_7 [20000/160000]
   power_7 [30000/160000]
   power_7 [40000/160000]
   power_7 [50000/160000]
   power_7 [60000/160000]
   power_7 [70000/160000]
   power_7 [80000/160000]
   power_7 [90000/160000]
   power_7 [100000/160000]
   power_7 [110000/160000]
   power_7 [120000/160000]
   power_7 [130000/160000]
   power_7 [140000/160000]
   power_7 [150000/160000]
   power_7 [160000/160000]
   7^160000 = 667897727(MOD 998244353)
   Test power_7 OK!
   get_time OK! 42
   current time_msec = 42
   AAAAAAAAAA [1/5]
   BBBBBBBBBB [1/5]
   CCCCCCCCCC [1/5]
   power_5 [70000/140000]
   AAAAAAAAAA [2/5]
   BBBBBBBBBB [2/5]
   CCCCCCCCCC [2/5]
   power_5 [80000/140000]
   power_5 [90000/140000]
   power_5 [100000/140000]
   power_5 [110000/140000]
   power_5 [120000/140000]
   power_5 [130000/140000]
   power_5 [140000/140000]
   5^140000 = 386471875(MOD 998244353)
   Test power_5 OK!
   AAAAAAAAAA [3/5]
   BBBBBBBBBB [3/5]
   CCCCCCCCCC [3/5]
   AAAAAAAAAA [4/5]
   BBBBBBBBBB [4/5]
   CCCCCCCCCC [4/5]
   AAAAAAAAAA [5/5]
   BBBBBBBBBB [5/5]
   CCCCCCCCCC [5/5]
   Test write A OK!
   Test write B OK!
   Test write C OK!
   time_msec = 143 after sleeping 100 ticks, delta = 101ms!
   Test sleep1 passed!
   Test sleep OK!
   Panicked at src/task/mod.rs:98 All applications completed!


`lab1(os3)参考框架: <https://github.com/LearningOS/rust-based-os-comp2022/tree/main/os3-ref>`_
--------------------------------------------------------------------------------------------------------------------

.. code-block::

   ── os3-ref
      ├── build.rs
      ├── Cargo.toml
      ├── Makefile
      └── src
          ├── batch.rs(移除：功能分别拆分到 loader 和 task 两个子模块)
          ├── config.rs(新增：保存内核的一些配置)
          ├── console.rs
          ├── logging.rs
          ├── sync
          ├── entry.asm
          ├── lang_items.rs
          ├── link_app.S
          ├── linker.ld
          ├── loader.rs(新增：将应用加载到内存并进行管理)
          ├── main.rs(修改：主函数进行了修改)
          ├── sbi.rs(修改：引入新的 sbi call set_timer)
          ├── syscall(修改：新增若干 syscall)
          │   ├── fs.rs
          │   ├── mod.rs
          │   └── process.rs
          ├── task(新增：task 子模块，主要负责任务管理)
          │   ├── context.rs(引入 Task 上下文 TaskContext)
          │   ├── mod.rs(全局任务管理器和提供给其他模块的接口)
          │   ├── switch.rs(将任务切换的汇编代码解释为 Rust 接口 __switch)
          │   ├── switch.S(任务切换的汇编代码)
          │   └── task.rs(任务控制块 TaskControlBlock 和任务状态 TaskStatus 的定义)
          ├── timer.rs(新增：计时器相关)
          └── trap
              ├── context.rs
              ├── mod.rs(修改：时钟中断相应处理)
              └── trap.S

   cloc os
   -------------------------------------------------------------------------------
   Language                     files          blank        comment           code
   -------------------------------------------------------------------------------
   Rust                            21             87             20            627
   Assembly                         4             12             22            144
   make                             1             11              4             36
   TOML                             1              2              1             10
   -------------------------------------------------------------------------------
   SUM:                            27            112             47            817
   -------------------------------------------------------------------------------


.. 本章代码导读
.. -----------------------------------------------------

.. 本章的重点是实现对应用之间的协作式和抢占式任务切换的操作系统支持。与上一章的操作系统实现相比，有如下一些不同的情况导致实现上也有差异：

.. - 多个应用同时放在内存中，所以他们的起始地址是不同的，且地址范围不能重叠
.. - 应用在整个执行过程中会暂停或被抢占，即会有主动或被动的任务切换

.. 这些实现上差异主要集中在对应用程序执行过程的管理、支持应用程序暂停的系统调用和主动切换应用程序所需的时钟中断机制的管理。

.. 对于第一个不同情况，需要对应用程序的地址空间布局进行调整，每个应用的地址空间都不相同，且不能重叠。这并不要修改应用程序本身，而是通过一个脚本 ``build.py`` 来针对每个应用程序修改链接脚本 ``linker.ld`` 中的 ``BASE_ADDRESS`` ，让编译器在编译不同应用时用到的 ``BASE_ADDRESS`` 都不同，且有足够大的地址间隔。这样就可以让每个应用所在的内存空间是不同的。

.. 对于第二个不同情况，需要实现任务切换，这就需要在上一章的 ``trap`` 上下文切换的基础上，再加上一个 ``task`` 上下文切换，才能完成完整的任务切换。这里面的关键数据结构是表示应用执行上下文的 ``TaskContext`` 数据结构和具体完成上下文切换的汇编语言编写的 ``__switch`` 函数。一个应用的执行需要被操作系统管理起来，这是通过 ``TaskControlBlock`` 数据结构来表示应用执行上下文的动态过程和动态状态（运行态、就绪态等）。而为了做好应用程序第一次执行的前期初始化准备， ``TaskManager`` 数据结构的全局变量实例 ``TASK_MANAGER`` 描述了应用程序初始化所需的数据， 而 ``TASK_MANAGER`` 的初始化赋值过程是实现这个准备的关键步骤。

.. 应用程序可以在用户态执行后，还需要有新的系统调用 ``sys_yield`` 的实现来支持应用自己的主动暂停；还要添加对时钟中断的处理，来支持抢占应用执行的抢占式切换。有了时钟中断，就可以在一定时间内打断应用的执行，并主动切换到另外一个应用，这部分主要是通过对 ``trap_handler`` 函数中进行扩展，来完成在时钟中断产生时可能进行的任务切换。  ``TaskManager`` 数据结构的成员函数 ``run_next_task`` 来实现基于任务控制块的切换，并会具体调用 ``__switch`` 函数完成硬件相关部分的任务上下文切换。

.. 如果理解了上面的数据结构和相关函数的关系和相互调用的情况，那么就比较容易理解本章改进后的操作系统了。


.. .. [#prionosuchus] 锯齿螈身长可达9米，是迄今出现过的最大的两栖动物，是二叠纪时期江河湖泊和沼泽中的顶级掠食者。
.. .. [#eoraptor] 始初龙（也称始盗龙）是后三叠纪时期的两足食肉动物，也是目前所知最早的恐龙，它们只有一米长，却代表着恐龙的黎明。
.. .. [#coelophysis] 腔骨龙（也称虚形龙）最早出现于三叠纪晚期，它体形纤细，善于奔跑，以小型动物为食。
