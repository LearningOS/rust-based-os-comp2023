多道程序放置与加载
=====================================

多道程序放置
----------------------------


在第二章中，内核让所有应用都共享同一个固定的起始地址。
正因如此，内存中同时最多只能驻留一个应用，

要一次加载运行多个程序，就要求每个用户程序被内核加载到内存中的起始地址都不同。
为此，我们编写脚本 ``user/build.py`` 为每个应用定制各自的起始地址。
它的思路很简单，对于每一个应用程序，使用 ``cargo rustc`` 单独编译，
用 ``-Clink-args=-Ttext=xxxx`` 选项指定链接时 .text 段的地址为 ``0x80400000 + app_id * 0x20000`` 。

.. note::

    qemu 预留的内存空间是有限的，如果加载的程序过多，程序地址超出内存空间，可能出现 ``core dumped``.

多道程序加载
----------------------------

在第二章中负责应用加载和执行的子模块 ``batch`` 被拆分为 ``loader`` 和 ``task`` ，
前者负责启动时加载应用程序，后者负责切换和调度。

其中， ``loader`` 模块的 ``load_apps`` 函数负责将所有用户程序在内核初始化的时一并加载进内存。

.. code-block:: rust
   :linenos:

    // os/src/loader.rs

    pub fn load_apps() {
        extern "C" {
            fn _num_app();
        }
        let num_app_ptr = _num_app as usize as *const usize;
        let num_app = get_num_app();
        let app_start = unsafe { core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1) };
        // clear i-cache first
        unsafe {
            core::arch::asm!("fence.i");
        }
        // load apps
        for i in 0..num_app {
            let base_i = get_base_i(i);
            // clear region
            (base_i..base_i + APP_SIZE_LIMIT)
                .for_each(|addr| unsafe { (addr as *mut u8).write_volatile(0) });
            // load app from data section to memory
            let src = unsafe {
                core::slice::from_raw_parts(app_start[i] as *const u8, app_start[i + 1] - app_start[i])
            };
            let dst = unsafe { core::slice::from_raw_parts_mut(base_i as *mut u8, src.len()) };
            dst.copy_from_slice(src);
        }
    }

第 :math:`i` 个应用被加载到以物理地址 ``base_i`` 开头的一段物理内存上，而 ``base_i`` 的计算方式如下：

.. code-block:: rust
   :linenos:

    // os/src/loader.rs

    fn get_base_i(app_id: usize) -> usize {
        APP_BASE_ADDRESS + app_id * APP_SIZE_LIMIT
    }

我们可以在 ``config`` 子模块中找到这两个常数， ``APP_BASE_ADDRESS`` 被设置为 ``0x80400000`` ，
而 ``APP_SIZE_LIMIT`` 和上一章一样被设置为 ``0x20000`` 。这种放置方式与 ``user/build.py`` 的实现一致。
