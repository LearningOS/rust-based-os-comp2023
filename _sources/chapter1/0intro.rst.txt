引言
=====================

本章导读
--------------------------

大多数程序员的职业生涯都从 ``Hello, world!`` 开始。

.. code-block::

   printf("Hello world!\n");
   cout << "Hello world!\n";
   print("Hello world!")
   System.out.println("Hello world!");
   echo "Hello world!"
   println!("Hello world!");

然而，要用几行代码向世界问好，并不像表面上那么简单。
``Hello, world!`` 程序能够编译运行，靠的是以 **编译器** 为主的开发环境和以 **操作系统** 为主的执行环境。

在本章中，我们将抽丝剥茧，一步步让 ``Hello, world!`` 程序脱离其依赖的执行环境，
编写一个能打印 ``Hello, world!`` 的 OS。这趟旅途将让我们对应用程序及其执行环境有更深入的理解。

.. attention::
   实验指导书存在的目的是帮助读者理解框架代码。

   为便于测试，完成编程实验时，请以框架代码为基础，不必跟着文档从零开始编写内核。

为了做到这一步，首先需要让程序不依赖于标准库，
并通过编译。

接下来要让脱离了标准库的程序能输出（即支持 ``println!``)，这对程序的开发和调试至关重要。
我们先在用户态下实现该功能，在 `此处 <https://github.com/LearningOS/rCore-Tutorial-Book-2021Autumn/tree/ch2-U-nostd>`_ 获取相关代码。

最后把程序移植到内核态，构建在裸机上支持输出的最小运行时环境。

实践体验
---------------------------

本章一步步实现了支持打印字符串的简单操作系统。

获取本章代码：

.. code-block:: console

   $ git clone https://github.com/LearningOS/rCore-Tutorial-Code-2022S
   $ cd rCore-Tutorial-Code-2022S
   $ git checkout ch1

运行本章代码，并设置日志级别为 ``TRACE``：

.. code-block:: console

   $ cd os
   $ make run LOG=TRACE


预期输出：

.. figure:: color-demo.png
   :align: center

除了 ``Hello, world!`` 之外还有一些额外的信息，最后关机。

本章代码树
------------------------------------------------


.. code-block::

   ├── bootloader (内核依赖的运行在 M 特权级的 SBI 实现，本项目中我们使用 RustSBI)
   │   └── rustsbi-qemu.bin
   ├── os
   │   ├── Cargo.toml (cargo 项目配置文件)
   │   ├── Makefile
   │   └── src
   │       ├── console.rs (将打印字符的 SBI 接口进一步封装实现更加强大的格式化输出)
   │       ├── entry.asm (设置内核执行环境的的一段汇编代码)
   │       ├── lang_items.rs (需要我们提供给 Rust 编译器的一些语义项，目前包含内核 panic 时的处理逻辑)
   │       ├── linker.ld (控制内核内存布局的链接脚本以使内核运行在 qemu 虚拟机上)
   │       ├── logging.rs (为本项目实现了日志功能)
   │       ├── main.rs (内核主函数)
   │       └── sbi.rs (封装底层 SBI 实现提供的 SBI 接口)
   └── rust-toolchain (整个项目的工具链版本)

   cloc os
   -------------------------------------------------------------------------------
   Language                     files          blank        comment           code
   -------------------------------------------------------------------------------
   Rust                             5             25              6            155
   make                             1             11              4             34
   Assembly                         1              1              0             11
   TOML                             1              2              1              7
   -------------------------------------------------------------------------------
   SUM:                             8             39             11            207
   -------------------------------------------------------------------------------