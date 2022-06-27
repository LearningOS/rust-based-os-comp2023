实现 SV39 多级页表机制（下）
========================================================

物理页帧管理
-----------------------------------

可用物理页的分配与回收
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

首先，我们需要知道物理内存的哪一部分是可用的。在 ``os/src/linker.ld`` 中，我们用符号 ``ekernel`` 指明了
内核数据的终止物理地址，在它之后的物理内存都是可用的。而在 ``config`` 子模块中：

.. code-block:: rust

    // os/src/config.rs

    pub const MEMORY_END: usize = 0x80800000;

我们硬编码整块物理内存的终止物理地址为 ``0x80800000`` 。 而物理内存的起始物理地址为 ``0x80000000`` ，
意味着我们将可用内存大小设置为 :math:`8\text{MiB}` ，当然也可以设置的更大一点。

用一个左闭右开的物理页号区间来表示可用的物理内存，则：

- 区间的左端点应该是 ``ekernel`` 的物理地址以上取整方式转化成的物理页号；
- 区间的右端点应该是 ``MEMORY_END`` 以下取整方式转化成的物理页号。

这个区间将被传给我们后面实现的物理页帧管理器用于初始化。

我们声明一个 ``FrameAllocator`` Trait 来描述一个物理页帧管理器需要提供哪些功能：

.. code-block:: rust

    // os/src/mm/frame_allocator.rs

    trait FrameAllocator {
        fn new() -> Self;
        fn alloc(&mut self) -> Option<PhysPageNum>;
        fn dealloc(&mut self, ppn: PhysPageNum);
    }

我们实现一种最简单的栈式物理页帧管理策略 ``StackFrameAllocator`` ：

.. code-block:: rust

    // os/src/mm/frame_allocator.rs

    pub struct StackFrameAllocator {
        current: usize,
        end: usize,
        recycled: Vec<usize>,
    }

其中各字段的含义是：物理页号区间 :math:`[\text{current},\text{end})` 此前均 *从未* 被分配出去过，而向量
``recycled`` 以后入先出的方式保存了被回收的物理页号（我们已经实现了堆分配器，参见第三章实验）。

初始化非常简单。在通过 ``FrameAllocator`` 的 ``new`` 方法创建实例的时候，只需将区间两端均设为 :math:`0` ，
然后创建一个新的向量；而在它真正被使用起来之前，需要调用 ``init`` 方法将自身的 :math:`[\text{current},\text{end})`
初始化为可用物理页号区间：

.. code-block:: rust

    // os/src/mm/frame_allocator.rs

    impl FrameAllocator for StackFrameAllocator {
        fn new() -> Self {
            Self {
                current: 0,
                end: 0,
                recycled: Vec::new(),
            }
        }
    }

    impl StackFrameAllocator {
        pub fn init(&mut self, l: PhysPageNum, r: PhysPageNum) {
            self.current = l.0;
            self.end = r.0;
        }
    }

接下来我们来看核心的物理页帧分配和回收如何实现：

.. code-block:: rust

    // os/src/mm/frame_allocator.rs

    impl FrameAllocator for StackFrameAllocator {
        fn alloc(&mut self) -> Option<PhysPageNum> {
            if let Some(ppn) = self.recycled.pop() {
                Some(ppn.into())
            } else {
                if self.current == self.end {
                    None
                } else {
                    self.current += 1;
                    Some((self.current - 1).into())
                }
            }
        }
        fn dealloc(&mut self, ppn: PhysPageNum) {
            let ppn = ppn.0;
            // validity check
            if ppn >= self.current || self.recycled
                .iter()
                .find(|&v| {*v == ppn})
                .is_some() {
                panic!("Frame ppn={:#x} has not been allocated!", ppn);
            }
            // recycle
            self.recycled.push(ppn);
        }
    }

- 在分配 ``alloc`` 的时候，首先会检查栈 ``recycled`` 内有没有之前回收的物理页号，如果有的话直接弹出栈顶并返回；
  否则的话我们只能从之前从未分配过的物理页号区间 :math:`[\text{current},\text{end})` 上进行分配，我们分配它的
  左端点 ``current`` ，同时将管理器内部维护的 ``current`` 加一代表 ``current`` 此前已经被分配过了。在即将返回
  的时候，我们使用 ``into`` 方法将 usize 转换成了物理页号 ``PhysPageNum`` 。

  注意极端情况下可能出现内存耗尽分配失败的情况：即 ``recycled`` 为空且 :math:`\text{current}==\text{end}` 。
  为了涵盖这种情况， ``alloc`` 的返回值被 ``Option`` 包裹，我们返回 ``None`` 即可。
- 在回收 ``dealloc`` 的时候，我们需要检查回收页面的合法性，然后将其压入 ``recycled`` 栈中。回收页面合法有两个
  条件：

  - 该页面之前一定被分配出去过，因此它的物理页号一定 :math:`<\text{current}` ；
  - 该页面没有正处在回收状态，即它的物理页号不能在栈 ``recycled`` 中找到。

  我们通过 ``recycled.iter()`` 获取栈上内容的迭代器，然后通过迭代器的 ``find`` 方法试图
  寻找一个与输入物理页号相同的元素。其返回值是一个 ``Option`` ，如果找到了就会是一个 ``Option::Some`` ，
  这种情况说明我们内核其他部分实现有误，直接报错退出。

之后创建 ``StackFrameAllocator`` 的全局实例 ``FRAME_ALLOCATOR``，并在正式分配物理页帧之前将 ``FRAME_ALLOCATOR`` 初始化，见 ``os/src/mm/frame_allocator.rs``。

分配/回收物理页帧的接口
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

公开给其他子模块调用的分配/回收物理页帧的接口：

.. code-block:: rust

    // os/src/mm/frame_allocator.rs

    pub fn frame_alloc() -> Option<FrameTracker> {
        FRAME_ALLOCATOR
            .exclusive_access()
            .alloc()
            .map(FrameTracker::new)
    }

    fn frame_dealloc(ppn: PhysPageNum) {
        FRAME_ALLOCATOR.exclusive_access().dealloc(ppn);
    }


可以发现， ``frame_alloc`` 的返回值类型并不是 ``FrameAllocator`` 要求的物理页号 ``PhysPageNum`` ，而是将其
进一步包装为一个 ``FrameTracker`` ，其定义如下。 ``FrameTracker`` 被创建时，需要从 ``FRAME_ALLOCATOR`` 中分配一个物理页帧：

.. code-block:: rust

    // os/src/mm/frame_allocator.rs

    pub struct FrameTracker {
        pub ppn: PhysPageNum,
    }

    impl FrameTracker {
        pub fn new(ppn: PhysPageNum) -> Self {
            // page cleaning
            let bytes_array = ppn.get_bytes_array();
            for i in bytes_array {
                *i = 0;
            }
            Self { ppn }
        }
    }

我们将分配来的物理页帧的物理页号作为参数传给 ``FrameTracker`` 的 ``new`` 方法来创建一个 ``FrameTracker``
实例。由于这个物理页帧之前可能被分配过并用做其他用途，我们在这里直接将这个物理页帧上的所有字节清零。这一过程并不
那么显然，我们后面再详细介绍。

当一个 ``FrameTracker`` 生命周期结束被编译器回收的时候，我们需要将它控制的物理页帧回收掉 ``FRAME_ALLOCATOR`` 中：

.. code-block:: rust

    // os/src/mm/frame_allocator.rs

    impl Drop for FrameTracker {
        fn drop(&mut self) {
            frame_dealloc(self.ppn);
        }
    }

这里我们只需为 ``FrameTracker`` 实现 ``Drop`` Trait 即可。当一个 ``FrameTracker`` 实例被回收的时候，它的
``drop`` 方法会自动被编译器调用，通过之前实现的 ``frame_dealloc`` 我们就将它控制的物理页帧回收以供后续使用了。

最后做一个小结：从其他模块的视角看来，物理页帧分配的接口是调用 ``frame_alloc`` 函数得到一个 ``FrameTracker``
（如果物理内存还有剩余），它就代表了一个物理页帧，当它的生命周期结束之后它所控制的物理页帧将被自动回收。

多级页表实现
-----------------------------------


页表基本数据结构与访问接口
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

我们知道，SV39 多级页表是以节点为单位进行管理的。每个节点恰好存储在一个物理页帧中，它的位置可以用一个物理页号来表示。

.. code-block:: rust
    :linenos:

    // os/src/mm/page_table.rs

    pub struct PageTable {
        root_ppn: PhysPageNum,
        frames: Vec<FrameTracker>,
    }

    impl PageTable {
        pub fn new() -> Self {
            let frame = frame_alloc().unwrap();
            PageTable {
                root_ppn: frame.ppn,
                frames: vec![frame],
            }
        }
    }

每个应用的地址空间都对应一个不同的多级页表，这也就意味这不同页表的起始地址（即页表根节点的地址）是不一样的。
因此 ``PageTable`` 要保存它根节点的物理页号 ``root_ppn`` 作为页表唯一的区分标志。此外，
向量 ``frames`` 以 ``FrameTracker`` 的形式保存了页表所有的节点（包括根节点）所在的物理页帧。这与物理页帧管理模块
的测试程序是一个思路，即将这些 ``FrameTracker`` 的生命周期进一步绑定到 ``PageTable`` 下面。当 ``PageTable``
生命周期结束后，向量 ``frames`` 里面的那些 ``FrameTracker`` 也会被回收，也就意味着存放多级页表节点的那些物理页帧
被回收了。

当我们通过 ``new`` 方法新建一个 ``PageTable`` 的时候，它只需有一个根节点。为此我们需要分配一个物理页帧
``FrameTracker`` 并挂在向量 ``frames`` 下，然后更新根节点的物理页号 ``root_ppn`` 。

多级页表并不是被创建出来之后就不再变化的，为了 MMU 能够通过地址转换正确找到应用地址空间中的数据实际被内核放在内存中
位置，操作系统需要动态维护一个虚拟页号到页表项的映射，支持插入/删除键值对，其方法签名如下：

.. code-block:: rust

    // os/src/mm/page_table.rs

    impl PageTable {
        pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags);
        pub fn unmap(&mut self, vpn: VirtPageNum);
    }

- 我们通过 ``map`` 方法来在多级页表中插入一个键值对，注意这里我们将物理页号 ``ppn`` 和页表项标志位 ``flags`` 作为
  不同的参数传入而不是整合为一个页表项；
- 相对的，我们通过 ``unmap`` 方法来删除一个键值对，在调用时仅需给出作为索引的虚拟页号即可。

.. _modify-page-table:

在这些操作的过程中，我们自然需要访问或修改多级页表节点的内容。每个节点都被保存在一个物理页帧中，在多级页表的架构中，我们以
一个节点被存放在的物理页帧的物理页号作为指针指向该节点，这意味着，对于每个节点来说，一旦我们知道了指向它的物理页号，我们
就能够修改这个节点的内容。

.. _term-identical-mapping:

这就需要我们提前扩充多级页表维护的映射，使得对于每一个对应于某一特定物理页帧的物理页号 ``ppn`` ，均存在一个虚拟页号
``vpn`` 能够映射到它，而且要能够较为简单的针对一个 ``ppn`` 找到某一个能映射到它的 ``vpn`` 。这里我们采用一种最
简单的 **恒等映射** (Identical Mapping) ，也就是说对于物理内存上的每个物理页帧，我们都在多级页表中用一个与其
物理页号相等的虚拟页号映射到它。当我们想针对物理页号构造一个能映射到它的虚拟页号的时候，也只需使用一个和该物理页号
相等的虚拟页号即可。


内核中访问物理页帧的方法
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. _access-frame-in-kernel-as:


于是，我们来看看在内核中应如何访问一个特定的物理页帧：

.. code-block:: rust

    // os/src/mm/address.rs

    impl PhysPageNum {
        pub fn get_pte_array(&self) -> &'static mut [PageTableEntry] {
            let pa: PhysAddr = self.clone().into();
            unsafe {
                core::slice::from_raw_parts_mut(pa.0 as *mut PageTableEntry, 512)
            }
        }
        pub fn get_bytes_array(&self) -> &'static mut [u8] {
            let pa: PhysAddr = self.clone().into();
            unsafe {
                core::slice::from_raw_parts_mut(pa.0 as *mut u8, 4096)
            }
        }
        pub fn get_mut<T>(&self) -> &'static mut T {
            let pa: PhysAddr = self.clone().into();
            unsafe {
                (pa.0 as *mut T).as_mut().unwrap()
            }
        }
    }

我们构造可变引用来直接访问一个物理页号 ``PhysPageNum`` 对应的物理页帧，不同的引用类型对应于物理页帧上的一种不同的
内存布局，如 ``get_pte_array`` 返回的是一个页表项定长数组的可变引用，可以用来修改多级页表中的一个节点；而
``get_bytes_array`` 返回的是一个字节数组的可变引用，可以以字节为粒度对物理页帧上的数据进行访问，前面进行数据清零
就用到了这个方法； ``get_mut`` 是个泛型函数，可以获取一个恰好放在一个物理页帧开头的类型为 ``T`` 的数据的可变引用。

在实现方面，都是先把物理页号转为物理地址 ``PhysAddr`` ，然后再转成 usize 形式的物理地址。接着，我们直接将它
转为裸指针用来访问物理地址指向的物理内存。在分页机制开启前，这样做自然成立；而开启之后，虽然裸指针被视为一个虚拟地址，
但是上面已经提到这种情况下虚拟地址会映射到一个相同的物理地址，因此在这种情况下也成立。注意，我们在返回值类型上附加了
静态生命周期泛型 ``'static`` ，这是为了绕过 Rust 编译器的借用检查，实质上可以将返回的类型也看成一个裸指针，因为
它也只是标识数据存放的位置以及类型。但与裸指针不同的是，无需通过 ``unsafe`` 的解引用访问它指向的数据，而是可以像一个
正常的可变引用一样直接访问。


建立和拆除虚实地址映射关系
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

接下来介绍建立和拆除虚实地址映射关系的 ``map`` 和 ``unmap`` 方法是如何实现的。它们都依赖于一个很重要的过程，
也即在多级页表中找到一个虚拟地址对应的页表项。找到之后，只要修改页表项的内容即可完成键值对的插入和删除。
在寻找页表项的时候，可能出现页表的中间级节点还未被创建的情况，这个时候我们需要手动分配一个物理页帧来存放这个节点，
并将这个节点接入到当前的多级页表的某级中。


.. code-block:: rust
    :linenos:

    // os/src/mm/address.rs

    impl VirtPageNum {
        pub fn indexes(&self) -> [usize; 3] {
            let mut vpn = self.0;
            let mut idx = [0usize; 3];
            for i in (0..3).rev() {
                idx[i] = vpn & 511;
                vpn >>= 9;
            }
            idx
        }
    }

    // os/src/mm/page_table.rs

    impl PageTable {
        fn find_pte_create(&mut self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
            let idxs = vpn.indexes();
            let mut ppn = self.root_ppn;
            let mut result: Option<&mut PageTableEntry> = None;
            for i in 0..3 {
                let pte = &mut ppn.get_pte_array()[idxs[i]];
                if i == 2 {
                    result = Some(pte);
                    break;
                }
                if !pte.is_valid() {
                    let frame = frame_alloc().unwrap();
                    *pte = PageTableEntry::new(frame.ppn, PTEFlags::V);
                    self.frames.push(frame);
                }
                ppn = pte.ppn();
            }
            result
        }
    }

- ``VirtPageNum`` 的 ``indexes`` 可以取出虚拟页号的三级页索引，并按照从高到低的顺序返回。注意它里面包裹的
  usize 可能有 :math:`27` 位，也有可能有 :math:`64-12=52` 位，但这里我们是用来在多级页表上进行遍历，因此
  只取出低 :math:`27` 位。
- ``PageTable::find_pte_create`` 在多级页表找到一个虚拟页号对应的页表项的可变引用方便后续的读写。如果在
  遍历的过程中发现有节点尚未创建则会新建一个节点。

  变量 ``ppn`` 表示当前节点的物理页号，最开始指向多级页表的根节点。随后每次循环通过 ``get_pte_array`` 将
  取出当前节点的页表项数组，并根据当前级页索引找到对应的页表项。如果当前节点是一个叶节点，那么直接返回这个页表项
  的可变引用；否则尝试向下走。走不下去的话就新建一个节点，更新作为下级节点指针的页表项，并将新分配的物理页帧移动到
  向量 ``frames`` 中方便后续的自动回收。注意在更新页表项的时候，不仅要更新物理页号，还要将标志位 V 置 1，
  不然硬件在查多级页表的时候，会认为这个页表项不合法，从而触发 Page Fault 而不能向下走。

于是， ``map/unmap`` 就非常容易实现了：

.. code-block:: rust

    // os/src/mm/page_table.rs

    impl PageTable {
        pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
            let pte = self.find_pte_create(vpn).unwrap();
            assert!(!pte.is_valid(), "vpn {:?} is mapped before mapping", vpn);
            *pte = PageTableEntry::new(ppn, flags | PTEFlags::V);
        }
        pub fn unmap(&mut self, vpn: VirtPageNum) {
            let pte = self.find_pte_create(vpn).unwrap();
            assert!(pte.is_valid(), "vpn {:?} is invalid before unmapping", vpn);
            *pte = PageTableEntry::empty();
        }
    }

只需根据虚拟页号找到页表项，然后修改或者直接清空其内容即可。

.. warning::

    目前的实现方式并不打算对物理页帧耗尽的情形做任何处理而是直接 ``panic`` 退出。因此在前面的代码中能够看到
    很多 ``unwrap`` ，这种使用方式并不为 Rust 所推荐，只是由于简单起见暂且这样做。

为了方便后面的实现，我们还需要 ``PageTable`` 提供一种不经过 MMU 而是手动查页表的方法：

.. code-block:: rust
    :linenos:

    // os/src/mm/page_table.rs

    impl PageTable {
        /// Temporarily used to get arguments from user space.
        pub fn from_token(satp: usize) -> Self {
            Self {
                root_ppn: PhysPageNum::from(satp & ((1usize << 44) - 1)),
                frames: Vec::new(),
            }
        }
        fn find_pte(&self, vpn: VirtPageNum) -> Option<&PageTableEntry> {
            let idxs = vpn.indexes();
            let mut ppn = self.root_ppn;
            let mut result: Option<&PageTableEntry> = None;
            for i in 0..3 {
                let pte = &ppn.get_pte_array()[idxs[i]];
                if i == 2 {
                    result = Some(pte);
                    break;
                }
                if !pte.is_valid() {
                    return None;
                }
                ppn = pte.ppn();
            }
            result
        }
        pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
            self.find_pte(vpn)
                .map(|pte| {pte.clone()})
        }
    }

- 第 5 行的 ``from_token`` 可以临时创建一个专用来手动查页表的 ``PageTable`` ，它仅有一个从传入的 ``satp`` token
  中得到的多级页表根节点的物理页号，它的 ``frames`` 字段为空，也即不实际控制任何资源；
- 第 11 行的 ``find_pte`` 和之前的 ``find_pte_create`` 不同之处在于它不会试图分配物理页帧。一旦在多级页表上遍历
  遇到空指针它就会直接返回 ``None`` 表示无法正确找到传入的虚拟页号对应的页表项；
- 第 28 行的 ``translate`` 调用 ``find_pte`` 来实现，如果能够找到页表项，那么它会将页表项拷贝一份并返回，否则就
  返回一个 ``None`` 。

.. chyyuu 没有提到from_token的作用???