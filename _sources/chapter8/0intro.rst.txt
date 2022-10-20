引言
=========================================

本章导读
-----------------------------------------

到本章开始之前，我们好像已经完成了组成应用程序执行环境的操作系统的三个重要抽象：进程、地址空间和文件，
让应用程序开发、运行和存储数据越来越方便和灵活。有了进程以后，可以让操作系统从宏观层面实现多个应用的并发执行，
而并发是通过操作系统基于处理器的时间片不断地切换进程来达到的。到目前为止的并发，仅仅是进程间的并发，
对于一个进程内部还没有并发性的体现。而这就是线程（Thread）出现的起因：提高一个进程内的并发性。

.. chyyuu
   https://en.wikipedia.org/wiki/Per_Brinch_Hansen 关于操作系统并发  Binch Hansen 和 Hoare ??？
    https://en.wikipedia.org/wiki/Thread_(computing) 关于线程
    http://www.serpentine.com/blog/threads-faq/the-history-of-threads/ The history of threads
    https://en.wikipedia.org/wiki/Core_War 我喜欢的一种早期游戏
    [Dijkstra, 65] Dijkstra, E. W., Cooperating sequential processes, in Programming Languages, Genuys, F. (ed.), Academic Press, 1965.
    [Saltzer, 66] Saltzer, J. H., Traffic control in a multiplexed computer system, MAC-TR-30 (Sc.D. Thesis), July, 1966.
    https://en.wikipedia.org/wiki/THE_multiprogramming_system
    http://www.cs.utexas.edu/users/EWD/ewd01xx/EWD196.PDF
    https://en.wikipedia.org/wiki/Edsger_W._Dijkstra
    https://en.wikipedia.org/wiki/Per_Brinch_Hansen
    https://en.wikipedia.org/wiki/Tony_Hoare
    https://en.wikipedia.org/wiki/Mutual_exclusion
    https://en.wikipedia.org/wiki/Semaphore_(programming)
    https://en.wikipedia.org/wiki/Monitor_(synchronization)
    Dijkstra, Edsger W. The structure of the 'THE'-multiprogramming system (EWD-196) (PDF). E.W. Dijkstra Archive. Center for American History, University of Texas at Austin. (transcription) (Jun 14, 1965)


有了进程以后，为什么还会出现线程呢？考虑如下情况，对于很多应用（以单一进程的形式运行）而言，
逻辑上存在多个可并行执行的任务，如果其中一个任务被阻塞，将会引起不依赖该任务的其他任务也被阻塞。
举个具体的例子，我们平常用编辑器来编辑文本内容的时候，都会有一个定时自动保存的功能，
这个功能的作用是在系统或应用本身出现故障的情况前，已有的文档内容会被提前保存。
假设编辑器自动保存时由于磁盘性能导致写入较慢，导致整个进程被操作系统挂起，这就会影响到用户编辑文档的人机交互体验：
即软件的及时响应能力不足，用户只有等到磁盘写入完成后，操作系统重新调度该进程运行后，用户才可编辑。
如果我们把一个进程内的多个可并行执行任务通过一种更细粒度的方式让操作系统进行调度，
那么就可以通过处理器时间片切换实现这种细粒度的并发执行。这种细粒度的调度对象就是线程。


.. _term-thread-define:

线程定义
~~~~~~~~~~~~~~~~~~~~

简单地说，线程是进程的组成部分，进程可包含1 -- n个线程，属于同一个进程的线程共享进程的资源，
比如地址空间、打开的文件等。基本的线程由线程ID、执行状态、当前指令指针 (PC)、寄存器集合和栈组成。
线程是可以被操作系统或用户态调度器独立调度（Scheduling）和分派（Dispatch）的基本单位。

在本章之前，进程是程序的基本执行实体，是程序关于某数据集合上的一次运行活动，是系统进行资源（处理器、
地址空间和文件等）分配和调度的基本单位。在有了线程后，对进程的定义也要调整了，进程是线程的资源容器，
线程成为了程序的基本执行实体。


同步互斥
~~~~~~~~~~~~~~~~~~~~~~

在上面提到了同步互斥和数据一致性，它们的含义是什么呢？当多个线程共享同一进程的地址空间时，
每个线程都可以访问属于这个进程的数据（全局变量）。如果每个线程使用到的变量都是其他线程不会读取或者修改的话，
那么就不存在一致性问题。如果变量是只读的，多个线程读取该变量也不会有一致性问题。但是，当一个线程修改变量时，
其他线程在读取这个变量时，可能会看到一个不一致的值，这就是数据不一致性的问题。

.. note::

    **并发相关术语**

    - 共享资源（shared resource）：不同的线程/进程都能访问的变量或数据结构。
    - 临界区（critical section）：访问共享资源的一段代码。
    - 竞态条件（race condition）：多个线程/进程都进入临界区时，都试图更新共享的数据结构，导致产生了不期望的结果。
    - 不确定性（indeterminate）： 多个线程/进程在执行过程中出现了竞态条件，导致执行结果取决于哪些线程在何时运行，
      即执行结果不确定，而开发者期望得到的是确定的结果。
    - 互斥（mutual exclusion）：一种操作原语，能保证只有一个线程进入临界区，从而避免出现竞态，并产生确定的执行结果。
    - 原子性（atomic）：一系列操作要么全部完成，要么一个都没执行，不会看到中间状态。在数据库领域，
      具有原子性的一系列操作称为事务（transaction）。
    - 同步（synchronization）：多个并发执行的进程/线程在一些关键点上需要互相等待，这种相互制约的等待称为进程/线程同步。
    - 死锁（dead lock）：一个线程/进程集合里面的每个线程/进程都在等待只能由这个集合中的其他一个线程/进程
      （包括他自身）才能引发的事件，这种情况就是死锁。
    - 饥饿（hungry）：指一个可运行的线程/进程尽管能继续执行，但由于操作系统的调度而被无限期地忽视，导致不能执行的情况。

在后续的章节中，会大量使用上述术语，如果现在还不够理解，没关系，随着后续的一步一步的分析和实验，
相信大家能够掌握上述术语的实际含义。



实践体验
-----------------------------------------

.. note::

   基于github classroom的开发方式

   基于github classroom，可方便建立开发用的git repository，并可基于github的 codespace （在线版ubuntu +vscode）在线开发使用。整个开发环境仅仅需要一个网络浏览器。

   1. 在网络浏览器中用自己的 github id 登录 github.com
   2. 接收 `第五个实验(os8)的github classroom在线邀请 <https://classroom.github.com/a/zqGJEPK->`_  ，根据提示一路选择OK即可。
   3. 完成第二步后，你的第五个实验的 github repository 会被自动建立好，点击此github repository的链接，就可看到你要完成的第一个实验了。
   4. 在你的第五个实验的网页的中上部可以看到一个醒目的 `code`  绿色按钮，点击后，可以进一步看到  `codespace` 标签和醒目的 `create codesapce on main` 绿色按钮。请点击这个绿色按钮，就可以进入到在线的ubuntu +vscode环境中
   5. 再按照下面的环境安装提示在vscode的 `console` 中安装配置开发环境：rustc，qemu等工具。注：也可在vscode的 `console` 中执行 ``make codespaces_setenv`` 来自动安装配置开发环境（执行``sudo``需要root权限，仅需要执行一次）。
   6. 在vscode的 `console` 中执行 `make setupclassroom_test8`  （该命令仅执行一次）配置githubclassroom 自动评分功能。
   7. 然后就可以基于在线vscode进行开发、运行、提交等完整的实验过程了。

   上述的3，4，5步不是必须的，你也可以线下本地开发。

   注：如果是本地的ubuntu中建立开发环境，可在shell中执行 ``make ubuntu_local_setenv`` 来自动安装配置开发环境（执行``sudo``需要root权限，仅需要执行一次）。

获取本章代码：

.. code-block:: console

   $ git clone ``gitaddr of github-classroom-build-lab5``
   $ cd ``github-classroom-build-lab5``
   $ make setupclassroom_test8  //注意：这一步很重要，是用于github classroom自动评测你的工作。这一步只需在首次克隆项目仓库时执行一次，以后一般就不用执行了，除非 .github/workflows/classroom.yml发生了变化。

.. note::

   实验名称 ：实验编号

   -  lab0-0 : test1
   -  lab0-1：test2
   -  lab1：test3
   -  lab2：test4
   -  lab3：test5
   -  lab4：test6
   -  lab5：test8

在 qemu 模拟器上运行本章代码 `lab5(os8)参考框架: <https://github.com/LearningOS/rust-based-os-comp2022/tree/main/os8-ref>`_ ：

.. code-block:: console

   $ cd os8-ref
   $ make run

内核初始化完成之后就会进入 shell 程序，我们可以体会一下线程的创建和执行过程。在这里我们运行一下本章的测例 ``ch8b_threads`` ：

.. code-block::

    >> ch8b_threads
    aaa....bbb...ccc...
    thread#1 exited with code 1
    thread#2 exited with code 2
    thread#3 exited with code 3
    main thread exited.
    Shell: Process 2 exited with code 0
    >>

它会有4个线程在执行，等前3个线程执行完毕后，主线程退出，导致整个进程退出。

此外，在本章的操作系统支持通过互斥来执行“哲学家就餐问题”这个应用程序：

.. code-block::

    >> ch8b_phil_din_mutex
    Here comes 5 philosophers!
    time cost = 720
    '-' -> THINKING; 'x' -> EATING; ' ' -> WAITING
    #0: -------                 xxxxxxxx----------       xxxx-----  xxxxxx--xxx
    #1: ---xxxxxx--      xxxxxxx----------    x---xxxxxx
    #2: -----          xx---------xx----xxxxxx------------        xxxx
    #3: -----xxxxxxxxxx------xxxxx--------    xxxxxx--   xxxxxxxxx
    #4: ------         x------          xxxxxx--    xxxxx------   xx
    #0: -------                 xxxxxxxx----------       xxxx-----  xxxxxx--xxx
    Shell: Process 2 exited with code 0
    >>

我们可以看到5个代表“哲学家”的线程通过操作系统的 **信号量** 互斥机制在进行 “THINKING”、“EATING”、“WAITING” 的日常生活。
没有哲学家由于拿不到筷子而饥饿，也没有两个哲学家同时拿到一个筷子。

.. note::

    **哲学家就餐问题**

    计算机科学家 Dijkstra 提出并解决的哲学家就餐问题是经典的进程同步互斥问题。哲学家就餐问题描述如下：

    有5个哲学家共用一张圆桌，分别坐在周围的5张椅子上，在圆桌上有5个碗和5只筷子，他们的生活方式是交替地进行思考和进餐。
    平时，每个哲学家进行思考，饥饿时便试图拿起其左右最靠近他的筷子，只有在他拿到两只筷子时才能进餐。进餐完毕，放下筷子继续思考。


本章的 `lab5(os8)参考框架: <https://github.com/LearningOS/rust-based-os-comp2022/tree/main/os8-ref>`_  代码树
-----------------------------------------

.. code-block::
   :linenos:

    .
    ├── bootloader
    │   └── rustsbi-qemu.bin
    ├── Dockerfile
    ├── easy-fs
    │   ├── Cargo.lock
    │   ├── Cargo.toml
    │   └── src
    │       ├── bitmap.rs
    │       ├── block_cache.rs
    │       ├── block_dev.rs
    │       ├── efs.rs
    │       ├── layout.rs
    │       ├── lib.rs
    │       └── vfs.rs
    ├── easy-fs-fuse
    │   ├── Cargo.lock
    │   ├── Cargo.toml
    │   └── src
    │       └── main.rs
    ├── LICENSE
    ├── Makefile
    ├── os
    │   ├── build.rs
    │   ├── Cargo.lock
    │   ├── Cargo.toml
    │   ├── Makefile
    │   └── src
    │       ├── config.rs (修改：扩大了内核堆空间)
    │       ├── console.rs
    │       ├── drivers
    │       │   ├── block
    │       │   │   ├── mod.rs
    │       │   │   └── virtio_blk.rs
    │       │   └── mod.rs
    │       ├── entry.asm
    │       ├── fs
    │       │   ├── inode.rs
    │       │   ├── mod.rs
    │       │   ├── pipe.rs
    │       │   └── stdio.rs
    │       ├── lang_items.rs
    │       ├── linker.ld
    │       ├── logging.rs
    │       ├── main.rs
    │       ├── mm
    │       │   ├── address.rs
    │       │   ├── frame_allocator.rs
    │       │   ├── heap_allocator.rs
    │       │   ├── memory_set.rs (修改：去除了构建进程地址空间时分配用户栈和映射陷入上下文的逻辑)
    │       │   ├── mod.rs
    │       │   └── page_table.rs
    │       ├── sbi.rs
    │       ├── sync (新增：互斥锁、信号量和条件变量三种同步互斥机制的实现)
    │       │   ├── condvar.rs
    │       │   ├── mod.rs
    │       │   ├── mutex.rs
    │       │   ├── semaphore.rs
    │       │   └── up.rs
    │       ├── syscall
    │       │   ├── fs.rs (修改：将原先对 task 的调用改为对 process 的调用)
    │       │   ├── mod.rs
    │       │   ├── process.rs (修改：将原先对 task 的调用改为对 process 的调用)
    │       │   ├── sync.rs (新增：三种同步互斥机制相关的系统调用，以及基于定时器条件变量的 sleep 调用)
    │       │   └── thread.rs (新增：线程相关系统调用)
    │       ├── task
    │       │   ├── context.rs (修改：将任务上下文的成员变量改为 pub 类型)
    │       │   ├── id.rs (新增：由 pid.rs 修改而来，提供 pid/tid 、 kstack/ustack 的分配和回收机制)
    │       │   ├── kthread.rs (新增：完全在内核态运行的线程，仅供参考，在实验中未使用)
    │       │   ├── manager.rs
    │       │   ├── mod.rs (修改：增加阻塞线程的功能，将 exit 扩展到多线程，并在主线程退出时一并退出进程)
    │       │   ├── processor.rs (修改：增加获取当前线程的中断上下文虚拟地址及获取当前进程的功能)
    │       │   ├── process.rs (新增：将原先 Task 中的地址空间、文件等机制拆分为进程)
    │       │   ├── stackless_coroutine.rs (新增：完全在内核态运行的无栈协程，仅供参考，在实验中未使用)
    │       │   ├── switch.rs
    │       │   ├── switch.S
    │       │   └── task.rs (修改：将进程相关的功能移至 process.rs 中)
    │       ├── timer.rs (修改：增加定时器条件变量的实现)
    │       └── trap
    │           ├── context.rs
    │           ├── mod.rs (修改：使用线程对应的中断上下文地址而非固定的 TRAP_CONTEXT)
    │           └── trap.S
    ├── README.md
    └── rust-toolchain.toml
