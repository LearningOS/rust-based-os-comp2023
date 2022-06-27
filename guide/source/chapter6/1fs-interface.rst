文件系统接口
=================================================

简易文件与目录抽象
-------------------------------------------------

与课堂所学相比，我们实现的文件系统进行了很大的简化：

- 扁平化：仅存在根目录 ``/`` 一个目录，所有的文件都放在根目录内。直接以文件名索引文件。
- 不设置用户和用户组概念，不记录文件访问/修改的任何时间戳，不支持软硬链接。
- 只实现了最基本的文件系统相关系统调用。

打开与读写文件的系统调用
--------------------------------------------------

打开文件
++++++++++++++++++++++++++++++++++++++++++++++++++

.. code-block:: rust

    /// 功能：打开一个常规文件，并返回可以访问它的文件描述符。
    /// 参数：path 描述要打开的文件的文件名（简单起见，文件系统不需要支持目录，所有的文件都放在根目录 / 下），
    /// flags 描述打开文件的标志，具体含义下面给出。
    /// dirfd 和 mode 仅用于保证兼容性，忽略
    /// 返回值：如果出现了错误则返回 -1，否则返回打开常规文件的文件描述符。可能的错误原因是：文件不存在。
    /// syscall ID：56
    fn sys_openat(dirfd: usize, path: &str, flags: u32, mode: u32) -> isize

目前我们的内核支持以下几种标志（多种不同标志可能共存）：

- 如果 ``flags`` 为 0，则表示以只读模式 *RDONLY* 打开；
- 如果 ``flags`` 第 0 位被设置（0x001），表示以只写模式 *WRONLY* 打开；
- 如果 ``flags`` 第 1 位被设置（0x002），表示既可读又可写 *RDWR* ；
- 如果 ``flags`` 第 9 位被设置（0x200），表示允许创建文件 *CREATE* ，在找不到该文件的时候应创建文件；如果该文件已经存在则应该将该文件的大小归零；
- 如果 ``flags`` 第 10 位被设置（0x400），则在打开文件的时候应该清空文件的内容并将该文件的大小归零，也即 *TRUNC* 。

在用户库 ``user_lib`` 中，我们将该系统调用封装为 ``open`` 接口：

.. code-block:: rust

    // user/src/lib.rs

    bitflags! {
        pub struct OpenFlags: u32 {
            const RDONLY = 0;
            const WRONLY = 1 << 0;
            const RDWR = 1 << 1;
            const CREATE = 1 << 9;
            const TRUNC = 1 << 10;
        }
    }

    pub fn open(path: &str, flags: OpenFlags) -> isize {
        sys_openat(AT_FDCWD as usize, path, flags.bits, OpenFlags::RDWR.bits)
    }

借助 ``bitflags!`` 宏我们将一个 ``u32`` 的 flags 包装为一个 ``OpenFlags`` 结构体，可以从它的 ``bits`` 字段获得 ``u32`` 表示。


顺序读写文件
++++++++++++++++++++++++++++++++++++++++++++++++++

在打开一个文件之后，我们就可以用之前的 ``sys_read/sys_write`` 两个系统调用来对它进行读写了。本教程只实现文件的顺序读写，而不考虑随机读写。
 
以本章的测试用例 ``ch6b_filetest_simple`` 来介绍文件系统接口的使用方法：

.. code-block:: rust
    :linenos:

    // user/src/bin/ch6b_filetest_simple.rs

    #![no_std]
    #![no_main]

    #[macro_use]
    extern crate user_lib;

    use user_lib::{
        open,
        close,
        read,
        write,
        OpenFlags,
    };

    #[no_mangle]
    pub fn main() -> i32 {
        let test_str = "Hello, world!";
        let filea = "filea\0";
        let fd = open(filea, OpenFlags::CREATE | OpenFlags::WRONLY);
        assert!(fd > 0);
        let fd = fd as usize;
        write(fd, test_str.as_bytes());
        close(fd);

        let fd = open(filea, OpenFlags::RDONLY);
        assert!(fd > 0);
        let fd = fd as usize;
        let mut buffer = [0u8; 100];
        let read_len = read(fd, &mut buffer) as usize;
        close(fd);

        assert_eq!(
            test_str,
            core::str::from_utf8(&buffer[..read_len]).unwrap(),
        );
        println!("file_test passed!");
        0
    }

- 第 20~25 行，我们以 *只写 + 创建* 的模式打开文件 ``filea`` ，向其中写入字符串 ``Hello, world!`` 而后关闭文件。
- 第 27~32 行，我们以只读 的方式将文件 ``filea`` 的内容读取到缓冲区 ``buffer`` 中。 ``filea`` 的总大小不超过缓冲区的大小，因此通过单次 ``read`` 即可将内容全部读出来而更常见的情况是需要进行多次 ``read`` ，直到返回值为 0 才能确认文件已被读取完毕。
