引言
================================

本章导读
---------------------------------


**批处理系统** (Batch System) 出现于计算资源匮乏的年代，其核心思想是：
将多个程序打包到一起输入计算机；当一个程序运行结束后，计算机会 *自动* 执行下一个程序。

应用程序难免会出错，如果一个程序的错误导致整个操作系统都无法运行，那就太糟糕了。
*保护* 操作系统不受出错程序破坏的机制被称为 **特权级** (Privilege) 机制，
它实现了用户态和内核态的隔离。

本章在上一章的基础上，让我们的 OS 内核能以批处理的形式一次运行多个应用程序，同时利用特权级机制，
令 OS 不因出错的用户态程序而崩溃。

本章首先为批处理操作系统设计用户程序，再阐述如何将这些应用程序链接到内核中，最后介绍如何利用特权级机制处理 Trap.

实践体验
---------------------------

本章我们引入了用户程序。为了将内核与应用解耦，我们将二者分成了两个仓库，分别是存放内核程序的 ``rCore-Tutorial-Code-20xxx`` （下称代码仓库，最后几位 x 表示学期）与存放用户程序的 ``rCore-Tutorial-Test-20xxx`` （下称测例仓库）。 你首先需要进入代码仓库文件夹并 clone 用户程序仓库（如果已经执行过该步骤则不需要再重复执行）：

.. code-block:: console

   $ git clone https://github.com/LearningOS/rust-based-os-comp2022.git
   $ cd rust-based-os-comp2022

上面的指令会将测例仓库克隆到代码仓库下并命名为 ``user`` ，注意 ``/user`` 在代码仓库的 ``.gitignore`` 文件中，因此不会出现 ``.git`` 文件夹嵌套的问题，并且你在代码仓库进行 checkout 操作时也不会影响测例仓库的内容。

在 qemu 模拟器上运行本章代码：

.. code-block:: console

   $ cd os2
   $ make run LOG=INFO

批处理系统自动加载并运行了所有的用户程序，尽管某些程序出错了：

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
  [ INFO] [kernel] num_app = 6
  [ INFO] [kernel] app_0 [0x8020b040, 0x8020f868)
  [ INFO] [kernel] app_1 [0x8020f868, 0x80214090)
  [ INFO] [kernel] app_2 [0x80214090, 0x80218988)
  [ INFO] [kernel] app_3 [0x80218988, 0x8021d160)
  [ INFO] [kernel] app_4 [0x8021d160, 0x80221a68)
  [ INFO] [kernel] app_5 [0x80221a68, 0x80226538)
  [ INFO] [kernel] Loading app_0
  [ERROR] [kernel] PageFault in application, core dumped.
  [ INFO] [kernel] Loading app_1
  [ERROR] [kernel] IllegalInstruction in application, core dumped.
  [ INFO] [kernel] Loading app_2
  [ERROR] [kernel] IllegalInstruction in application, core dumped.
  [ INFO] [kernel] Loading app_3
  [ INFO] [kernel] Application exited with code 1234
  [ INFO] [kernel] Loading app_4
  Hello, world from user mode program!
  [ INFO] [kernel] Application exited with code 0
  [ INFO] [kernel] Loading app_5
  3^10000=5079(MOD 10007)
  3^20000=8202(MOD 10007)
  3^30000=8824(MOD 10007)
  3^40000=5750(MOD 10007)
  3^50000=3824(MOD 10007)
  3^60000=8516(MOD 10007)
  3^70000=2510(MOD 10007)
  3^80000=9379(MOD 10007)
  3^90000=2621(MOD 10007)
  3^100000=2749(MOD 10007)
  Test power OK!
  [ INFO] [kernel] Application exited with code 0
  Panicked at src/batch.rs:68 All applications completed!

本章代码树
-------------------------------------------------

.. code-block::

   ── os2
   │   ├── Cargo.toml
   │   ├── Makefile (修改：构建内核之前先构建应用)
   │   ├── build.rs (新增：生成 link_app.S 将应用作为一个数据段链接到内核)
   │   └── src
   │       ├── batch.rs(新增：实现了一个简单的批处理系统)
   │       ├── console.rs
   │       ├── entry.asm
   │       ├── lang_items.rs
   │       ├── link_app.S(构建产物，由 os/build.rs 输出)
   │       ├── linker.ld
   │       ├── logging.rs
   │       ├── main.rs(修改：主函数中需要初始化 Trap 处理并加载和执行应用)
   │       ├── sbi.rs
   │       ├── sync(新增：包装了RefCell，暂时不用关心)
   │       │   ├── mod.rs
   │       │   └── up.rs
   │       ├── syscall(新增：系统调用子模块 syscall)
   │       │   ├── fs.rs(包含文件 I/O 相关的 syscall)
   │       │   ├── mod.rs(提供 syscall 方法根据 syscall ID 进行分发处理)
   │       │   └── process.rs(包含任务处理相关的 syscall)
   │       └── trap(新增：Trap 相关子模块 trap)
   │           ├── context.rs(包含 Trap 上下文 TrapContext)
   │           ├── mod.rs(包含 Trap 处理入口 trap_handler)
   │           └── trap.S(包含 Trap 上下文保存与恢复的汇编代码)
   └── user(新增：应用测例保存在 user 目录下)
      ├── Cargo.toml
      ├── Makefile
      └── src
         ├── bin(基于用户库 user_lib 开发的应用，每个应用放在一个源文件中)
         │   ├── ...
         ├── console.rs
         ├── lang_items.rs
         ├── lib.rs(用户库 user_lib)
         ├── linker.ld(应用的链接脚本)
         └── syscall.rs(包含 syscall 方法生成实际用于系统调用的汇编指令，
                        各个具体的 syscall 都是通过 syscall 来实现的)

   cloc os
   -------------------------------------------------------------------------------
   Language                     files          blank        comment           code
   -------------------------------------------------------------------------------
   Rust                            14             62             21            435
   Assembly                         3              9             16            106
   make                             1             12              4             36
   TOML                             1              2              1              9
   -------------------------------------------------------------------------------
   SUM:                            19             85             42            586
   -------------------------------------------------------------------------------
