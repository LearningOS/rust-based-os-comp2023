.. _term-remove-std:

移除标准库依赖
==========================

.. toctree::
   :hidden:
   :maxdepth: 5


首先在 ``os`` 目录下新建 ``.cargo`` 目录，并在这个目录下创建 ``config`` 文件，输入如下内容：

.. code-block:: toml

   # os/.cargo/config
   [build]
   target = "riscv64gc-unknown-none-elf"


这将使 cargo 工具在 os 目录下默认会使用 riscv64gc-unknown-none-elf 作为目标平台。
这种编译器运行的平台（x86_64）与可执行文件运行的目标平台不同的情况，称为 **交叉编译** (Cross Compile)。

移除 println! 宏
----------------------------------


我们在 ``main.rs`` 的开头加上一行 ``#![no_std]``，
告诉 Rust 编译器不使用 Rust 标准库 std 转而使用核心库 core。重新编译，报错如下：

.. error::

   .. code-block:: console

      $ cargo build
         Compiling os v0.1.0 (/home/shinbokuow/workspace/v3/rCore-Tutorial-v3/os)
      error: cannot find macro `println` in this scope
      --> src/main.rs:4:5
        |
      4 |     println!("Hello, world!");
        |     ^^^^^^^

println! 宏是由标准库 std 提供的，且会使用到一个名为 write 的系统调用。
无论如何，我们先将这行代码注释掉。


提供语义项 panic_handler
----------------------------------------------------

.. error::

   .. code-block:: console

      $ cargo build
         Compiling os v0.1.0 (/home/shinbokuow/workspace/v3/rCore-Tutorial-v3/os)
      error: `#[panic_handler]` function required, but not found

标准库 std 提供了 Rust 错误处理函数 ``#[panic_handler]``，其大致功能是打印出错位置和原因并杀死当前应用。
但核心库 core 并没有提供这项功能，得靠我们自己实现。

新建一个子模块 ``lang_items.rs``，在里面编写 panic 处理函数，通过标记 ``#[panic_handler]`` 告知编译器采用我们的实现：

.. code-block:: rust

   // os/src/lang_items.rs
   use core::panic::PanicInfo;

   #[panic_handler]
   fn panic(_info: &PanicInfo) -> ! {
       loop {}
   }

目前我们遇到错误什么都不做，只在原地 ``loop`` 。

移除 main 函数
-----------------------------

重新编译，又有了新错误：

.. error::

   .. code-block::

      $ cargo build
         Compiling os v0.1.0 (/home/shinbokuow/workspace/v3/rCore-Tutorial-v3/os)
      error: requires `start` lang_item

编译器提醒我们缺少一个名为 ``start`` 的语义项。
``start`` 语义项代表了标准库 std 在执行应用程序之前需要进行的一些初始化工作。由于我们禁用了标准库，编译器也就找不到这项功能的实现了。

在 ``main.rs`` 的开头加入设置 ``#![no_main]`` 告诉编译器我们没有一般意义上的 ``main`` 函数，
并将原来的 ``main`` 函数删除。这样编译器也就不需要考虑初始化工作了。

.. code-block:: console

   $ cargo build
      Compiling os v0.1.0 (/home/shinbokuow/workspace/v3/rCore-Tutorial-v3/os)
       Finished dev [unoptimized + debuginfo] target(s) in 0.06s

至此，我们终于移除了所有标准库依赖，目前的代码如下：

.. code-block:: rust

   // os/src/main.rs
   #![no_std]
   #![no_main]

   mod lang_items;

   // os/src/lang_items.rs
   use core::panic::PanicInfo;

   #[panic_handler]
   fn panic(_info: &PanicInfo) -> ! {
       loop {}
   }


分析被移除标准库的程序
-----------------------------

我们可以通过一些工具来分析目前的程序：

.. code-block:: console

   [文件格式]
   $ file target/riscv64gc-unknown-none-elf/debug/os
   target/riscv64gc-unknown-none-elf/debug/os: ELF 64-bit LSB executable, UCB RISC-V, ......

   [文件头信息]
   $ rust-readobj -h target/riscv64gc-unknown-none-elf/debug/os
      File: target/riscv64gc-unknown-none-elf/debug/os
      Format: elf64-littleriscv
      Arch: riscv64
      AddressSize: 64bit
      ......
      Type: Executable (0x2)
      Machine: EM_RISCV (0xF3)
      Version: 1
      Entry: 0x0
      ......
      }

   [反汇编导出汇编程序]
   $ rust-objdump -S target/riscv64gc-unknown-none-elf/debug/os
      target/riscv64gc-unknown-none-elf/debug/os:	file format elf64-littleriscv


通过 ``file`` 工具对二进制程序 ``os`` 的分析可以看到，它好像是一个合法的 RV64 执行程序，
但 ``rust-readobj`` 工具告诉我们它的入口地址 Entry 是 ``0``。
再通过 ``rust-objdump`` 工具把它反汇编，没有生成任何汇编代码。
可见，这个二进制程序虽然合法，但它是一个空程序，原因是缺少了编译器规定的入口函数 ``_start`` 。

从下一节开始，我们将着手实现本节移除的、由用户态执行环境提供的功能。

.. note:: 

   本节内容部分参考自 `BlogOS 的相关章节 <https://os.phil-opp.com/freestanding-rust-binary/>`_ 。

