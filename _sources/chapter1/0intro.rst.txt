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

.. note::

   基于github classroom的开发方式
   
   基于github classroom，可方便建立开发用的git repository，并可基于github的 codespace （在线版ubuntu +vscode）在线开发使用。整个开发环境仅仅需要一个网络浏览器。

   1. 在网络浏览器中用自己的 github  id 登录 github.com
   2. 接收 `第一个实验练习 setup-env-run-os1 的github classroom在线邀请 <https://classroom.github.com/a/hnoWuKGF>`_  ，根据提示一路选择OK即可。
   3. 完成第二步后，你的第一个实验练习 setup-env-run-os1 的 github repository 会被自动建立好，点击此github repository的链接，就可看到你要完成的第一个实验了。
   4. 在你的第一个实验练习的网页的中上部可以看到一个醒目的 `code`  绿色按钮，点击后，可以进一步看到  `codespace` 标签和醒目的 `create codesapce on main` 绿色按钮。请点击这个绿色按钮，就可以进入到在线的ubuntu +vscode环境中
   5. 再按照下面的环境安装提示在vscode的 `console` 中安装配置开发环境：rustc，qemu等工具。注：也可在vscode的 `console` 中执行 ``make codespaces_setenv`` 来自动安装配置开发环境（执行``sudo``需要root权限，仅需要执行一次）。
   6. 在vscode的 `console` 中执行 `make setupclassroom_test1`  （该命令仅执行一次）配置githubclassroom 自动评分功能。
   7. 然后就可以基于在线vscode进行开发、运行、提交等完整的实验过程了。

   上述的3，4，5步不是必须的，你也可以线下本地开发。

   注：如果是本地的ubuntu中建立开发环境，可在shell中执行 ``make ubuntu_local_setenv`` 来自动安装配置开发环境（执行``sudo``需要root权限，仅需要执行一次）。

本章一步步实现了支持打印字符串的简单操作系统。

获取本章代码：

.. code-block:: console

获取本章代码：

.. code-block:: console

   $ git clone ``gitaddr of github-classroom-build-lab0-0``
   $ cd ``github-classroom-build-lab0-0`` 
   $ make setupclassroom_test1  //注意：这一步很重要，是用于github classroom自动评测你的工作。这一步只需在首次克隆项目仓库时执行一次，以后一般就不用执行了，除非 .github/workflows/classroom.yml发生了变化。

.. note::

   实验名称 ：实验编号 
   
   -  lab0-0 : test1
   -  lab0-1：test2 
   -  lab1：test3
   -  lab2：test4
   -  lab3：test5
   -  lab4：test6
   -  lab5：test8

运行本章代码，并设置日志级别为 ``TRACE``：

.. code-block:: console

   $ cd os1
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