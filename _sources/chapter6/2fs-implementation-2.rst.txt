简易文件系统 easy-fs (下)
=======================================


磁盘块管理器
---------------------------------------

本层的代码在 ``efs.rs`` 中。

.. code-block:: rust

    // easy-fs/src/efs.rs

    pub struct EasyFileSystem {
        pub block_device: Arc<dyn BlockDevice>,
        pub inode_bitmap: Bitmap,
        pub data_bitmap: Bitmap,
        inode_area_start_block: u32,
        data_area_start_block: u32,
    }

``EasyFileSystem`` 包含索引节点和数据块的两个位图 ``inode_bitmap`` 和 ``data_bitmap`` ，还记录下索引节点区域和数据块区域起始块编号方便确定每个索引节点和数据块在磁盘上的具体位置。我们还要在其中保留块设备的一个指针 ``block_device`` ，在进行后续操作的时候，该指针会被拷贝并传递给下层的数据结构，让它们也能够直接访问块设备。

通过 ``create`` 方法可以在块设备上创建并初始化一个 easy-fs 文件系统：

.. code-block:: rust
    :linenos:

    // easy-fs/src/efs.rs

    impl EasyFileSystem {
        pub fn create(
            block_device: Arc<dyn BlockDevice>,
            total_blocks: u32,
            inode_bitmap_blocks: u32,
        ) -> Arc<Mutex<Self>> {
            // calculate block size of areas & create bitmaps
            let inode_bitmap = Bitmap::new(1, inode_bitmap_blocks as usize);
            let inode_num = inode_bitmap.maximum();
            let inode_area_blocks =
                ((inode_num * core::mem::size_of::<DiskInode>() + BLOCK_SZ - 1) / BLOCK_SZ) as u32;
            let inode_total_blocks = inode_bitmap_blocks + inode_area_blocks;
            let data_total_blocks = total_blocks - 1 - inode_total_blocks;
            let data_bitmap_blocks = (data_total_blocks + 4096) / 4097;
            let data_area_blocks = data_total_blocks - data_bitmap_blocks;
            let data_bitmap = Bitmap::new(
                (1 + inode_bitmap_blocks + inode_area_blocks) as usize,
                data_bitmap_blocks as usize,
            );
            let mut efs = Self {
                block_device: Arc::clone(&block_device),
                inode_bitmap,
                data_bitmap,
                inode_area_start_block: 1 + inode_bitmap_blocks,
                data_area_start_block: 1 + inode_total_blocks + data_bitmap_blocks,
            };
            // clear all blocks
            for i in 0..total_blocks {
                get_block_cache(
                    i as usize, 
                    Arc::clone(&block_device)
                )
                .lock()
                .modify(0, |data_block: &mut DataBlock| {
                    for byte in data_block.iter_mut() { *byte = 0; }
                });
            }
            // initialize SuperBlock
            get_block_cache(0, Arc::clone(&block_device))
            .lock()
            .modify(0, |super_block: &mut SuperBlock| {
                super_block.initialize(
                    total_blocks,
                    inode_bitmap_blocks,
                    inode_area_blocks,
                    data_bitmap_blocks,
                    data_area_blocks,
                );
            });
            // write back immediately
            // create a inode for root node "/"
            assert_eq!(efs.alloc_inode(), 0);
            let (root_inode_block_id, root_inode_offset) = efs.get_disk_inode_pos(0);
            get_block_cache(
                root_inode_block_id as usize,
                Arc::clone(&block_device)
            )
            .lock()
            .modify(root_inode_offset, |disk_inode: &mut DiskInode| {
                disk_inode.initialize(DiskInodeType::Directory);
            });
            Arc::new(Mutex::new(efs))
        }
    }

- 第 10~21 行根据传入的参数计算每个区域各应该包含多少块。根据 inode 位图的大小计算 inode 区域至少需要多少个块才能够使得 inode 位图中的每个bit都能够有一个实际的 inode 可以对应，这样就确定了 inode 位图区域和 inode 区域的大小。剩下的块都分配给数据块位图区域和数据块区域。我们希望数据块位图中的每个bit仍然能够对应到一个数据块，但是数据块位图又不能过小，不然会造成某些数据块永远不会被使用。因此数据块位图区域最合理的大小是剩余的块数除以 4097 再上取整，因为位图中的每个块能够对应 4096 个数据块。其余的块就都作为数据块使用。
- 第 22 行创建我们的 ``EasyFileSystem`` 实例 ``efs`` 。
- 第 30 行首先将块设备的前 ``total_blocks`` 个块清零，因为我们的 easy-fs 要用到它们，这也是为初始化做准备。
- 第 41 行将位于块设备编号为 0 块上的超级块进行初始化，只需传入之前计算得到的每个区域的块数就行了。
- 第 54~63 行我们要做的事情是创建根目录 ``/`` 。首先需要调用 ``alloc_inode`` 在 inode 位图中分配一个 inode ，由于这是第一次分配，它的编号固定是 0 。接下来需要将分配到的 inode 初始化为 easy-fs 中的唯一一个目录，我们需要调用 ``get_disk_inode_pos`` 来根据 inode 编号获取该 inode 所在的块的编号以及块内偏移，之后就可以将它们传给 ``get_block_cache`` 和 ``modify`` 了。

通过 ``open`` 方法可以从一个已写入了 easy-fs 镜像的块设备上打开我们的 easy-fs ：

.. code-block:: rust

    // easy-fs/src/efs.rs

    impl EasyFileSystem {
        pub fn open(block_device: Arc<dyn BlockDevice>) -> Arc<Mutex<Self>> {
            // read SuperBlock
            get_block_cache(0, Arc::clone(&block_device))
                .lock()
                .read(0, |super_block: &SuperBlock| {
                    assert!(super_block.is_valid(), "Error loading EFS!");
                    let inode_total_blocks =
                        super_block.inode_bitmap_blocks + super_block.inode_area_blocks;
                    let efs = Self {
                        block_device,
                        inode_bitmap: Bitmap::new(
                            1,
                            super_block.inode_bitmap_blocks as usize
                        ),
                        data_bitmap: Bitmap::new(
                            (1 + inode_total_blocks) as usize,
                            super_block.data_bitmap_blocks as usize,
                        ),
                        inode_area_start_block: 1 + super_block.inode_bitmap_blocks,
                        data_area_start_block: 1 + inode_total_blocks + super_block.data_bitmap_blocks,
                    };
                    Arc::new(Mutex::new(efs))
                })        
        }
    }

它只需将块设备编号为 0 的块作为超级块读取进来，就可以从中知道 easy-fs 的磁盘布局，由此可以构造 ``efs`` 实例。

``EasyFileSystem`` 知道整个磁盘布局，即可以从 inode位图 或数据块位图上分配的 bit 编号，来算出各个存储inode和数据块的磁盘块在磁盘上的实际位置。

.. code-block:: rust

    // easy-fs/src/efs.rs

    impl EasyFileSystem {
        pub fn get_disk_inode_pos(&self, inode_id: u32) -> (u32, usize) {
            let inode_size = core::mem::size_of::<DiskInode>();
            let inodes_per_block = (BLOCK_SZ / inode_size) as u32;
            let block_id = self.inode_area_start_block + inode_id / inodes_per_block;
            (block_id, (inode_id % inodes_per_block) as usize * inode_size)
        }

        pub fn get_data_block_id(&self, data_block_id: u32) -> u32 {
            self.data_area_start_block + data_block_id
        }
    }

inode 和数据块的分配/回收也由它负责：

.. code-block:: rust

    // easy-fs/src/efs.rs

    impl EasyFileSystem {
        pub fn alloc_inode(&mut self) -> u32 {
            self.inode_bitmap.alloc(&self.block_device).unwrap() as u32
        }

        /// Return a block ID not ID in the data area.
        pub fn alloc_data(&mut self) -> u32 {
            self.data_bitmap.alloc(&self.block_device).unwrap() as u32 + self.data_area_start_block
        }

        pub fn dealloc_data(&mut self, block_id: u32) {
            get_block_cache(
                block_id as usize,
                Arc::clone(&self.block_device)
            )
            .lock()
            .modify(0, |data_block: &mut DataBlock| {
                data_block.iter_mut().for_each(|p| { *p = 0; })
            });
            self.data_bitmap.dealloc(
                &self.block_device,
                (block_id - self.data_area_start_block) as usize
            )
        }
    }

注意：

- ``alloc_data`` 和 ``dealloc_data`` 分配/回收数据块传入/返回的参数都表示数据块在块设备上的编号，而不是在数据块位图中分配的bit编号；
- ``dealloc_inode`` 未实现，不支持文件删除。

索引节点
---------------------------------------

服务于文件相关系统调用的索引节点层的代码在 ``vfs.rs`` 中。

``EasyFileSystem`` 实现了我们设计的磁盘布局并能够将所有块有效的管理起来。但是对于文件系统的使用者而言，他们往往不关心磁盘布局是如何实现的，而是更希望能够直接看到目录树结构中逻辑上的文件和目录。为此我们设计索引节点 ``Inode`` 暴露给文件系统的使用者，让他们能够直接对文件和目录进行操作。 ``Inode`` 和 ``DiskInode`` 的区别从它们的名字中就可以看出： ``DiskInode`` 放在磁盘块中比较固定的位置，而 ``Inode`` 是放在内存中的记录文件索引节点信息的数据结构。

.. code-block:: rust

    // easy-fs/src/vfs.rs

    pub struct Inode {
        block_id: usize,
        block_offset: usize,
        fs: Arc<Mutex<EasyFileSystem>>,
        block_device: Arc<dyn BlockDevice>,
    }

``block_id`` 和 ``block_offset`` 记录该 ``Inode`` 对应的 ``DiskInode`` 保存在磁盘上的具体位置方便我们后续对它进行访问。 ``fs`` 是指向 ``EasyFileSystem`` 的一个指针，因为对 ``Inode`` 的种种操作实际上都是要通过底层的文件系统来完成。

仿照 ``BlockCache::read/modify`` ，我们可以设计两个方法来简化对于 ``Inode`` 对应的磁盘上的 ``DiskInode`` 的访问流程，而不是每次都需要 ``get_block_cache.lock.read/modify`` ：

.. code-block:: rust

    // easy-fs/src/vfs.rs

    impl Inode {
        fn read_disk_inode<V>(&self, f: impl FnOnce(&DiskInode) -> V) -> V {
            get_block_cache(
                self.block_id,
                Arc::clone(&self.block_device)
            ).lock().read(self.block_offset, f)
        }

        fn modify_disk_inode<V>(&self, f: impl FnOnce(&mut DiskInode) -> V) -> V {
            get_block_cache(
                self.block_id,
                Arc::clone(&self.block_device)
            ).lock().modify(self.block_offset, f)
        }
    }

下面我们分别介绍文件系统的使用者对于文件系统的一些常用操作：

获取根目录的 inode
+++++++++++++++++++++++++++++++++++++++

文件系统的使用者在通过 ``EasyFileSystem::open`` 从装载了 easy-fs 镜像的块设备上打开 easy-fs 之后，要做的第一件事情就是获取根目录的 ``Inode`` 。因为我们目前仅支持绝对路径，对于任何文件/目录的索引都必须从根目录开始向下逐级进行。等到索引完成之后，我们才能对文件/目录进行操作。事实上 ``EasyFileSystem`` 提供了另一个名为 ``root_inode`` 的方法来获取根目录的 ``Inode`` :

.. code-block:: rust

    // easy-fs/src/efs.rs

    impl EasyFileSystem {
        pub fn root_inode(efs: &Arc<Mutex<Self>>) -> Inode {
            let block_device = Arc::clone(&efs.lock().block_device);
            // acquire efs lock temporarily
            let (block_id, block_offset) = efs.lock().get_disk_inode_pos(0);
            // release efs lock
            Inode::new(
                block_id,
                block_offset,
                Arc::clone(efs),
                block_device,
            )
        }
    }

    // easy-fs/src/vfs.rs

    impl Inode {
        /// We should not acquire efs lock here.
        pub fn new(
            block_id: u32,
            block_offset: usize,
            fs: Arc<Mutex<EasyFileSystem>>,
            block_device: Arc<dyn BlockDevice>,
        ) -> Self {
            Self {
                block_id: block_id as usize,
                block_offset,
                fs,
                block_device,
            }
        }
    }

在 ``root_inode`` 中，主要是在 ``Inode::new`` 的时候将传入的 ``inode_id`` 设置为 0 ，因为根目录对应于文件系统中第一个分配的 inode ，因此它的 ``inode_id`` 总会是 0 。同时在设计上，我们不会在 ``Inode::new`` 中尝试获取整个 ``EasyFileSystem`` 的锁来查询 inode 在块设备中的位置，而是在调用它之前预先查询并作为参数传过去。

文件索引
+++++++++++++++++++++++++++++++++++++++

为了尽可能简化我们的实现，所有的文件都在根目录下面。于是，我们不必实现目录索引。文件索引的查找比较简单，仅需在根目录的目录项中根据文件名找到文件的 inode 编号即可。由于没有子目录的存在，这个过程只会进行一次。

.. code-block:: rust

    // easy-fs/src/vfs.rs

    impl Inode {
        pub fn find(&self, name: &str) -> Option<Arc<Inode>> {
            let fs = self.fs.lock();
            self.read_disk_inode(|disk_inode| {
                self.find_inode_id(name, disk_inode)
                .map(|inode_id| {
                    let (block_id, block_offset) = fs.get_disk_inode_pos(inode_id);
                    Arc::new(Self::new(
                        block_id,
                        block_offset,
                        self.fs.clone(),
                        self.block_device.clone(),
                    ))
                })
            })
        }

        fn find_inode_id(
            &self,
            name: &str,
            disk_inode: &DiskInode,
        ) -> Option<u32> {
            // assert it is a directory
            assert!(disk_inode.is_dir());
            let file_count = (disk_inode.size as usize) / DIRENT_SZ;
            let mut dirent = DirEntry::empty();
            for i in 0..file_count {
                assert_eq!(
                    disk_inode.read_at(
                        DIRENT_SZ * i,
                        dirent.as_bytes_mut(),
                        &self.block_device,
                    ),
                    DIRENT_SZ,
                );
                if dirent.name() == name {
                    return Some(dirent.inode_number() as u32);
                }
            }
            None
        }
    }

``find`` 方法只会被根目录 ``Inode`` 调用，文件系统中其他文件的 ``Inode`` 不会调用这个方法。它首先调用 ``find_inode_id`` 方法尝试从根目录的 ``DiskInode`` 上找到要索引的文件名对应的 inode 编号。这就需要将根目录内容中的所有目录项都读到内存进行逐个比对。如果能够找到的话， ``find`` 方法会根据查到 inode 编号对应生成一个 ``Inode`` 用于后续对文件的访问。

这里需要注意的是，包括 ``find`` 在内所有暴露给文件系统的使用者的文件系统操作（还包括接下来将要介绍的几种），全程均需持有 ``EasyFileSystem`` 的互斥锁（相对的，文件系统内部的操作如之前的 ``Inode::new`` 或是上面的 ``find_inode_id`` 都是假定在已持有 efs 锁的情况下才被调用的，因此它们不应尝试获取锁）。这能够保证在多核情况下，同时最多只能有一个核在进行文件系统相关操作。这样也许会带来一些不必要的性能损失，但我们目前暂时先这样做。如果我们在这里加锁的话，其实就能够保证块缓存的互斥访问了。

文件列举
+++++++++++++++++++++++++++++++++++++++

``ls`` 方法可以收集根目录下的所有文件的文件名并以向量的形式返回，这个方法只有根目录的 ``Inode`` 才会调用：

.. code-block:: rust

    // easy-fs/src/vfs.rs

    impl Inode {
        pub fn ls(&self) -> Vec<String> {
            let _fs = self.fs.lock();
            self.read_disk_inode(|disk_inode| {
                let file_count = (disk_inode.size as usize) / DIRENT_SZ;
                let mut v: Vec<String> = Vec::new();
                for i in 0..file_count {
                    let mut dirent = DirEntry::empty();
                    assert_eq!(
                        disk_inode.read_at(
                            i * DIRENT_SZ,
                            dirent.as_bytes_mut(),
                            &self.block_device,
                        ),
                        DIRENT_SZ,
                    );
                    v.push(String::from(dirent.name()));
                }
                v
            })
        }
    }

文件创建
+++++++++++++++++++++++++++++++++++++++

``create`` 方法可以在根目录下创建一个文件，该方法只有根目录的 ``Inode`` 会调用：

.. code-block:: rust
    :linenos:

    // easy-fs/src/vfs.rs

    impl Inode {
        pub fn create(&self, name: &str) -> Option<Arc<Inode>> {
            let mut fs = self.fs.lock();
            if self.modify_disk_inode(|root_inode| {
                // assert it is a directory
                assert!(root_inode.is_dir());
                // has the file been created?
                self.find_inode_id(name, root_inode)
            }).is_some() {
                return None;
            }
            // create a new file
            // alloc a inode with an indirect block
            let new_inode_id = fs.alloc_inode();
            // initialize inode
            let (new_inode_block_id, new_inode_block_offset) 
                = fs.get_disk_inode_pos(new_inode_id);
            get_block_cache(
                new_inode_block_id as usize,
                Arc::clone(&self.block_device)
            ).lock().modify(new_inode_block_offset, |new_inode: &mut DiskInode| {
                new_inode.initialize(DiskInodeType::File);
            });
            self.modify_disk_inode(|root_inode| {
                // append file in the dirent
                let file_count = (root_inode.size as usize) / DIRENT_SZ;
                let new_size = (file_count + 1) * DIRENT_SZ;
                // increase size
                self.increase_size(new_size as u32, root_inode, &mut fs);
                // write dirent
                let dirent = DirEntry::new(name, new_inode_id);
                root_inode.write_at(
                    file_count * DIRENT_SZ,
                    dirent.as_bytes(),
                    &self.block_device,
                );
            });

            let (block_id, block_offset) = fs.get_disk_inode_pos(new_inode_id);
            // return inode
            Some(Arc::new(Self::new(
                block_id,
                block_offset,
                self.fs.clone(),
                self.block_device.clone(),
            )))
            // release efs lock automatically by compiler
        }
    }

- 第 6~13 行，检查文件是否已经在根目录下，如果找到的话返回 ``None`` ；
- 第 14~25 行，为待创建文件分配一个新的 inode 并进行初始化；
- 第 26~39 行，将待创建文件的目录项插入到根目录的内容中使得之后可以索引过来。

文件清空
+++++++++++++++++++++++++++++++++++++++

在以某些标志位打开文件（例如带有 *CREATE* 标志打开一个已经存在的文件）的时候，需要首先将文件清空。在索引到文件的 ``Inode`` 之后可以调用 ``clear`` 方法：

.. code-block:: rust

    // easy-fs/src/vfs.rs

    impl Inode {
        pub fn clear(&self) {
            let mut fs = self.fs.lock();
            self.modify_disk_inode(|disk_inode| {
                let size = disk_inode.size;
                let data_blocks_dealloc = disk_inode.clear_size(&self.block_device);
                assert!(data_blocks_dealloc.len() == DiskInode::total_blocks(size) as usize);
                for data_block in data_blocks_dealloc.into_iter() {
                    fs.dealloc_data(data_block);
                }
            });
        }
    }

这会将之前该文件占据的索引块和数据块在 ``EasyFileSystem`` 中回收。

文件读写
+++++++++++++++++++++++++++++++++++++++

从根目录索引到一个文件之后可以对它进行读写，注意，和 ``DiskInode`` 一样，这里的读写作用在字节序列的一段区间上：

.. code-block:: rust

    // easy-fs/src/vfs.rs

    impl Inode {
        pub fn read_at(&self, offset: usize, buf: &mut [u8]) -> usize {
            let _fs = self.fs.lock();
            self.read_disk_inode(|disk_inode| {
                disk_inode.read_at(offset, buf, &self.block_device)
            })
        }

        pub fn write_at(&self, offset: usize, buf: &[u8]) -> usize {
            let mut fs = self.fs.lock();
            self.modify_disk_inode(|disk_inode| {
                self.increase_size((offset + buf.len()) as u32, disk_inode, &mut fs);
                disk_inode.write_at(offset, buf, &self.block_device)
            })
        }
    }

具体实现比较简单，需要注意在 ``DiskInode::write_at`` 之前先调用 ``increase_size`` 对自身进行扩容：

.. code-block:: rust

    // easy-fs/src/vfs.rs

    impl Inode {
        fn increase_size(
            &self,
            new_size: u32,
            disk_inode: &mut DiskInode,
            fs: &mut MutexGuard<EasyFileSystem>,
        ) {
            if new_size < disk_inode.size {
                return;
            }
            let blocks_needed = disk_inode.blocks_num_needed(new_size);
            let mut v: Vec<u32> = Vec::new();
            for _ in 0..blocks_needed {
                v.push(fs.alloc_data());
            }
            disk_inode.increase_size(new_size, v, &self.block_device);
        }
    }

这里会从 ``EasyFileSystem`` 中分配一些用于扩容的数据块并传给 ``DiskInode::increase_size`` 。

将应用打包为 easy-fs 镜像
---------------------------------------

在第六章中我们需要将所有的应用都链接到内核中，随后在应用管理器中通过应用名进行索引来找到应用的 ELF 数据。这样做有一个缺点，就是会造成内核体积过度膨胀。同时这也会浪费内存资源，因为未被执行的应用也占据了内存空间。在实现了我们自己的文件系统之后，终于可以将这些应用打包到 easy-fs 镜像中放到磁盘中，当我们要执行应用的时候只需从文件系统中取出ELF 执行文件格式的应用 并加载到内存中执行即可，这样就避免了上面的那些问题。

``easy-fs-fuse`` 的主体 ``easy-fs-pack`` 函数就实现了这个功能：

.. code-block:: rust
    :linenos:

    // easy-fs-fuse/src/main.rs

    use clap::{Arg, App};

    fn easy_fs_pack() -> std::io::Result<()> {
        let matches = App::new("EasyFileSystem packer")
            .arg(Arg::with_name("source")
                .short("s")
                .long("source")
                .takes_value(true)
                .help("Executable source dir(with backslash)")
            )
            .arg(Arg::with_name("target")
                .short("t")
                .long("target")
                .takes_value(true)
                .help("Executable target dir(with backslash)")    
            )
            .get_matches();
        let src_path = matches.value_of("source").unwrap();
        let target_path = matches.value_of("target").unwrap();
        println!("src_path = {}\ntarget_path = {}", src_path, target_path);
        let block_file = Arc::new(BlockFile(Mutex::new({
            let f = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(format!("{}{}", target_path, "fs.img"))?;
            f.set_len(8192 * 512).unwrap();
            f
        })));
        // 4MiB, at most 4095 files
        let efs = EasyFileSystem::create(
            block_file.clone(),
            8192,
            1,
        );
        let root_inode = Arc::new(EasyFileSystem::root_inode(&efs));
        let apps: Vec<_> = read_dir(src_path)
            .unwrap()
            .into_iter()
            .map(|dir_entry| {
                let mut name_with_ext = dir_entry.unwrap().file_name().into_string().unwrap();
                name_with_ext.drain(name_with_ext.find('.').unwrap()..name_with_ext.len());
                name_with_ext
            })
            .collect();
        for app in apps {
            // load app data from host file system
            let mut host_file = File::open(format!("{}{}", target_path, app)).unwrap();
            let mut all_data: Vec<u8> = Vec::new();
            host_file.read_to_end(&mut all_data).unwrap();
            // create a file in easy-fs
            let inode = root_inode.create(app.as_str()).unwrap();
            // write data to easy-fs
            inode.write_at(0, all_data.as_slice());
        }
        // list apps
        for app in root_inode.ls() {
            println!("{}", app);
        }
        Ok(())
    }

- 为了实现 ``easy-fs-fuse`` 和 ``os/user`` 的解耦，第 6~21 行使用 ``clap`` crate 进行命令行参数解析，需要通过 ``-s`` 和 ``-t`` 分别指定应用的源代码目录和保存应用 ELF 的目录而不是在 ``easy-fs-fuse`` 中硬编码。如果解析成功的话它们会分别被保存在变量 ``src_path`` 和 ``target_path`` 中。
- 第 23~38 行依次完成：创建 4MiB 的 easy-fs 镜像文件、进行 easy-fs 初始化、获取根目录 inode 。
- 第 39 行获取源码目录中的每个应用的源代码文件并去掉后缀名，收集到向量 ``apps`` 中。
- 第 48 行开始，枚举 ``apps`` 中的每个应用，从放置应用执行程序的目录中找到对应应用的 ELF 文件（这是一个 HostOS 上的文件）并将数据读入内存。接着需要在我们的 easy-fs 中创建一个同名文件并将 ELF 数据写入到这个文件中。这个过程相当于将 HostOS 上的文件系统中的一个文件复制到我们的 easy-fs 中。

尽管没有进行任何同步写回磁盘的操作，我们也不用担心块缓存中的修改没有写回磁盘。因为在 ``easy-fs-fuse`` 这个应用正常退出的过程中，块缓存因生命周期结束会被回收，届时如果 ``modified`` 标志为 true 就会将修改写回磁盘。