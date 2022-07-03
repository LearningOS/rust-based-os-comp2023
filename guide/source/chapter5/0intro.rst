引言
===========================================

本章导读
-------------------------------------------

.. note::

   基于github classroom的开发方式
   
   基于github classroom，可方便建立开发用的git repository，并可基于github的 codespace （在线版ubuntu +vscode）在线开发使用。整个开发环境仅仅需要一个网络浏览器。

   1. 在网络浏览器中用自己的 github id 登录 github.com
   2. 接收 `第三个实验(os5)的github classroom在线邀请 <https://classroom.github.com/a/RxB6h4-x>`_  ，根据提示一路选择OK即可。
   3. 完成第二步后，你的第三个实验的 github repository 会被自动建立好，点击此github repository的链接，就可看到你要完成的第一个实验了。
   4. 在你的第三个实验的网页的中上部可以看到一个醒目的 `code`  绿色按钮，点击后，可以进一步看到  `codespace` 标签和醒目的 `create codesapce on main` 绿色按钮。请点击这个绿色按钮，就可以进入到在线的ubuntu +vscode环境中
   5. 再按照下面的环境安装提示在vscode的 `console` 中安装配置开发环境：rustc，qemu等工具。注：也可在vscode的 `console` 中执行 ``make codespaces_setenv`` 来自动安装配置开发环境（执行``sudo``需要root权限，仅需要执行一次）。
   6. 在vscode的 `console` 中执行 `make setupclassroom_test5`  （该命令仅执行一次）配置githubclassroom 自动评分功能。
   7. 然后就可以基于在线vscode进行开发、运行、提交等完整的实验过程了。

   上述的3，4，5步不是必须的，你也可以线下本地开发。

   注：如果是本地的ubuntu中建立开发环境，可在shell中执行 ``make ubuntu_local_setenv`` 来自动安装配置开发环境（执行``sudo``需要root权限，仅需要执行一次）。

我们将开发一个用户 **终端** (Terminal) 或 **命令行** (Command Line Application, 俗称 **Shell** ) ，
形成用户与操作系统进行交互的命令行界面 (Command Line Interface)。

为此，我们要对任务建立新的抽象： **进程** ，并实现若干基于 **进程** 的强大系统调用。

.. note::

   **任务和进程的关系与区别**

   第三章提到的 **任务** 是这里提到的 **进程** 的初级阶段，与任务相比，进程能在运行中创建 **子进程** 、
   用新的 **程序** 内容覆盖已有的 **程序** 内容、可管理更多物理或虚拟 **资源** 。

实践体验
-------------------------------------------


在 qemu 模拟器上运行`lab3(os5)参考框架: <https://github.com/LearningOS/rust-based-os-comp2022/tree/main/os5-ref>`_ ：

.. code-block:: console

   $ git clone ``gitaddr of github-classroom-build-lab3``
   $ cd ``github-classroom-build-lab3`` 
   $ make setupclassroom_test5  //注意：这一步很重要，是用于github classroom自动评测你的工作。这一步只需在首次克隆项目仓库时执行一次，以后一般就不用执行了，除非 .github/workflows/classroom.yml发生了变化。   
   $ cd os5-ref
   $ make run


.. note::

   实验名称 ：实验编号 
   
   -  lab0-0 : test1
   -  lab0-1：test2 
   -  lab1：test3
   -  lab2：test4
   -  lab3：test5
   -  lab4：test6
   -  lab5：test8

待内核初始化完毕之后，将在屏幕上打印可用的应用列表并进入shell程序：

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
   /**** APPS ****
   ch2b_bad_address
   ch2b_bad_instructions
   ch2b_bad_register
   ch2b_hello_world
   ch2b_power_3
   ch2b_power_5
   ch2b_power_7
   ch3b_sleep
   ch3b_sleep1
   ch3b_yield0
   ch3b_yield1
   ch3b_yield2
   ch5b_exit
   ch5b_forktest
   ch5b_forktest2
   ch5b_forktest_simple
   ch5b_forktree
   ch5b_initproc
   ch5b_user_shell
   **************/
   Rust user shell
   >>

可以通过输入ch5b开头的应用来测试ch5实现的fork等功能:

.. code-block::

   >> ch5b_forktest_simple

   sys_wait without child process test passed!
   parent start, pid = 2!
   ready waiting on parent process!
   hello child process!
   child process pid = 3, exit code = 100
   Shell: Process 2 exited with code 0

`lab3(os5)参考框架: <https://github.com/LearningOS/rust-based-os-comp2022/tree/main/os5-ref>`_
----------------------------------------------------------------------------------------------------------------------

.. code-block::
   :linenos:

   ├── os5-ref
      ├── build.rs(修改：基于应用名的应用构建器)
      ├── ...
      └── src
          ├── ...
          ├── loader.rs(修改：基于应用名的应用加载器)
          ├── main.rs(修改)
          ├── mm(修改：为了支持本章的系统调用对此模块做若干增强)
          │   ├── address.rs
          │   ├── frame_allocator.rs
          │   ├── heap_allocator.rs
          │   ├── memory_set.rs
          │   ├── mod.rs
          │   └── page_table.rs
          ├── syscall
          │   ├── fs.rs(修改：新增 sys_read)
          │   ├── mod.rs(修改：新的系统调用的分发处理)
          │   └── process.rs（修改：新增 sys_getpid/fork/exec/waitpid）
          ├── task
          │   ├── context.rs
          │   ├── manager.rs(新增：任务管理器，为上一章任务管理器功能的一部分)
          │   ├── mod.rs(修改：调整原来的接口实现以支持进程)
          │   ├── pid.rs(新增：进程标识符和内核栈的 Rust 抽象)
          │   ├── processor.rs(新增：处理器管理结构 ``Processor`` ，为上一章任务管理器功能的一部分)
          │   ├── switch.rs
          │   ├── switch.S
          │   └── task.rs(修改：支持进程机制的任务控制块)
          └── trap
              ├── context.rs
              ├── mod.rs(修改：对于系统调用的实现进行修改以支持进程系统调用)
              └── trap.S

   cloc os
   -------------------------------------------------------------------------------
   Language                     files          blank        comment           code
   -------------------------------------------------------------------------------
   Rust                            29            180            138           2049
   Assembly                         4             20             26            229
   make                             1             11              4             36
   TOML                             1              2              1             13
   -------------------------------------------------------------------------------
   SUM:                            35            213            169           2327
   -------------------------------------------------------------------------------


.. 本章代码导读
.. -----------------------------------------------------

.. 本章的第一小节 :doc:`/chapter5/1process` 介绍了操作系统中经典的进程概念，并描述我们将要实现的参考自 Unix 系内核并经过简化的精简版进程模型。在该模型下，若想对进程进行管理，实现创建、退出等操作，核心就在于 ``fork/exec/waitpid`` 三个系统调用。

.. 首先我们修改运行在应用态的应用软件，它们均放置在 ``user`` 目录下。在新增系统调用的时候，需要在 ``user/src/lib.rs`` 中新增一个 ``sys_*`` 的函数，它的作用是将对应的系统调用按照与内核约定的 ABI 在 ``syscall`` 中转化为一条用于触发系统调用的 ``ecall`` 的指令；还需要在用户库 ``user_lib`` 将 ``sys_*`` 进一步封装成一个应用可以直接调用的与系统调用同名的函数。通过这种方式我们新增三个进程模型中核心的系统调用 ``fork/exec/waitpid`` ，一个查看进程 PID 的系统调用 ``getpid`` ，还有一个允许应用程序获取用户键盘输入的 ``read`` 系统调用。

.. 基于进程模型，我们在 ``user/src/bin`` 目录下重新实现了一组应用程序。其中有两个特殊的应用程序：用户初始程序 ``initproc.rs`` 和 shell 程序 ``user_shell.rs`` ，可以认为它们位于内核和其他应用程序之间的中间层提供一些基础功能，但是它们仍处于应用层。前者会被内核唯一自动加载、也是最早加载并执行，后者则负责从键盘接收用户输入的应用名并执行对应的应用。剩下的应用从不同层面测试了我们内核实现的正确性，读者可以自行参考。值得一提的是， ``usertests`` 可以按照顺序执行绝大部分应用，会在测试的时候为我们提供很多方便。

.. 接下来就需要在内核中实现简化版的进程机制并支持新增的系统调用。在本章第二小节 :doc:`/chapter5/2core-data-structures` 中我们对一些进程机制相关的数据结构进行了重构或者修改：

.. - 为了支持基于应用名而不是应用 ID 来查找应用 ELF 可执行文件，从而实现灵活的应用加载，在 ``os/build.rs`` 以及 ``os/src/loader.rs`` 中更新了 ``link_app.S`` 的格式使得它包含每个应用的名字，另外提供 ``get_app_data_by_name`` 接口获取应用的 ELF 数据。
.. - 在本章之前，任务管理器 ``TaskManager`` 不仅负责管理所有的任务状态，还维护着我们的 CPU 当前正在执行哪个任务。这种设计耦合度较高，我们将后一个功能分离到 ``os/src/task/processor.rs`` 中的处理器管理结构 ``Processor`` 中，它负责管理 CPU 上执行的任务和一些其他信息；而 ``os/src/task/manager.rs`` 中的任务管理器 ``TaskManager`` 仅负责管理所有任务。
.. - 针对新的进程模型，我们复用前面章节的任务控制块 ``TaskControlBlock`` 作为进程控制块来保存进程的一些信息，相比前面章节还要新增 PID、内核栈、应用数据大小、父子进程、退出码等信息。它声明在 ``os/src/task/task.rs`` 中。
.. - 从本章开始，内核栈在内核地址空间中的位置由所在进程的 PID 决定，我们需要在二者之间建立联系并提供一些相应的资源自动回收机制。可以参考 ``os/src/task/pid.rs`` 。

.. 有了这些数据结构的支撑，我们在本章第三小节 :doc:`/chapter5/3implement-process-mechanism` 实现进程机制。它可以分成如下几个方面：

.. - 初始进程的自动创建。在内核初始化的时候需要调用 ``os/src/task/mod.rs`` 中的 ``add_initproc`` 函数，它会调用 ``TaskControlBlock::new`` 读取并解析初始应用 ``initproc`` 的 ELF 文件数据并创建初始进程 ``INITPROC`` ，随后会将它加入到全局任务管理器 ``TASK_MANAGER`` 中参与调度。
.. - 进程切换机制。当一个进程退出或者是主动/被动交出 CPU 使用权之后需要由内核将 CPU 使用权交给其他进程。在本章中我们沿用 ``os/src/task/mod.rs`` 中的 ``suspend_current_and_run_next`` 和 ``exit_current_and_run_next`` 两个接口来实现进程切换功能，但是需要适当调整它们的实现。我们需要调用 ``os/src/task/task.rs`` 中的 ``schedule`` 函数进行进程切换，它会首先切换到处理器的 idle 控制流（即 ``os/src/task/processor`` 的 ``Processor::run`` 方法），然后在里面选取要切换到的进程并切换过去。
.. - 进程调度机制。在进程切换的时候我们需要选取一个进程切换过去。选取进程逻辑可以参考 ``os/src/task/manager.rs`` 中的 ``TaskManager::fetch_task`` 方法。
.. - 进程生成机制。这主要是指 ``fork/exec`` 两个系统调用。它们的实现分别可以在 ``os/src/syscall/process.rs`` 中找到，分别基于 ``os/src/process/task.rs`` 中的 ``TaskControlBlock::fork/exec`` 。
.. - 进程资源回收机制。当一个进程主动退出或出错退出的时候，在 ``exit_current_and_run_next`` 中会立即回收一部分资源并在进程控制块中保存退出码；而需要等到它的父进程通过 ``waitpid`` 系统调用（与 ``fork/exec`` 两个系统调用放在相同位置）捕获到它的退出码之后，它的进程控制块才会被回收，从而所有资源都被回收。
.. - 为了支持用户终端 ``user_shell`` 读取用户键盘输入的功能，还需要实现 ``read`` 系统调用，它可以在 ``os/src/syscall/fs.rs`` 中找到。