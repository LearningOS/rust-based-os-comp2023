
.. _term-batchos:

实现批处理操作系统
==============================

.. toctree::
   :hidden:
   :maxdepth: 5

将应用程序链接到内核
--------------------------------------------

在本章中，我们要把应用程序的二进制镜像文件作为数据段链接到内核里，
内核需要知道应用程序的数量和它们的位置。

在 ``os/src/main.rs`` 中能够找到这样一行：

.. code-block:: rust

    core::arch::global_asm!(include_str!("link_app.S"));

这里我们引入了一段汇编代码 ``link_app.S`` ，它是在 ``make run`` 构建操作系统时自动生成的，里面的内容大致如下：

.. code-block:: asm
    :linenos:

    # os/src/link_app.S

        .align 3
        .section .data
        .global _num_app
    _num_app:
        .quad 3
        .quad app_0_start
        .quad app_1_start
        .quad app_2_start
        .quad app_2_end

        .section .data
        .global app_0_start
        .global app_0_end
    app_0_start:
        .incbin "../user/target/riscv64gc-unknown-none-elf/release/hello_world.bin"
    app_0_end:

        .section .data
        .global app_1_start
        .global app_1_end
    app_1_start:
        .incbin "../user/target/riscv64gc-unknown-none-elf/release/bad_address.bin"
    app_1_end:

        .section .data
        .global app_2_start
        .global app_2_end
    app_2_start:
        .incbin "../user/target/riscv64gc-unknown-none-elf/release/power.bin"
    app_2_end:

第 13 行开始的三个数据段分别插入了三个应用程序的二进制镜像，
并且各自有一对全局符号 ``app_*_start, app_*_end`` 指示它们的开始和结束位置。
而第 3 行开始的另一个数据段相当于一个 64 位整数数组。
数组中的第一个元素表示应用程序的数量，后面则按照顺序放置每个应用程序的起始地址，
最后一个元素放置最后一个应用程序的结束位置。这样数组中相邻两个元素记录了每个应用程序的始末位置，
这个数组所在的位置由全局符号 ``_num_app`` 所指示。

这个文件是在 ``cargo build`` 时，由脚本 ``os/build.rs`` 控制生成的。

找到并加载应用程序二进制码
-----------------------------------------------

我们在 ``os`` 的 ``batch`` 子模块中实现一个应用管理器 ``AppManager`` ，结构体定义如下：

.. code-block:: rust

    struct AppManager {
        num_app: usize,
        current_app: usize,
        app_start: [usize; MAX_APP_NUM + 1],
    }

初始化 ``AppManager`` 的全局实例：

.. code-block:: rust

    lazy_static! {
        static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe {
            UPSafeCell::new({
                extern "C" {
                    fn _num_app();
                }
                let num_app_ptr = _num_app as usize as *const usize;
                let num_app = num_app_ptr.read_volatile();
                let mut app_start: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
                let app_start_raw: &[usize] =
                    core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1);
                app_start[..=num_app].copy_from_slice(app_start_raw);
                AppManager {
                    num_app,
                    current_app: 0,
                    app_start,
                }
            })
        };
    }

初始化的逻辑很简单，就是找到 ``link_app.S`` 中提供的符号 ``_num_app`` ，并从这里开始解析出应用数量以及各个应用的开头地址。
用容器 ``UPSafeCell`` 包裹 ``AppManager`` 是为了防止全局对象 ``APP_MANAGER`` 被重复获取。

.. note::

    ``UPSafeCell`` 实现在 ``sync`` 模块中，调用 ``exclusive_access`` 方法能获取其内部对象的可变引用，
    如果程序运行中同时存在多个这样的引用，会触发 ``already borrowed: BorrowMutError``。

    ``UPSafeCell`` 既提供了内部可变性，又在单核情境下防止了内部对象被重复借用，我们将在后文中多次见到它。

这里使用了外部库 ``lazy_static`` 提供的 ``lazy_static!`` 宏。

``lazy_static!`` 宏提供了全局变量的运行时初始化功能。一般情况下，全局变量必须在编译期设置初始值，
但是有些全局变量的初始化依赖于运行期间才能得到的数据。
如这里我们借助 ``lazy_static!`` 声明了一个 ``AppManager`` 结构的名为 ``APP_MANAGER`` 的全局实例，
只有在它第一次被使用到的时候才会进行实际的初始化工作。

``AppManager`` 的方法中， ``print_app_info/get_current_app/move_to_next_app`` 都相当简单直接，需要说明的是 ``load_app``：

.. code-block:: rust
    :linenos:

    unsafe fn load_app(&self, app_id: usize) {
        if app_id >= self.num_app {
            panic!("All applications completed!");
        }
        info!("[kernel] Loading app_{}", app_id);
        // clear icache
        core::arch::asm!("fence.i");
        // clear app area
        core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT).fill(0);
        let app_src = core::slice::from_raw_parts(
            self.app_start[app_id] as *const u8,
            self.app_start[app_id + 1] - self.app_start[app_id],
        );
        let app_dst = core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len());
        app_dst.copy_from_slice(app_src);
    }

这个方法负责将参数 ``app_id`` 对应的应用程序的二进制镜像加载到物理内存以 ``0x80400000`` 起始的位置，
这个位置是批处理操作系统和应用程序之间约定的常数地址。
我们将从这里开始的一块内存清空，然后找到待加载应用二进制镜像的位置，并将它复制到正确的位置。

清空内存前，我们插入了一条奇怪的汇编指令 ``fence.i`` ，它是用来清理 i-cache 的。
我们知道， 缓存又分成 **数据缓存** (d-cache) 和 **指令缓存** (i-cache) 两部分，分别在 CPU 访存和取指的时候使用。
通常情况下， CPU 会认为程序的代码段不会发生变化，因此 i-cache 是一种只读缓存。
但在这里，我们会修改会被 CPU 取指的内存区域，使得 i-cache 中含有与内存不一致的内容，
必须使用 ``fence.i`` 指令手动清空 i-cache ，让里面所有的内容全部失效，
才能够保证程序执行正确性。

.. warning::

   **模拟器与真机的不同之处**

   在 Qemu 模拟器上，即使不加刷新 i-cache 的指令，大概率也能正常运行，但在物理计算机上不是这样。

``batch`` 子模块对外暴露出如下接口：

- ``init`` ：调用 ``print_app_info`` 的时第一次用到了全局变量 ``APP_MANAGER`` ，它在这时完成初始化；
- ``run_next_app`` ：批处理操作系统的核心操作，即加载并运行下一个应用程序。
  批处理操作系统完成初始化，或者应用程序运行结束/出错后会调用该函数。下节再介绍其具体实现。