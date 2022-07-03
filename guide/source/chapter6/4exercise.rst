chapter6练习
================================================

Lab4 编程作业
-------------------------------------------------

硬链接
++++++++++++++++++++++++++++++++++++++++++++++++++

硬链接要求两个不同的目录项指向同一个文件，在我们的文件系统中也就是两个不同名称目录项指向同一个磁盘块。

本节要求实现三个系统调用 ``sys_linkat、sys_unlinkat、sys_stat`` 。
  
**linkat**：

    * syscall ID: 37
    * 功能：创建一个文件的一个硬链接， `linkat标准接口 <https://linux.die.net/man/2/linkat>`_ 。
    * Ｃ接口： ``int linkat(int olddirfd, char* oldpath, int newdirfd, char* newpath, unsigned int flags)``
    * Rust 接口： ``fn linkat(olddirfd: i32, oldpath: *const u8, newdirfd: i32, newpath: *const u8, flags: u32) -> i32``
    * 参数：
        * olddirfd，newdirfd: 仅为了兼容性考虑，本次实验中始终为 AT_FDCWD (-100)，可以忽略。
        * flags: 仅为了兼容性考虑，本次实验中始终为 0，可以忽略。
        * oldpath：原有文件路径
        * newpath: 新的链接文件路径。
    * 说明：
        * 为了方便，不考虑新文件路径已经存在的情况（属于未定义行为），除非链接同名文件。
        * 返回值：如果出现了错误则返回 -1，否则返回 0。
    * 可能的错误
        * 链接同名文件。

**unlinkat**:

    * syscall ID: 35
    * 功能：取消一个文件路径到文件的链接, `unlinkat标准接口 <https://linux.die.net/man/2/unlinkat>`_ 。
    * Ｃ接口： ``int unlinkat(int dirfd, char* path, unsigned int flags)``
    * Rust 接口： ``fn unlinkat(dirfd: i32, path: *const u8, flags: u32) -> i32``
    * 参数：
        * dirfd: 仅为了兼容性考虑，本次实验中始终为 AT_FDCWD (-100)，可以忽略。
        * flags: 仅为了兼容性考虑，本次实验中始终为 0，可以忽略。
        * path：文件路径。
    * 说明：
        * 注意考虑使用 unlink 彻底删除文件的情况，此时需要回收inode以及它对应的数据块。
    * 返回值：如果出现了错误则返回 -1，否则返回 0。
    * 可能的错误
        * 文件不存在。

**fstat**:

    * syscall ID: 80
    * 功能：获取文件状态。
    * Ｃ接口： ``int fstat(int fd, struct Stat* st)``
    * Rust 接口： ``fn fstat(fd: i32, st: *mut Stat) -> i32``
    * 参数：
        * fd: 文件描述符
        * st: 文件状态结构体

        .. code-block:: rust

            #[repr(C)]
            #[derive(Debug)]
            pub struct Stat {
                /// 文件所在磁盘驱动器号，该实验中写死为 0 即可
                pub dev: u64,
                /// inode 文件所在 inode 编号
                pub ino: u64,
                /// 文件类型
                pub mode: StatMode,
                /// 硬链接数量，初始为1
                pub nlink: u32,
                /// 无需考虑，为了兼容性设计
                pad: [u64; 7],
            }
            
            /// StatMode 定义：
            bitflags! {
                pub struct StatMode: u32 {
                    const NULL  = 0;
                    /// directory
                    const DIR   = 0o040000;
                    /// ordinary regular file
                    const FILE  = 0o100000;
                }
            }
        

实验要求
+++++++++++++++++++++++++++++++++++++++++++++
-  `lab4(os6)参考框架: <https://github.com/LearningOS/rust-based-os-comp2022/tree/main/os6-ref>`_ 
- 实验目录要求不变。
- 通过所有测例。

  在 ``os6`` 目录下 ``make run BASE=2`` 加载所有测例， ``ch6_usertest`` 打包了所有你需要通过的测例，你也可以通过修改这个文件调整本地测试的内容。

  你的内核必须前向兼容，能通过前一章的所有测例。


.. note::

    **如何调试 easy-fs**

    如果你在第一章练习题中已经借助 ``log`` crate 实现了日志功能，那么你可以直接在 ``easy-fs`` 中引入 ``log`` crate，通过 ``log::info!/debug!`` 等宏即可进行调试并在内核中看到日志输出。具体来说，在 ``easy-fs`` 中的修改是：在 ``easy-fs/Cargo.toml`` 的依赖中加入一行 ``log = "0.4.0"``，然后在 ``easy-fs/src/lib.rs`` 中加入一行 ``extern crate log`` 。

    你也可以完全在用户态进行调试。仿照 ``easy-fs-fuse`` 建立一个在当前操作系统中运行的应用程序，将测试逻辑写在 ``main`` 函数中。这个时候就可以将它引用的 ``easy-fs`` 的 ``no_std`` 去掉并使用 ``println!`` 进行调试。

- 开发并通过所有测例

在 ``YOUR_LAB4_REPO_DIR`` 下进行编码（可学习参考 ``os6-ref/src`` 目录下的源代码，并在 ``os6/src`` 中完成编码），并进行编译测试。

注意：本次实验需要扩展文件系统的功能，所以，除了需要修改扩展 ``os6`` 目录下的代码外，还需要修改扩展 ``easy-fs`` 和 ``easy-fs-fuse`` 下面的部分代码。

.. note::

   **测试方式**

   你的实现只需且必须通过测例，建议读者感到困惑时先检查测例
   
   如果实现正确，可在项目仓库的根目录下执行 ``make test6`` ，应该看到类似如下的显示输出：

   .. code-block:: console
   
      $ cd  YOUR_LAB4_REPO_DIR
      $ make test6
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
      [PASS] found <Test 04_5 ummap OK53852!>
      [PASS] found <Test 04_1 OK53852!>
      .....
      [PASS] found <Test write C OK53852!>
      [PASS] found <get_time OK53852! (\d+)>
      [PASS] not found <FAIL: T.T>
      [PASS] not found <Should cause error, Test 04_2 fail!>
      [PASS] not found <Should cause error, Test 04_3 fail!>

      Test passed53852: 27/27
      Report for lab1 found.
      Report for lab2 found.
      Report for lab3 found.
      Report for lab4 found.

- 提交你的修改
  
   - 如果是基于GitHub Classroom 开发, 在本地环境或在线codespaces环境下，执行 ``git push`` 命令，提交修改的代码到gitub进行CI自动评测。如果评测结果是 红色小叉 （位于repo的中上位置），可进一步点击红色小叉查找具体出错时的CI执行情况。 


问答作业
----------------------------------------------------------

1. 在我们的easy-fs中，root inode起着什么作用？如果root inode中的内容损坏了，会发生什么？

报告要求
-----------------------------------------------------------
- 简单总结你实现的功能（200字以内，不要贴代码）。
- 完成问答题。
- (optional) 你对本次实验设计及难度/工作量的看法，以及有哪些需要改进的地方，欢迎畅所欲言。
