在内核中使用 easy-fs
===============================================

块设备驱动层
-----------------------------------------------

在 ``drivers`` 子模块中的 ``block/mod.rs`` 中，我们可以找到内核访问的块设备实例 ``BLOCK_DEVICE`` ：

.. code-block:: rust

    // os/src/drivers/block/mod.rs

    type BlockDeviceImpl = virtio_blk::VirtIOBlock;

    lazy_static! {
        pub static ref BLOCK_DEVICE: Arc<dyn BlockDevice> = Arc::new(BlockDeviceImpl::new());
    }

在 qemu 上，我们使用 ``VirtIOBlock`` 访问 VirtIO 块设备，并将它全局实例化为 ``BLOCK_DEVICE`` ，使内核的其他模块可以访问。

在启动 Qemu 模拟器的时候，我们可以配置参数来添加一块 VirtIO 块设备：

.. code-block:: makefile
    :linenos:
    :emphasize-lines: 11-12

    # os/Makefile

    FS_IMG := ../user/target/$(TARGET)/$(MODE)/fs.img

    run: build
        @qemu-system-riscv64 \
            -machine virt \
            -nographic \
            -bios $(BOOTLOADER) \
            -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) \
            -drive file=$(FS_IMG),if=none,format=raw,id=x0 \
            -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0

- 第 11 行，我们为虚拟机添加一块虚拟硬盘，内容为我们之前通过 ``easy-fs-fuse`` 工具打包的包含应用 ELF 的 easy-fs 镜像，并命名为 ``x0`` 。
- 第 12 行，我们将硬盘 ``x0`` 作为一个 VirtIO 总线中的一个块设备接入到虚拟机系统中。 ``virtio-mmio-bus.0`` 表示 VirtIO 总线通过 MMIO 进行控制，且该块设备在总线中的编号为 0 。

**内存映射 I/O** (MMIO, Memory-Mapped I/O) 指通过特定的物理内存地址来访问外设的设备寄存器。查阅资料，可知 VirtIO 总线的 MMIO 物理地址区间为从 0x10001000 开头的 4KiB 。

在 ``config`` 子模块中我们硬编码 Qemu 上的 VirtIO 总线的 MMIO 地址区间（起始地址，长度）。在创建内核地址空间的时候需要建立页表映射：

.. code-block:: rust

    // os/src/config.rs

    pub const MMIO: &[(usize, usize)] = &[
        (0x10001000, 0x1000),
    ];

    // os/src/mm/memory_set.rs

    use crate::config::MMIO;

    impl MemorySet {
        /// Without kernel stacks.
        pub fn new_kernel() -> Self {
            ...
            println!("mapping memory-mapped registers");
            for pair in MMIO {
                memory_set.push(MapArea::new(
                    (*pair).0.into(),
                    ((*pair).0 + (*pair).1).into(),
                    MapType::Identical,
                    MapPermission::R | MapPermission::W,
                ), None);
            }
            memory_set
        }
    }

这里我们进行的是透明的恒等映射，让内核可以兼容于直接访问物理地址的设备驱动库。

由于设备驱动的开发过程比较琐碎，我们这里直接使用已有的 `virtio-drivers <https://github.com/rcore-os/virtio-drivers>`_ crate，感兴趣的同学可以自行了解。


内核索引节点层
-----------------------------------------------

内核将 ``easy-fs`` 提供的 ``Inode`` 进一步封装为 OS 中的索引节点 ``OSInode`` 。

.. code-block:: rust

    // os/src/fs/inode.rs

    pub struct OSInode {
        readable: bool,
        writable: bool,
        inner: UPSafeCell<OSInodeInner>,
    }

    pub struct OSInodeInner {
        offset: usize,
        inode: Arc<Inode>,
    }

``OSInode`` 就表示进程中一个被打开的常规文件或目录。 ``readable/writable`` 分别表明该文件是否允许通过 ``sys_read/write`` 进行读写，读写过程中的偏移量 ``offset`` 和 ``Inode`` 则加上互斥锁丢到 ``OSInodeInner`` 中。

文件描述符层
-----------------------------------------------

``OSInode`` 也是要一种要放到进程文件描述符表中，通过 ``sys_read/write`` 进行读写的文件，我们需要为它实现 ``File`` Trait ：

.. code-block:: rust

    // os/src/fs/inode.rs

    impl File for OSInode {
        fn readable(&self) -> bool { self.readable }
        fn writable(&self) -> bool { self.writable }
        fn read(&self, mut buf: UserBuffer) -> usize {
            let mut inner = self.inner.lock();
            let mut total_read_size = 0usize;
            for slice in buf.buffers.iter_mut() {
                let read_size = inner.inode.read_at(inner.offset, *slice);
                if read_size == 0 {
                    break;
                }
                inner.offset += read_size;
                total_read_size += read_size;
            }
            total_read_size
        }
        fn write(&self, buf: UserBuffer) -> usize {
            let mut inner = self.inner.lock();
            let mut total_write_size = 0usize;
            for slice in buf.buffers.iter() {
                let write_size = inner.inode.write_at(inner.offset, *slice);
                assert_eq!(write_size, slice.len());
                inner.offset += write_size;
                total_write_size += write_size;
            }
            total_write_size
        }
    }

``read/write`` 的实现也比较简单，只需遍历 ``UserBuffer`` 中的每个缓冲区片段，调用 ``Inode`` 写好的 ``read/write_at`` 接口就好了。注意 ``read/write_at`` 的起始位置是在 ``OSInode`` 中维护的 ``offset`` ，这个 ``offset`` 也随着遍历的进行被持续更新。在 ``read/write`` 的全程需要获取 ``OSInode`` 的互斥锁，保证两个进程无法同时访问同个文件。

本章我们为 ``File`` Trait 新增了 ``readable/writable`` 两个抽象接口，从而在 ``sys_read/sys_write`` 的时候进行简单的访问权限检查。 

文件系统相关内核机制实现
-----------------------------------------------

文件系统初始化
+++++++++++++++++++++++++++++++++++++++++++++++

为了使用 ``easy-fs`` 提供的抽象，内核需要进行一些初始化操作。我们需要从块设备 ``BLOCK_DEVICE`` 上打开文件系统，并从文件系统中获取根目录的 inode 。


.. code-block:: rust

    // os/src/fs/inode.rs

    lazy_static! {
        pub static ref ROOT_INODE: Arc<Inode> = {
            let efs = EasyFileSystem::open(BLOCK_DEVICE.clone());
            Arc::new(EasyFileSystem::root_inode(&efs))
        };
    }

这之后就可以使用根目录的 inode ``ROOT_INODE`` ，在内核中调用 ``easy-fs`` 的相关接口了。例如，在文件系统初始化完毕之后，调用 ``list_apps`` 函数来打印所有可用应用的文件名：

.. code-block:: rust

    // os/src/fs/inode.rs

    pub fn list_apps() {
        println!("/**** APPS ****");
        for app in ROOT_INODE.ls() {
            println!("{}", app);
        }
        println!("**************/")
    }


通过 sys_open 打开文件
+++++++++++++++++++++++++++++++++++++++++++++++

在内核中也定义一份打开文件的标志 ``OpenFlags`` ：

.. code-block:: rust

    // os/src/fs/inode.rs

    bitflags! {
        pub struct OpenFlags: u32 {
            const RDONLY = 0;
            const WRONLY = 1 << 0;
            const RDWR = 1 << 1;
            const CREATE = 1 << 9;
            const TRUNC = 1 << 10;
        }
    }

    impl OpenFlags {
        /// Do not check validity for simplicity
        /// Return (readable, writable)
        pub fn read_write(&self) -> (bool, bool) {
            if self.is_empty() {
                (true, false)
            } else if self.contains(Self::WRONLY) {
                (false, true)
            } else {
                (true, true)
            }
        }
    }

它的 ``read_write`` 方法可以根据标志的情况返回要打开的文件是否允许读写。简单起见，这里假设标志自身一定合法。

接着，我们实现 ``open_file`` 内核函数，可根据文件名打开一个根目录下的文件：

.. code-block:: rust

    // os/src/fs/inode.rs

    pub fn open_file(name: &str, flags: OpenFlags) -> Option<Arc<OSInode>> {
        let (readable, writable) = flags.read_write();
        if flags.contains(OpenFlags::CREATE) {
            if let Some(inode) = ROOT_INODE.find(name) {
                // clear size
                inode.clear();
                Some(Arc::new(OSInode::new(
                    readable,
                    writable,
                    inode,
                )))
            } else {
                // create file
                ROOT_INODE.create(name)
                    .map(|inode| {
                        Arc::new(OSInode::new(
                            readable,
                            writable,
                            inode,
                        ))
                    })
            }
        } else {
            ROOT_INODE.find(name)
                .map(|inode| {
                    if flags.contains(OpenFlags::TRUNC) {
                        inode.clear();
                    }
                    Arc::new(OSInode::new(
                        readable,
                        writable,
                        inode
                    ))
                })
        }
    }

这里主要是实现了 ``OpenFlags`` 各标志位的语义。例如只有 ``flags`` 参数包含 `CREATE` 标志位才允许创建文件；而如果文件已经存在，则清空文件的内容。

在其基础上， ``sys_open`` 也就很容易实现了。

通过 sys_exec 加载并执行应用
+++++++++++++++++++++++++++++++++++++++++++++++

有了文件系统支持后， ``sys_exec`` 所需的表示应用 ELF 格式数据改为从文件系统中获取：

.. code-block:: rust
    :linenos:
    :emphasize-lines: 17-25

    // os/src/syscall/process.rs

    pub fn sys_exec(path: *const u8, mut args: *const usize) -> isize {
    let token = current_user_token();
    let path = translated_str(token, path);
    let mut args_vec: Vec<String> = Vec::new();
    loop {
        let arg_str_ptr = *translated_ref(token, args);
        if arg_str_ptr == 0 {
            break;
        }
        args_vec.push(translated_str(token, arg_str_ptr as *const u8));
        unsafe {
            args = args.add(1);
        }
    }
    if let Some(app_inode) = open_file(path.as_str(), OpenFlags::RDONLY) {
        let all_data = app_inode.read_all();
        let task = current_task().unwrap();
        let argc = args_vec.len();
        task.exec(all_data.as_slice(), args_vec);
        argc as isize
    } else {
        -1
    }

注意上面代码片段中的高亮部分。当执行获取应用的 ELF 数据的操作时，首先调用 ``open_file`` 函数，以只读的方式在内核中打开应用文件并获取它对应的 ``OSInode`` 。接下来可以通过 ``OSInode::read_all`` 将该文件的数据全部读到一个向量 ``all_data`` 中：

之后，就可以从向量 ``all_data`` 中拿到应用中的 ELF 数据，当解析完毕并创建完应用地址空间后该向量将会被回收。

同样的，我们在内核中创建初始进程 ``initproc`` 也需要替换为基于文件系统的实现：

.. code-block:: rust

    // os/src/task/mod.rs

    lazy_static! {
        pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new({
            let inode = open_file("ch6b_initproc", OpenFlags::RDONLY).unwrap();
            let v = inode.read_all();
            TaskControlBlock::new(v.as_slice())
        });
    }
