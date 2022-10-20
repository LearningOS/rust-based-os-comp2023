.. _term-print-userminienv:

构建用户态执行环境
=================================

.. toctree::
   :hidden:
   :maxdepth: 5

.. note::

  前三小节的用户态程序案例代码在 `此处 <https://github.com/LearningOS/rCore-Tutorial-Book-2021Autumn/tree/ch2-U-nostd>`_ 获取。


用户态最小化执行环境
----------------------------

执行环境初始化
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

首先我们要给 Rust 编译器编译器提供入口函数 ``_start()`` ，
在 ``main.rs`` 中添加如下内容：


.. code-block:: rust

  // os/src/main.rs
  #[no_mangle]
  extern "C" fn _start() {
      loop{};
  }


对上述代码重新编译，再用分析工具分析：


.. code-block:: console

   $ cargo build
      Compiling os v0.1.0 (/home/shinbokuow/workspace/v3/rCore-Tutorial-v3/os)
       Finished dev [unoptimized + debuginfo] target(s) in 0.06s

   [反汇编导出汇编程序]
   $ rust-objdump -S target/riscv64gc-unknown-none-elf/debug/os
      target/riscv64gc-unknown-none-elf/debug/os:	file format elf64-littleriscv

      Disassembly of section .text:

      0000000000011120 <_start>:
      ;     loop {}
        11120: 09 a0        	j	2 <_start+0x2>
        11122: 01 a0        	j	0 <_start+0x2>


反汇编出的两条指令就是一个死循环，
这说明编译器生成的已经是一个合理的程序了。
用 ``qemu-riscv64 target/riscv64gc-unknown-none-elf/debug/os`` 命令可以执行这个程序。


程序正常退出
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

我们把 ``_start()`` 函数中的循环语句注释掉，重新编译并分析，看到其汇编代码是：


.. code-block:: console

   $ rust-objdump -S target/riscv64gc-unknown-none-elf/debug/os

    target/riscv64gc-unknown-none-elf/debug/os:	file format elf64-littleriscv


    Disassembly of section .text:

    0000000000011120 <_start>:
    ; }
      11120: 82 80        	ret

看起来是合法的执行程序。但如果我们执行它，会引发问题：

.. code-block:: console

  $ qemu-riscv64 target/riscv64gc-unknown-none-elf/debug/os
    段错误 (核心已转储)

这个简单的程序导致 ``qemu-riscv64`` 崩溃了！为什么会这样？

.. note::

  QEMU有两种运行模式：

  ``User mode`` 模式，即用户态模拟，如 ``qemu-riscv64`` 程序，
  能够模拟不同处理器的用户态指令的执行，并可以直接解析ELF可执行文件，
  加载运行那些为不同处理器编译的用户级Linux应用程序。

  ``System mode`` 模式，即系统态模式，如 ``qemu-system-riscv64`` 程序，
  能够模拟一个完整的基于不同CPU的硬件系统，包括处理器、内存及其他外部设备，支持运行完整的操作系统。


目前的执行环境还缺了一个退出机制，我们需要操作系统提供的 ``exit`` 系统调用来退出程序。这里先给出代码：

.. code-block:: rust

  // os/src/main.rs

  const SYSCALL_EXIT: usize = 93;

  fn syscall(id: usize, args: [usize; 3]) -> isize {
      let mut ret;
      unsafe {
          core::arch::asm!(
              "ecall",
              inlateout("x10") args[0] => ret,
              in("x11") args[1],
              in("x12") args[2],
              in("x17") id,
          );
      }
      ret
  }

  pub fn sys_exit(xstate: i32) -> isize {
      syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
  }

  #[no_mangle]
  extern "C" fn _start() {
      sys_exit(9);
  }

``main.rs`` 增加的内容不多，但还是有点与一般的应用程序有所不同，因为它引入了汇编和系统调用。
第二章的第二节 :doc:`/chapter2/2application` 会详细介绍上述代码的含义。
这里读者只需要知道 ``_start`` 函数调用了一个 ``sys_exit`` 函数，
向操作系统发出了退出的系统调用请求，退出码为 ``9`` 。

我们编译执行以下修改后的程序：

.. code-block:: console

    $ cargo build --target riscv64gc-unknown-none-elf
      Compiling os v0.1.0 (/media/chyyuu/ca8c7ba6-51b7-41fc-8430-e29e31e5328f/thecode/rust/os_kernel_lab/os)
        Finished dev [unoptimized + debuginfo] target(s) in 0.26s

    [打印程序的返回值]
    $ qemu-riscv64 target/riscv64gc-unknown-none-elf/debug/os; echo $?
    9

可以看到，返回的结果确实是 ``9`` 。这样，我们勉强完成了一个简陋的用户态最小化执行环境。


有显示支持的用户态执行环境
----------------------------

没有 ``println`` 输出信息，终究觉得缺了点啥。

Rust 的 core 库内建了以一系列帮助实现显示字符的基本 Trait 和数据结构，函数等，我们可以对其中的关键部分进行扩展，就可以实现定制的 ``println!`` 功能。


实现输出字符串的相关函数
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
.. attention::

  如果你觉得理解 Rust 宏有困难，把它当成黑盒就好！

  学习rust宏的参考链接: `The Little Book of Rust Macros <https://veykril.github.io/tlborm/introduction.html>`_


首先封装一下对 ``SYSCALL_WRITE`` 系统调用。

.. code-block:: rust

  // os/src/main.rs

  const SYSCALL_WRITE: usize = 64;

  pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
  }

然后实现基于 ``Write`` Trait 的数据结构，并完成 ``Write`` Trait 所需要的  ``write_str`` 函数，并用 ``print`` 函数进行包装。最后，基于 ``print`` 函数，实现Rust语言 **格式化宏** ( `formatting macros <https://doc.rust-lang.org/std/fmt/#related-macros>`_ )。

.. code-block:: rust

    //  os/src/console.rs
    use core::fmt::{Write, Arguments, Result};
    use crate::sys_write;

    struct Stdout;

    impl Write for Stdout {
        fn write_str(&mut self, s: &str) -> Result {
            sys_write(1, s.as_bytes());
            Ok(())
        }
    }

    pub fn print(args: Arguments) {
        Stdout.write_fmt(args).unwrap();
    }

    macro_rules! print {
        ($fmt: literal $(, $($arg: tt)+)?) => {
            $crate::console::print(format_args!($fmt $(, $($arg)+)?));
        }
    }

    macro_rules! println {
        ($fmt: literal $(, $($arg: tt)+)?) => {
            $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
        }
    }

注： ``os/src/console.rs`` 文件的代码中使用到了 ``main.rs`` 文件中的sys_write方法，需要说明在文件头部声明 ``use crate::sys_write;`` 。

而 ``main.rs`` 为了能够用到 ``console.rs`` 提供的功能，也需要添加对 console 的引用。主要的添加如下：

.. code-block:: rust

    // os/src/main.rs

    //... other code
    #[macro_use]
    mod console;
    //... other code

  ...

接下来，我们调整一下应用程序，让它发出显示字符串和退出的请求：

.. code-block:: rust

  // os/src/main.rs

  #[no_mangle]
  extern "C" fn _start() {
      print!("Hello, ");
      println!("world!");
      sys_exit(9);
  }


现在，我们编译并执行一下，可以看到正确的字符串输出，且程序也能正确退出！


.. code-block:: console

  $ cargo build --target riscv64gc-unknown-none-elf
     Compiling os v0.1.0 (/media/chyyuu/ca8c7ba6-51b7-41fc-8430-e29e31e5328f/thecode/rust/os_kernel_lab/os)
    Finished dev [unoptimized + debuginfo] target(s) in 0.61s

  $ qemu-riscv64 target/riscv64gc-unknown-none-elf/debug/os; echo $?
    Hello, world!
    9


.. 下面出错的情况是会在采用 linker.ld，加入了 .cargo/config
.. 的内容后会出错：
.. .. [build]
.. .. target = "riscv64gc-unknown-none-elf"
.. .. [target.riscv64gc-unknown-none-elf]
.. .. rustflags = [
.. ..    "-Clink-arg=-Tsrc/linker.ld", "-Cforce-frame-pointers=yes"
.. .. ]

.. 重新定义了栈和地址空间布局后才会出错

.. 段错误 (核心已转储)

.. 系统崩溃了！借助以往的操作系统内核编程经验和与下一节调试kernel的成果经验，我们直接定位为是 **栈** (Stack) 没有设置的问题。我们需要添加建立栈的代码逻辑。

.. .. code-block:: asm

..   # entry.asm

..       .section .text.entry
..       .globl _start
..   _start:
..       la sp, boot_stack_top
..       call rust_main

..       .section .bss.stack
..       .globl boot_stack
..   boot_stack:
..       .space 4096 * 16
..       .globl boot_stack_top
..   boot_stack_top:

.. 然后把汇编代码嵌入到 ``main.rs`` 中，并进行微调。

.. .. code-block:: rust

..   #![feature(global_asm)]

..   global_asm!(include_str!("entry.asm"));

..   #[no_mangle]
..   #[link_section=".text.entry"]
..   extern "C" fn rust_main() {

.. 再次编译执行，可以看到正确的字符串输出，且程序也能正确结束！
