实现应用程序
===========================

.. toctree::
   :hidden:
   :maxdepth: 5

.. note::

    拓展阅读：`RISC-V 特权级机制 <https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter2/1rv-privilege.html>`_


应用程序设计
-----------------------------

.. attention::

    用户库看起来很复杂，它预留了直到 ch7 内核才能实现的系统调用接口，console 模块还实现了输出缓存区。它们不是为本章准备的，你只需关注本节提到的部分即可。


应用程序、用户库（包括入口函数、初始化函数、I/O函数和系统调用接口等多个rs文件组成）放在项目根目录的 ``user`` 目录下：

- user/src/bin/*.rs：各个应用程序
- user/src/*.rs：用户库（包括入口函数、初始化函数、I/O函数和系统调用接口等）
- user/src/linker.ld：应用程序的内存布局说明

项目结构
^^^^^^^^^^^^^^^^^^^^^^

``user/src/bin`` 里面有多个文件，其中三个是：

- ``hello_world``：在屏幕上打印一行 ``Hello, world!``
- ``bad_address``：访问一个非法的物理地址，测试批处理系统是否会被该错误影响
- ``power``：不断在计算操作和打印字符串操作之间切换

批处理系统会按照文件名顺序加载并运行它们。

每个应用程序的实现都在对应的单个文件中。打开 ``hello_world.rs``，能看到一个 ``main`` 函数，还有外部库引用：

.. code-block:: rust

    #[macro_use]
    extern crate user_lib;

这个外部库其实就是 ``user`` 目录下的 ``lib.rs`` 以及它引用的若干子模块。
在 ``user/Cargo.toml`` 中我们对于库的名字进行了设置： ``name =  "user_lib"`` 。
它作为 ``bin`` 目录下的源程序所依赖的用户库，等价于其他编程语言提供的标准库。

在 ``lib.rs`` 中我们定义了用户库的入口点 ``_start`` ：

.. code-block:: rust
    :linenos:

    #[no_mangle]
    #[link_section = ".text.entry"]
    pub extern "C" fn _start() -> ! {
        clear_bss();
        exit(main());
    }

第 2 行使用 ``link_section`` 宏将 ``_start`` 函数编译后的汇编代码放在名为 ``.text.entry`` 的代码段中，
方便用户库链接脚本将它作为用户程序的入口。

而从第 4 行开始，我们手动清零 ``.bss`` 段，然后调用 ``main`` 函数得到一个类型为 ``i32`` 的返回值，
最后，调用用户库提供的 ``exit`` 接口退出，并将返回值告知批处理系统。

我们在 ``lib.rs`` 中看到了另一个 ``main`` ：

.. code-block:: rust
    :linenos:

    #![feature(linkage)]    // 启用弱链接特性

    #[linkage = "weak"]
    #[no_mangle]
    fn main() -> i32 {
        panic!("Cannot find main!");
    }

我们使用 Rust 宏将其标志为弱链接。这样在最后链接的时候，
虽然 ``lib.rs`` 和 ``bin`` 目录下的某个应用程序中都有 ``main`` 符号，
但由于 ``lib.rs`` 中的 ``main`` 符号是弱链接，
链接器会使用 ``bin`` 目录下的函数作为 ``main`` 。
如果在 ``bin`` 目录下找不到任何 ``main`` ，那么编译也能通过，但会在运行时报错。

内存布局
^^^^^^^^^^^^^^^^^^^^^^

我们使用链接脚本 ``user/src/linker.ld`` 规定用户程序的内存布局：

- 将程序的起始物理地址调整为 ``0x80400000`` ，三个应用程序都会被加载到这个物理地址上运行；
- 将 ``_start`` 所在的 ``.text.entry`` 放在整个程序的开头 ``0x80400000``；
  批处理系统在加载应用后，跳转到 ``0x80400000``，就进入了用户库的 ``_start`` 函数；
- 提供了最终生成可执行文件的 ``.bss`` 段的起始和终止地址，方便 ``clear_bss`` 函数使用。

其余的部分和第一章基本相同。

系统调用
^^^^^^^^^^^^^^^^^^^^^^

在子模块 ``syscall`` 中我们来通过 ``ecall`` 调用批处理系统提供的接口，
由于应用程序运行在用户态（即 U 模式）， ``ecall`` 指令会触发名为 ``Environment call from U-mode`` 的异常，
并 Trap 进入 S 模式执行批处理系统针对这个异常特别提供的服务程序。
这个接口被称为 ABI 或者系统调用。
现在我们不关心 S 态的批处理系统如何提供应用程序所需的功能，只考虑如何使用它。

在本章中，应用程序和批处理系统约定如下两个系统调用：

.. code-block:: rust
    :caption: 第二章新增系统调用

    /// 功能：将内存中缓冲区中的数据写入文件。
    /// 参数：`fd` 表示待写入文件的文件描述符；
    ///      `buf` 表示内存中缓冲区的起始地址；
    ///      `len` 表示内存中缓冲区的长度。
    /// 返回值：返回成功写入的长度。
    /// syscall ID：64
    fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize;

    /// 功能：退出应用程序并将返回值告知批处理系统。
    /// 参数：`xstate` 表示应用程序的返回值。
    /// 返回值：该系统调用不应该返回。
    /// syscall ID：93
    fn sys_exit(xstate: usize) -> !;

实际调用时，我们要按照 RISC-V 调用规范，在合适的寄存器中放置参数，
然后执行 ``ecall`` 指令触发 Trap。当 Trap 结束，回到 U 模式后，
用户程序会从 ``ecall`` 的下一条指令继续执行，同时在合适的寄存器中读取返回值。

.. note::

   RISC-V 寄存器编号从 ``0~31`` ，表示为 ``x0~x31`` 。 其中：
   -  ``x10~x17`` : 对应  ``a0~a7``
   -  ``x1`` ：对应 ``ra``

约定寄存器 ``a0~a6`` 保存系统调用的参数， ``a0`` 保存系统调用的返回值，
寄存器 ``a7`` 用来传递 syscall ID。
这超出了 Rust 语言的表达能力，我们需要内嵌汇编来完成参数/返回值绑定和 ``ecall`` 指令的插入：

.. code-block:: rust
    :linenos:

    // user/src/syscall.rs

    fn syscall(id: usize, args: [usize; 3]) -> isize {
       let mut ret: isize;
       unsafe {
           core::arch::asm!(
               "ecall",
               inlateout("x10") args[0] => ret,
               in("x11") args[1],
               in("x12") args[2],
               in("x17") id
           );
       }
       ret
    }

第 3 行，我们将所有的系统调用都封装成 ``syscall`` 函数，可以看到它支持传入 syscall ID 和 3 个参数。

第 6 行开始，我们使用 Rust 提供的 ``asm!`` 宏在代码中内嵌汇编。
Rust 编译器无法判定汇编代码的安全性，所以我们需要将其包裹在 unsafe 块中。

简而言之，这条汇编代码的执行结果是以寄存器 ``a0~a2`` 来保存系统调用的参数，以及寄存器 ``a7`` 保存 syscall ID，
返回值通过寄存器 ``a0`` 传递给局部变量 ``ret``。

这段汇编代码与第一章中出现过的内嵌汇编很像，读者可以查看 ``os/src/sbi.rs`` 。

.. note::

   可以查看 `Inline assembly <https://doc.rust-lang.org/nightly/reference/inline-assembly.html>`_ 了解 ``asm`` 宏。

于是 ``sys_write`` 和 ``sys_exit`` 只需将 ``syscall`` 进行包装：

.. code-block:: rust
    :linenos:

    // user/src/syscall.rs

    const SYSCALL_WRITE: usize = 64;
    const SYSCALL_EXIT: usize = 93;

    pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
        syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
    }

    pub fn sys_exit(xstate: i32) -> isize {
        syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
    }

我们将上述两个系统调用在用户库 ``user_lib`` 中进一步封装，像标准库一样：

.. code-block:: rust
    :linenos:

    // user/src/lib.rs
    use syscall::*;

    pub fn write(fd: usize, buf: &[u8]) -> isize { sys_write(fd, buf) }
    pub fn exit(exit_code: i32) -> isize { sys_exit(exit_code) }

在 ``console`` 子模块中，借助 ``write``，我们为应用程序实现了 ``println!`` 宏。
传入到 ``write`` 的 ``fd`` 参数设置为 1，代表标准输出 STDOUT，暂时不用考虑其他的 ``fd`` 选取情况。


编译生成应用程序二进制码
-------------------------------

简要介绍一下应用程序的构建，在 ``user`` 目录下 ``make build``：

1. 对于 ``src/bin`` 下的每个应用程序，
   在 ``target/riscv64gc-unknown-none-elf/release`` 目录下生成一个同名的 ELF 可执行文件；
2. 使用 objcopy 二进制工具删除所有 ELF header 和符号，得到 ``.bin`` 后缀的纯二进制镜像文件。
   它们将被链接进内核，并由内核在合适的时机加载到内存。
