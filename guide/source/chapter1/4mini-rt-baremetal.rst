.. _term-print-kernelminienv:

构建裸机执行环境
=================================

.. toctree::
   :hidden:
   :maxdepth: 5

有了上一节实现的用户态的最小执行环境，稍加改造，就可以完成裸机上的最小执行环境了。
本节中，我们将把 ``Hello world!`` 应用程序从用户态搬到内核态。


裸机启动过程
----------------------------

用 QEMU 软件 ``qemu-system-riscv64`` 来模拟 RISC-V 64 计算机。加载内核程序的命令如下：

.. code-block:: bash

    qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)


-  ``-bios $(BOOTLOADER)`` 意味着硬件加载了一个 BootLoader 程序，即 RustSBI
-  ``-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)`` 表示硬件内存中的特定位置 ``$(KERNEL_ENTRY_PA)`` 放置了操作系统的二进制代码 ``$(KERNEL_BIN)`` 。 ``$(KERNEL_ENTRY_PA)`` 的值是 ``0x80200000`` 。

当我们执行包含上述启动参数的 qemu-system-riscv64 软件，就意味给这台虚拟的 RISC-V64 计算机加电了。
此时，CPU 的其它通用寄存器清零，而 PC 会指向 ``0x1000`` 的位置，这里有固化在硬件中的一小段引导代码，
它会很快跳转到 ``0x80000000`` 的 RustSBI 处。
RustSBI完成硬件初始化后，会跳转到 ``$(KERNEL_BIN)`` 所在内存位置 ``0x80200000`` 处，
执行操作系统的第一条指令。

.. figure:: chap1-intro.png
   :align: center

.. note::

  **RustSBI 是什么？**

  SBI 是 RISC-V 的一种底层规范，RustSBI 是它的一种实现。
  操作系统内核与 RustSBI 的关系有点像应用与操作系统内核的关系，后者向前者提供一定的服务。只是SBI提供的服务很少，
  比如关机，显示字符串等。

实现关机功能
----------------------------

对上一节实现的代码稍作调整，通过 ``ecall`` 调用 RustSBI 实现关机功能：

.. _term-llvm-sbicall:

.. code-block:: rust

    // bootloader/rustsbi-qemu.bin 直接添加的SBI规范实现的二进制代码，给操作系统提供基本支持服务

    // os/src/sbi.rs
    fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
     let mut ret;
      unsafe {
          core::arch::asm!(
              "ecall",
    ...

    const SBI_SHUTDOWN: usize = 8;

    pub fn shutdown() -> ! {
        sbi_call(SBI_SHUTDOWN, 0, 0, 0);
        panic!("It should shutdown!");
    }

    // os/src/main.rs
    #[no_mangle]
    extern "C" fn _start() {
        shutdown();
    }


应用程序访问操作系统提供的系统调用的指令是 ``ecall`` ，操作系统访问
RustSBI提供的SBI调用的指令也是 ``ecall`` ，
虽然指令一样，但它们所在的特权级是不一样的。
简单地说，应用程序位于最弱的用户特权级（User Mode），
操作系统位于内核特权级（Supervisor Mode），
RustSBI位于机器特权级（Machine Mode）。
下一章会进一步阐释具体细节。

编译执行，结果如下：

.. code-block:: bash

  # 编译生成ELF格式的执行文件
  $ cargo build --release
   Compiling os v0.1.0 (/media/chyyuu/ca8c7ba6-51b7-41fc-8430-e29e31e5328f/thecode/rust/os_kernel_lab/os)
    Finished release [optimized] target(s) in 0.15s
  # 把ELF执行文件转成bianary文件
  $ rust-objcopy --binary-architecture=riscv64 target/riscv64gc-unknown-none-elf/release/os --strip-all -O binary target/riscv64gc-unknown-none-elf/release/os.bin

  # 加载运行
  $ qemu-system-riscv64 -machine virt -nographic -bios ../bootloader/rustsbi-qemu.bin -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000
  # 无法退出，风扇狂转，感觉碰到死循环

问题在哪？通过 rust-readobj 分析 ``os`` 可执行程序，发现其入口地址不是
RustSBI 约定的 ``0x80200000`` 。我们需要修改程序的内存布局并设置好栈空间。


设置正确的程序内存布局
----------------------------

可以通过 **链接脚本** (Linker Script) 调整链接器的行为，使得最终生成的可执行文件的内存布局符合我们的预期。

修改 Cargo 的配置文件来使用我们自己的链接脚本 ``os/src/linker.ld``：

.. code-block::
    :linenos:
    :emphasize-lines: 5,6,7,8

    // os/.cargo/config
    [build]
    target = "riscv64gc-unknown-none-elf"

    [target.riscv64gc-unknown-none-elf]
    rustflags = [
        "-Clink-arg=-Tsrc/linker.ld", "-Cforce-frame-pointers=yes"
    ]

具体的链接脚本 ``os/src/linker.ld`` 如下：

.. code-block::
    :linenos:

    OUTPUT_ARCH(riscv)
    ENTRY(_start)
    BASE_ADDRESS = 0x80200000;

    SECTIONS
    {
        . = BASE_ADDRESS;
        skernel = .;

        stext = .;
        .text : {
            *(.text.entry)
            *(.text .text.*)
        }

        . = ALIGN(4K);
        etext = .;
        srodata = .;
        .rodata : {
            *(.rodata .rodata.*)
        }

        . = ALIGN(4K);
        erodata = .;
        sdata = .;
        .data : {
            *(.data .data.*)
        }

        . = ALIGN(4K);
        edata = .;
        .bss : {
            *(.bss.stack)
            sbss = .;
            *(.bss .bss.*)
        }

        . = ALIGN(4K);
        ebss = .;
        ekernel = .;

        /DISCARD/ : {
            *(.eh_frame)
        }
    }

第 1 行我们设置了目标平台为 riscv ；第 2 行我们设置了整个程序的入口点为之前定义的全局符号 ``_start``；
第 3 行定义了一个常量 ``BASE_ADDRESS`` 为 ``0x80200000`` ，RustSBI 期望的 OS 起始地址；

.. attention::

    linker 脚本的语法不做要求，感兴趣的同学可以自行查阅相关资料。

从 ``BASE_ADDRESS`` 开始，代码段 ``.text``, 只读数据段 ``.rodata``，数据段 ``.data``, bss 段 ``.bss`` 由低到高依次放置，
且每个段都有两个全局变量给出其起始和结束地址（比如 ``.text`` 段的开始和结束地址分别是 ``stext`` 和 ``etext`` ）。


正确配置栈空间布局
----------------------------

用另一段汇编代码初始化栈空间：

.. code-block:: asm
    :linenos:

    # os/src/entry.asm
        .section .text.entry
        .globl _start
    _start:
        la sp, boot_stack_top
        call rust_main

        .section .bss.stack
        .globl boot_stack
    boot_stack:
        .space 4096 * 16
        .globl boot_stack_top
    boot_stack_top:

在第 8 行，我们预留了一块大小为 4096 * 16 字节，也就是 :math:`64\text{KiB}` 的空间，
用作操作系统的栈空间。
栈顶地址被全局符号 ``boot_stack_top`` 标识，栈底则被全局符号 ``boot_stack`` 标识。
同时，这块栈空间被命名为
``.bss.stack`` ，链接脚本里有它的位置。

``_start`` 作为操作系统的入口地址，将依据链接脚本被放在 ``BASE_ADDRESS`` 处。
``la sp, boot_stack_top`` 作为 OS 的第一条指令，
将 sp 设置为栈空间的栈顶。
简单起见，我们目前不考虑 sp 越过栈底 ``boot_stack`` ，也就是栈溢出的情形。
第二条指令则是函数调用 ``rust_main`` ，这里的 ``rust_main`` 是我们稍后自己编写的应用入口。

接着，我们在 ``main.rs`` 中嵌入这些汇编代码并声明应用入口 ``rust_main`` ：

.. code-block:: rust
    :linenos:
    :emphasize-lines: 7,9,10,11,12

    // os/src/main.rs
    #![no_std]
    #![no_main]

    mod lang_items;

    core::arch::global_asm!(include_str!("entry.asm"));

    #[no_mangle]
    pub fn rust_main() -> ! {
        shutdown();
    }

背景高亮指出了 ``main.rs`` 中新增的代码。

第 7 行，我们使用 ``global_asm`` 宏，将同目录下的汇编文件 ``entry.asm`` 嵌入到代码中。

从第 9 行开始，
我们声明了应用的入口点 ``rust_main`` ，需要注意的是，这里通过宏将 ``rust_main``
标记为 ``#[no_mangle]`` 以避免编译器对它的名字进行混淆，不然在链接时，
``entry.asm`` 将找不到 ``main.rs`` 提供的外部符号 ``rust_main``，导致链接失败。

再次使用上节中的编译，生成和运行操作，我们看到QEMU模拟的RISC-V 64计算机 **优雅** 地退出了！

.. code-block:: console
    # 教程使用的 RustSBI 版本比代码框架稍旧，输出有所不同
    $ qemu-system-riscv64 \
    > -machine virt \
    > -nographic \
    > -bios ../bootloader/rustsbi-qemu.bin \
    > -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000
    [rustsbi] Version 0.1.0
    .______       __    __      _______.___________.  _______..______   __
    |   _  \     |  |  |  |    /       |           | /       ||   _  \ |  |
    |  |_)  |    |  |  |  |   |   (----`---|  |----`|   (----`|  |_)  ||  |
    |      /     |  |  |  |    \   \       |  |      \   \    |   _  < |  |
    |  |\  \----.|  `--'  |.----)   |      |  |  .----)   |   |  |_)  ||  |
    | _| `._____| \______/ |_______/       |__|  |_______/    |______/ |__|

    [rustsbi] Platform: QEMU
    [rustsbi] misa: RV64ACDFIMSU
    [rustsbi] mideleg: 0x222
    [rustsbi] medeleg: 0xb1ab
    [rustsbi] Kernel entry: 0x80200000


清空 .bss 段
----------------------------------

等一等，与内存相关的部分太容易出错了， **清零 .bss 段** 的工作我们还没有完成。

.. code-block:: rust
    :linenos:

    // os/src/main.rs
    fn clear_bss() {
        extern "C" {
            fn sbss();
            fn ebss();
        }
        (sbss as usize..ebss as usize).for_each(|a| {
            unsafe { (a as *mut u8).write_volatile(0) }
        });
    }

    pub fn rust_main() -> ! {
        clear_bss();
        shutdown();
    }

链接脚本 ``linker.ld`` 中给出的全局符号 ``sbss`` 和 ``ebss`` 让我们能轻松确定 ``.bss`` 段的位置。


添加裸机打印相关函数
----------------------------------

在上一节中我们为用户态程序实现的 ``println`` 宏，略作修改即可用于本节的内核态操作系统。
详见 ``os/src/console.rs``。

利用 ``println`` 宏，我们重写异常处理函数 ``panic``，使其在 panic 时能打印错误发生的位置。
相关代码位于 ``os/src/lang_items.rs`` 中。

我们还使用第三方库 ``log`` 为你实现了日志模块，相关代码位于 ``os/src/logging.rs`` 中。

.. note::

    在 cargo 项目中引入外部库 log，需要修改 ``Cargo.toml`` 加入相应的依赖信息。

现在，让我们重复一遍本章开头的试验，``make run LOG=TRACE``！

.. figure:: color-demo.png
   :align: center

产生 panic 的地点与源码中的实际位置一致！至此，我们完成了第一章的实验内容，


.. note::

    背景知识：`理解应用程序和执行环境 <https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter1/4understand-prog.html>`_