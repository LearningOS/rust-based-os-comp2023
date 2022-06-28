附录 B：常见工具的使用方法
========================================

.. toctree::
   :hidden:
   :maxdepth: 4



分析可执行文件
------------------------

对于Rust编译器生成的执行程序，可通过各种有效工具进行分析。如果掌握了对这些工具的使用，那么在后续的开发工作中，对碰到的各种奇怪问题就进行灵活处理和解决了。
我们以Rust编译生成的一个简单的“Hello, world”应用执行程序为分析对象，看看如何进行分析。

让我们先来通过 ``file`` 工具看看最终生成的可执行文件的格式：

.. code-block:: console

   $ cargo new os
   $ cd os; cargo build
      Compiling os v0.1.0 (/tmp/os)
      Finished dev [unoptimized + debuginfo] target(s) in 0.26s

   $ file target/debug/os
   target/debug/os: ELF 64-bit LSB shared object, x86-64, version 1 (SYSV), dynamically linked, 
   interpreter /lib64/ld-linux-x86-64.so.2, ......

   $

.. _term-elf:
.. _term-metadata:

从中可以看出可执行文件的格式为 **可执行和链接格式** (Executable and Linkable Format, ELF)，硬件平台是 x86-64。在 ELF 文件中，
除了程序必要的代码、数据段（它们本身都只是一些二进制的数据）之外，还有一些 **元数据** (Metadata) 描述这些段在地址空间中的位置和在
文件中的位置以及一些权限控制信息，这些元数据只能放在代码、数据段的外面。

rust-readobj
^^^^^^^^^^^^^^^^^^^^^^^

我们可以通过二进制工具 ``rust-readobj`` 来看看 ELF 文件中究竟包含什么内容，输入命令：

.. code-block:: console

   $ rust-readobj -all target/debug/os 

首先可以看到一个 ELF header，它位于 ELF 文件的开头：

.. code-block:: objdump
   :linenos:
   :emphasize-lines: 8,19,20,21,24,25,26,27

   File: target/debug/os
   Format: elf64-x86-64
   Arch: x86_64
   AddressSize: 64bit
   LoadName: 
   ElfHeader {
   Ident {
      Magic: (7F 45 4C 46)
      Class: 64-bit (0x2)
      DataEncoding: LittleEndian (0x1)
      FileVersion: 1
      OS/ABI: SystemV (0x0)
      ABIVersion: 0
      Unused: (00 00 00 00 00 00 00)
   }
   Type: SharedObject (0x3)
   Machine: EM_X86_64 (0x3E)
   Version: 1
   Entry: 0x5070
   ProgramHeaderOffset: 0x40
   SectionHeaderOffset: 0x32D8D0
   Flags [ (0x0)
   ]
   HeaderSize: 64
   ProgramHeaderEntrySize: 56
   ProgramHeaderCount: 12
   SectionHeaderEntrySize: 64
   SectionHeaderCount: 42
   StringTableSectionIndex: 41
   }
   ......

.. _term-magic:

- 第 8 行是一个称之为 **魔数** (Magic) 独特的常数，存放在 ELF header 的一个固定位置。当加载器将 ELF 文件加载到内存之前，通常会查看
  该位置的值是否正确，来快速确认被加载的文件是不是一个 ELF 。
- 第 19 行给出了可执行文件的入口点为 ``0x5070`` 。
- 从 20-21 行中，我们可以知道除了 ELF header 之外，还有另外两种不同的 header，分别称为 program header 和 section header，
  它们都有多个。ELF header 中给出了其他两种header 的大小、在文件中的位置以及数目。
- 从 24-27 行中，可以看到有 12 个不同的 program header，它们从文件的 0x40 字节偏移处开始，每个 56 字节；
  有64个section header,它们从文件的 0x2D8D0 字节偏移处开始，每个 64 字节；


有多个不同的 section header，下面是个具体的例子：

.. code-block:: objdump

   ......
   Section {
      Index: 14
      Name: .text (157)
      Type: SHT_PROGBITS (0x1)
      Flags [ (0x6)
         SHF_ALLOC (0x2)
         SHF_EXECINSTR (0x4)
      ]
      Address: 0x5070
      Offset: 0x5070
      Size: 208067
      Link: 0
      Info: 0
      AddressAlignment: 16
      EntrySize: 0
   }
         

每个 section header 则描述一个段的元数据。

其中，我们看到了代码段 ``.text`` 需要被加载到地址 ``0x5070`` ,大小 208067 字节，。
它们分别由元数据的字段 Offset、 Size 和 Address 给出。。

我们还能够看到程序中的符号表：

.. code-block::

  Symbol {
    Name: _start (37994)
    Value: 0x5070
    Size: 47
    Binding: Global (0x1)
    Type: Function (0x2)
    Other: 0
    Section: .text (0xE)
  }
   Symbol {
      Name: main (38021)
      Value: 0x51A0
      Size: 47
      Binding: Global (0x1)
      Type: Function (0x2)
      Other: 0
      Section: .text (0xE)
   }

里面包括了我们写的 ``main`` 函数的地址以及用户态执行环境的起始地址 ``_start`` 函数的地址。

因此，从 ELF header 中可以看出，ELF 中的内容按顺序应该是：

- ELF header
- 若干个 program header
- 程序各个段的实际数据
- 若干的 section header


rust-objdump
^^^^^^^^^^^^^^^^^^^^^^^

如果想了解正常的ELF文件的具体指令内容，可以通过 ``rust-objdump`` 工具反汇编ELF文件得到：

.. code-block:: console

   $ rust-objdump -all target/debug/os 

具体结果如下：

.. code-block:: objdump

      505b: e9 c0 ff ff ff                jmp     0x5020 <.plt>

      Disassembly of section .plt.got:

      0000000000005060 <.plt.got>:
         5060: ff 25 5a 3f 04 00             jmpq    *278362(%rip)  # 48fc0 <_GLOBAL_OFFSET_TABLE_+0x628>
         5066: 66 90                         nop

      Disassembly of section .text:

      0000000000005070 <_start>:
         5070: f3 0f 1e fa                   endbr64
         5074: 31 ed                         xorl    %ebp, %ebp
         5076: 49 89 d1                      movq    %rdx, %r9
         5079: 5e                            popq    %rsi
         507a: 48 89 e2                      movq    %rsp, %rdx
         507d: 48 83 e4 f0                   andq    $-16, %rsp
         5081: 50                            pushq   %rax
         5082: 54                            pushq   %rsp
         5083: 4c 8d 05 86 2c 03 00          leaq    208006(%rip), %r8  # 37d10 <__libc_csu_fini>
         508a: 48 8d 0d 0f 2c 03 00          leaq    207887(%rip), %rcx  # 37ca0 <__libc_csu_init>
         5091: 48 8d 3d 08 01 00 00          leaq    264(%rip), %rdi  # 51a0 <main>
         5098: ff 15 d2 3b 04 00             callq   *277458(%rip)  # 48c70 <_GLOBAL_OFFSET_TABLE_+0x2d8>
      ......
      00000000000051a0 <main>:
         51a0: 48 83 ec 18                   subq    $24, %rsp
         51a4: 8a 05 db 7a 03 00             movb    228059(%rip), %al  # 3cc85 <__rustc_debug_gdb_scripts_section__>
         51aa: 48 63 cf                      movslq  %edi, %rcx
         51ad: 48 8d 3d ac ff ff ff          leaq    -84(%rip), %rdi  # 5160 <_ZN2os4main17h717a6a6e05a70248E>
         51b4: 48 89 74 24 10                movq    %rsi, 16(%rsp)
         51b9: 48 89 ce                      movq    %rcx, %rsi
         51bc: 48 8b 54 24 10                movq    16(%rsp), %rdx
         51c1: 88 44 24 0f                   movb    %al, 15(%rsp)
         51c5: e8 f6 00 00 00                callq   0x52c0 <_ZN3std2rt10lang_start17hc258028f546a93a1E>
         51ca: 48 83 c4 18                   addq    $24, %rsp
         51ce: c3                            retq
         51cf: 90                            nop
      ......

从上面的反汇编结果，我们可以看到用户态执行环境的入口函数 ``_start`` 以及应用程序的主函数 ``main`` 的地址和具体汇编代码内容。      


rust-objcopy
^^^^^^^^^^^^^^^^^^^^^^^

当前的ELF执行程序有许多与执行无直接关系的信息（如调试信息等），可以通过 ``rust-objcopy`` 工具来清除。

.. code-block:: console

   $ rust-objcopy --strip-all target/debug/os target/debug/os.bin
   $ ls -l target/debug/os*
      -rwxrwxr-x 2 chyyuu chyyuu 3334992 1月  19 22:26 target/debug/os
      -rwxrwxr-x 1 chyyuu chyyuu  297200 1月  19 22:59 target/debug/os.bin

   $ ./target/debug/os.bin
      Hello, world!

可以看到，经过处理的ELF文件 ``os.bin`` 在文件长度上大大减少了，但也能正常执行。

另外，当将程序加载到内存的时候，对于每个 program header 所指向的区域，我们需要将对应的数据从文件复制到内存中。这就需要解析 ELF 的元数据
才能知道数据在文件中的位置以及即将被加载到内存中的位置。但如果我们不需要从 ELF 中解析元数据就知道程序的内存布局
（这个内存布局是我们按照需求自己指定的），我们可以手动完成加载任务。

具体的做法是利用 ``rust-objcopy`` 工具删除掉 ELF 文件中的
所有 header 只保留各个段的实际数据得到一个没有任何符号的纯二进制镜像文件：

.. code-block:: console

   $ rust-objcopy --strip-all target/debug/os -O binary target/debug/os.bin



这样就生成了一个没有任何符号的纯二进制镜像文件。由于缺少了必要的元数据，我们的 ``file`` 工具也没有办法
对它完成解析了。而后，我们可直接将这个二进制镜像文件手动载入到内存中合适位置即可。



qemu 平台上可执行文件和二进制镜像的生成流程
----------------------------------------------



make & Makefile
^^^^^^^^^^^^^^^^^^^^^^^

首先我们还原一下可执行文件和二进制镜像的生成流程：

.. code-block:: makefile

   # os/Makefile
   TARGET := riscv64gc-unknown-none-elf
   MODE := release
   KERNEL_ELF := target/$(TARGET)/$(MODE)/os
   KERNEL_BIN := $(KERNEL_ELF).bin

   $(KERNEL_BIN): kernel
      @$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

   kernel:
      @cargo build --release

这里可以看出 ``KERNEL_ELF`` 保存最终可执行文件 ``os`` 的路径，而 ``KERNEL_BIN`` 保存只保留各个段数据的二进制镜像文件 ``os.bin`` 
的路径。目标 ``kernel`` 直接通过 ``cargo build`` 以 release 模式最终可执行文件，目标 ``KERNEL_BIN`` 依赖于目标 ``kernel``，将
可执行文件通过 ``rust-objcopy`` 工具加上适当的配置移除所有的 header 和符号得到二进制镜像。

我们可以通过 ``make run`` 直接在 qemu 上运行我们的应用程序，qemu 是一个虚拟机，它完整的模拟了一整套硬件平台，就像是一台真正的计算机
一样，我们来看运行 qemu 的具体命令：

.. code-block:: makefile
   :linenos:
   :emphasize-lines: 11,12,13,14,15

   KERNEL_ENTRY_PA := 0x80020000

   BOARD		?= qemu
   SBI			?= rustsbi
   BOOTLOADER	:= ../bootloader/$(SBI)-$(BOARD).bin

   run: run-inner

   run-inner: build
      @qemu-system-riscv64 \
         -machine virt \
         -nographic \
         -bios $(BOOTLOADER) \
         -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)



qemu
^^^^^^^^^^^^^^^^^^^^^^^

注意其中高亮部分给出了传给 qemu 的参数。

- ``-machine`` 告诉 qemu 使用预设的硬件配置。在整个项目中我们将一直沿用该配置。
- ``-bios`` 告诉 qemu 使用我们放在 ``bootloader`` 目录下的预编译版本作为 bootloader。
- ``-device`` 则告诉 qemu 将二进制镜像加载到内存指定的位置。

可以先输入 Ctrl+A ，再输入 X 来退出 qemu 终端。

.. warning::

   **FIXME：使用 GDB 跟踪 qemu 的运行状态**

其他工具和文件格式说明的参考
-------------------------------------------------------

- `链接脚本(Linker Scripts)语法和规则解析(翻译自官方手册) <https://blog.csdn.net/m0_47799526/article/details/108765403>`_ 
- `Make 命令教程 <https://www.w3cschool.cn/mexvtg/>`_ 
