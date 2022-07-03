chapter4练习
============================================

Lab2 编程作业
---------------------------------------------

重写 sys_get_time 和 sys_task_info
++++++++++++++++++++++++++++++++++++++++++++

引入虚存机制后，原来内核的 sys_get_time 和 sys_task_info 函数实现就无效了。请你重写这个函数，恢复其正常功能。

mmap 和 munmap 匿名映射
++++++++++++++++++++++++++++++++++++++++++++

`mmap <https://man7.org/linux/man-pages/man2/mmap.2.html>`_ 在 Linux 中主要用于在内存中映射文件，
本次实验简化它的功能，仅用于申请内存。

请实现 mmap 和 munmap 系统调用，mmap 定义如下：


.. code-block:: rust

    fn sys_mmap(start: usize, len: usize, port: usize) -> isize

- syscall ID：222
- 申请长度为 len 字节的物理内存（不要求实际物理内存位置，可以随便找一块），将其映射到 start 开始的虚存，内存页属性为 port
- 参数：
    - start 需要映射的虚存起始地址，要求按页对齐
    - len 映射字节长度，可以为 0
    - port：第 0 位表示是否可读，第 1 位表示是否可写，第 2 位表示是否可执行。其他位无效且必须为 0
- 返回值：执行成功则返回 0，错误返回 -1
- 说明：
    - 为了简单，目标虚存区间要求按页对齐，len 可直接按页向上取整，不考虑分配失败时的页回收。
- 可能的错误：
    - start 没有按页大小对齐
    - port & !0x7 != 0 (port 其余位必须为0)
    - port & 0x7 = 0 (这样的内存无意义)
    - [start, start + len) 中存在已经被映射的页
    - 物理内存不足

munmap 定义如下：

.. code-block:: rust

    fn sys_munmap(start: usize, len: usize) -> isize

- syscall ID：215
- 取消到 [start, start + len) 虚存的映射
- 参数和返回值请参考 mmap
- 说明：
    - 为了简单，参数错误时不考虑内存的恢复和回收。
- 可能的错误：
    - [start, start + len) 中存在未被映射的虚存。

tips:

- 一定要注意 mmap 是的页表项，注意 riscv 页表项的格式与 port 的区别。
- 你增加 PTE_U 了吗？

实验要求
++++++++++++++++++++++++++++++++++++++++++

-  `lab2(os4)参考框架: <https://github.com/LearningOS/rust-based-os-comp2022/tree/main/os4-ref>`_
- 在 ``os4`` 目录下，实现 mmap 和 munmap 两个系统调用，通过所有测例。
- 报告命名 lab2.md，位于 ``reports`` 目录下

TIPS：注意 port 参数的语义，它与内核定义的 MapPermission 有明显不同！

- 开发并通过所有测例

在 ``YOUR_LAB2_REPO_DIR`` 下进行编码（可学习参考 ``os4-ref/src`` 目录下的源代码，并在 ``os4/src`` 中完成编码），并进行编译测试。

.. note::

   **测试方式**

   你的实现只需且必须通过测例，建议读者感到困惑时先检查测例
   
   如果实现正确，可在项目仓库的根目录下执行 ``make test4`` ，应该看到类似如下的显示输出：

   .. code-block:: console
   
      $ cd  YOUR_LAB2_REPO_DIR
      $ make test4
      ......
      [rustsbi] RustSBI version 0.2.2, adapting to RISC-V SBI v1.0.0
      .______       __    __      _______.___________.  _______..______   __
      |   _  \     |  |  |  |    /       |           | /       ||   _  \ |  |
      |  |_)  |    |  |  |  |   |   (----`---|  |----`|   (----`|  |_)  ||  |
      |      /     |  |  |  |    \   \       |  |      \   \    |   _  < |  |
      |  |\  \----.|  `--'  |.----)   |      |  |  .----)   |   |  |_)  ||  |
      | _| `._____| \______/ |_______/       |__|  |_______/    |______/ |__|
      [rustsbi] Implementation     : RustSBI-QEMU Version 0.1.1
      [rustsbi] Platform Name      : riscv-virtio,qemu
      [rustsbi] Platform SMP       : 1
      [rustsbi] Platform Memory    : 0x80000000..0x88000000
      [rustsbi] Boot HART          : 0
      [rustsbi] Device Tree Region : 0x87000000..0x87000ef2
      [rustsbi] Firmware Address   : 0x80000000
      [rustsbi] Supervisor Address : 0x80200000
      [rustsbi] pmp01: 0x00000000..0x80000000 (-wr)
      [rustsbi] pmp02: 0x80000000..0x80200000 (---)
      [rustsbi] pmp03: 0x80200000..0x88000000 (xwr)
      ......
      [PASS] found <Hello, world from user mode program!>
      [PASS] found <Test power_3 OK29777!>
      [PASS] found <Test power_5 OK29777!>
      [PASS] found <Test power_7 OK29777!>
      [PASS] found <get_time OK29777! (\d+)>
      [PASS] found <Test sleep OK29777!>
      [PASS] found <current time_msec = (\d+)>
      [PASS] found <time_msec = (\d+) after sleeping (\d+) ticks, delta = (\d+)ms!>
      [PASS] found <Test sleep1 passed29777!>
      [PASS] found <Test write A OK29777!>
      [PASS] found <Test write B OK29777!>
      [PASS] found <Test write C OK29777!>
      [PASS] found <string from task info test>
      [PASS] found <Test task info OK29777!>
      [PASS] found <Test 04_1 OK29777!>
      [PASS] found <Test 04_4 test OK29777!>
      [PASS] found <Test 04_5 ummap OK29777!>
      [PASS] found <Test 04_6 ummap2 OK29777!>
      [PASS] not found <FAIL: T.T>
      [PASS] not found <Should cause error, Test 04_2 fail!>
      [PASS] not found <Should cause error, Test 04_3 fail!>

      Test passed29777: 21/21
      Report for lab1 found.
      Report for lab2 found.

- 提交你的修改

   - 如果是基于GitHub Classroom 开发, 在本地环境或在线codespaces环境下，执行 ``git push`` 命令，提交修改的代码到gitub进行CI自动评测。如果评测结果是 红色小叉 （位于repo的中上位置），可进一步点击红色小叉查找具体出错时的CI执行情况。 
  
问答作业
-------------------------------------------------

1. 请列举 SV39 页表页表项的组成，描述其中的标志位有何作用？

2. 缺页
    缺页指的是进程访问页面时页面不在页表中或在页表中无效的现象，此时 MMU 将会返回一个中断，
    告知 os 进程内存访问出了问题。os 选择填补页表并重新执行异常指令或者杀死进程。

    - 请问哪些异常可能是缺页导致的？
    - 发生缺页时，描述相关重要寄存器的值，上次实验描述过的可以简略。

    缺页有两个常见的原因，其一是 Lazy 策略，也就是直到内存页面被访问才实际进行页表操作。
    比如，一个程序被执行时，进程的代码段理论上需要从磁盘加载到内存。但是 os 并不会马上这样做，
    而是会保存 .text 段在磁盘的位置信息，在这些代码第一次被执行时才完成从磁盘的加载操作。

    - 这样做有哪些好处？

    其实，我们的 mmap 也可以采取 Lazy 策略，比如：一个用户进程先后申请了 10G 的内存空间，
    然后用了其中 1M 就直接退出了。按照现在的做法，我们显然亏大了，进行了很多没有意义的页表操作。

    - 处理 10G 连续的内存页面，对应的 SV39 页表大致占用多少内存 (估算数量级即可)？
    - 请简单思考如何才能实现 Lazy 策略，缺页时又如何处理？描述合理即可，不需要考虑实现。

    缺页的另一个常见原因是 swap 策略，也就是内存页面可能被换到磁盘上了，导致对应页面失效。

    - 此时页面失效如何表现在页表项(PTE)上？

3. 双页表与单页表

   为了防范侧信道攻击，我们的 os 使用了双页表。但是传统的设计一直是单页表的，也就是说，
   用户线程和对应的内核线程共用同一张页表，只不过内核对应的地址只允许在内核态访问。
   (备注：这里的单/双的说法仅为自创的通俗说法，并无这个名词概念，详情见 `KPTI <https://en.wikipedia.org/wiki/Kernel_page-table_isolation>`_ )

   - 在单页表情况下，如何更换页表？
   - 单页表情况下，如何控制用户态无法访问内核页面？（tips:看看上一题最后一问）
   - 单页表有何优势？（回答合理即可）
   - 双页表实现下，何时需要更换页表？假设你写一个单页表操作系统，你会选择何时更换页表（回答合理即可）？

报告要求
--------------------------------------------------------

- 简单总结你实现的功能（200字以内，不要贴代码）。
- 完成问答题。
- (optional) 你对本次实验设计及难度/工作量的看法，以及有哪些需要改进的地方，欢迎畅所欲言。
