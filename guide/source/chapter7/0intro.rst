引言
=========================================

本章导读
-----------------------------------------

本章将基于文件描述符实现父子进程之间的通信机制——管道。
我们还将扩展 ``exec`` 系统调用，使之能传递运行参数，并进一步改进 shell 程序，使其支持重定向符号 ``>`` 和 ``<`` 。

实践体验
-----------------------------------------

获取本章代码：

.. code-block:: console

   $ git clone https://github.com/LearningOS/rust-based-os-comp2022.git
   $ cd rust-based-os-comp2022/

在 qemu 模拟器上运行  `os7参考框架: <https://github.com/LearningOS/rust-based-os-comp2022/tree/main/os7-ref>`_  ：

.. code-block:: console

   $ cd os7-ref
   $ make run

进入shell程序后，可以运行管道机制的简单测例 ``ch7b_pipetest``， ``ch7b_pipetest`` 需要保证父进程通过管道传输给子进程的字符串不会发生变化。

测例输出大致如下：

.. code-block::

   >> ch7b_pipetest
   Read OK, child process exited!
   pipetest passed!
   Shell: Process 2 exited with code 0
   >>

同样的，也可以运行较为复杂的测例 ``ch7b_pipe_large_test``，体验通过两个管道实现双向通信。

此外，在本章我们为shell程序支持了输入/输出重定向功能，可以将一个应用的输出保存到一个指定的文件。例如，下面的命令可以将 ``ch7b_yield`` 应用的输出保存在文件 ``fileb`` 当中，并在应用执行完毕之后确认它的输出：

.. code-block::

   >> ch7b_yield > fileb
   Shell: Process 2 exited with code 0
   >> ch7b_cat fileb
   Hello, I am process 2.
   Back in process 2, iteration 0.
   Back in process 2, iteration 1.
   Back in process 2, iteration 2.
   Back in process 2, iteration 3.
   Back in process 2, iteration 4.
   yield pass.

   Shell: Process 2 exited with code 0
   >>

`os7参考框架: <https://github.com/LearningOS/rust-based-os-comp2022/tree/main/os7-ref>`_  
-----------------------------------------------------------------------------------------------------------------

.. code-block::

    ── os7-ref
       └── src
           ├── ...
           ├── fs
           │   ├── inode.rs
           │   ├── mod.rs
           │   ├── pipe.rs(新增：实现了 File Trait 的第三个实现——可用来进程间通信的管道)
           │   └── stdio.rs
           ├── mm
           │   ├── address.rs
           │   ├── frame_allocator.rs
           │   ├── heap_allocator.rs
           │   ├── memory_set.rs
           │   ├── mod.rs
           │   └── page_table.rs
           ├── syscall
           │   ├── fs.rs(修改：添加了sys_pipe和sys_dup)
           │   ├── mod.rs
           │   └── process.rs(修改：sys_exec添加了对参数的支持)
           ├── task
               ├── context.rs
               ├── manager.rs
               ├── mod.rs
               ├── pid.rs
               ├── processor.rs
               ├── switch.rs
               ├── switch.S
               └── task.rs(修改：在exec中将参数压入用户栈中)

   cloc easy-fs os
   -------------------------------------------------------------------------------
   Language                     files          blank        comment           code
   -------------------------------------------------------------------------------
   Rust                            42            317            434           3574
   Assembly                         4             53             26            526
   make                             1             13              4             48
   TOML                             2              4              2             23
   -------------------------------------------------------------------------------
   SUM:                            49            387            466           4171
   -------------------------------------------------------------------------------


.. 本章代码导读
.. -----------------------------------------------------

.. 在本章第一节 :doc:`/chapter6/1file-descriptor` 中，我们引入了文件的概念，用它来代表进程可以读写的多种被内核管理的硬件/软件资源。进程必须通过系统调用打开一个文件，将文件加入到自身的文件描述符表中，才能通过文件描述符（也就是某个特定文件在自身文件描述符表中的下标）来读写该文件。

.. 文件的抽象 Trait ``File`` 声明在 ``os/src/fs/mod.rs`` 中，它提供了 ``read/write`` 两个接口，可以将数据写入应用缓冲区抽象 ``UserBuffer`` ，或者从应用缓冲区读取数据。应用缓冲区抽象类型 ``UserBuffer`` 来自 ``os/src/mm/page_table.rs`` 中，它将 ``translated_byte_buffer`` 得到的 ``Vec<&'static mut [u8]>`` 进一步包装，不仅保留了原有的分段读写能力，还可以将其转化为一个迭代器逐字节进行读写，这在读写一些流式设备的时候特别有用。

.. 在进程控制块 ``TaskControlBlock`` 中需要加入文件描述符表字段 ``fd_table`` ，可以看到它是一个向量，里面保存了若干实现了 ``File`` Trait 的文件，由于采用动态分发，文件的类型可能各不相同。 ``os/src/syscall/fs.rs`` 的 ``sys_read/write`` 两个读写文件的系统调用需要访问当前进程的文件描述符表，用应用传入内核的文件描述符来索引对应的已打开文件，并调用 ``File`` Trait 的 ``read/write`` 接口； ``sys_close`` 这可以关闭一个文件。调用 ``TaskControlBlock`` 的 ``alloc_fd`` 方法可以在文件描述符表中分配一个文件描述符。进程控制块的其他操作也需要考虑到新增的文件描述符表字段的影响，如 ``TaskControlBlock::new`` 的时候需要对 ``fd_table`` 进行初始化， ``TaskControlBlock::fork`` 中则需要将父进程的 ``fd_table`` 复制一份给子进程。

.. 到本章为止我们支持两种文件：标准输入输出和管道。不同于前面章节，我们将标准输入输出分别抽象成 ``Stdin`` 和 ``Stdout`` 两个类型，并为他们实现 ``File`` Trait 。在 ``TaskControlBlock::new`` 创建初始进程的时候，就默认打开了标准输入输出，并分别绑定到文件描述符 0 和 1 上面。

.. 管道 ``Pipe`` 是另一种文件，它可以用于父子进程间的单向进程间通信。我们也需要为它实现 ``File`` Trait 。 ``os/src/syscall/fs.rs`` 中的系统调用 ``sys_pipe`` 可以用来打开一个管道并返回读端/写端两个文件的文件描述符。管道的具体实现在 ``os/src/fs/pipe.rs`` 中，本章第二节 :doc:`/chapter6/2pipe` 中给出了详细的讲解。管道机制的测试用例可以参考 ``user/src/bin`` 目录下的 ``pipetest.rs`` 和 ``pipe_large_test.rs`` 两个文件。
