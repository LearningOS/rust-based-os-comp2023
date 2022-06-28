基于地址空间的分时多任务
==============================================================

本节我们介绍如何基于地址空间抽象来实现第三章的分时多任务系统。

建立并开启基于分页模式的虚拟地址空间
--------------------------------------------

当 SBI 实现（本项目中基于 RustSBI）初始化完成后， CPU 将跳转到内核入口点并在 S 特权级上执行，此时还并没有开启分页模式
，内核的每一次访存仍被视为一个物理地址直接访问物理内存。而在开启分页模式之后，内核的代码在访存的时候只能看到内核地址空间，
此时每次访存将被视为一个虚拟地址且需要通过 MMU 基于内核地址空间的多级页表的地址转换。这两种模式之间的过渡在内核初始化期间
完成。

创建内核地址空间
^^^^^^^^^^^^^^^^^^^^^^^^


我们创建内核地址空间的全局实例：

.. code-block:: rust

    // os/src/mm/memory_set.rs

    lazy_static! {
        pub static ref KERNEL_SPACE: Arc<UPSafeCell<MemorySet>> = Arc::new(unsafe {
            UPSafeCell::new(MemorySet::new_kernel()
        )});
    }

从之前对于 ``lazy_static!`` 宏的介绍可知， ``KERNEL_SPACE`` 在运行期间它第一次被用到时才会实际进行初始化，而它所
占据的空间则是编译期被放在全局数据段中。 ``Arc<UPSafeCell<_>>`` 同时带来 ``Arc<T>`` 提供的共享
引用，和 ``UPSafeCell<T>`` 提供的互斥访问。

在 ``rust_main`` 函数中，我们首先调用 ``mm::init`` 进行内存管理子系统的初始化：

.. code-block:: rust

    // os/src/mm/mod.rs

    pub use memory_set::KERNEL_SPACE;

    pub fn init() {
        heap_allocator::init_heap();
        frame_allocator::init_frame_allocator();
        KERNEL_SPACE.exclusive_access().activate();
    }

可以看到，我们最先进行了全局动态内存分配器的初始化，因为接下来马上就要用到 Rust 的堆数据结构。接下来我们初始化物理页帧
管理器（内含堆数据结构 ``Vec<T>`` ）使能可用物理页帧的分配和回收能力。最后我们创建内核地址空间并让 CPU 开启分页模式，
MMU 在地址转换的时候使用内核的多级页表，这一切均在一行之内做到：

- 首先，我们引用 ``KERNEL_SPACE`` ，这是它第一次被使用，就在此时它会被初始化，调用 ``MemorySet::new_kernel``
  创建一个内核地址空间并使用 ``Arc<UPSafeCell<T>>`` 包裹起来；

- 最然后，我们调用 ``MemorySet::activate`` ：

    .. code-block:: rust
        :linenos:

        // os/src/mm/page_table.rs

        pub fn token(&self) -> usize {
            8usize << 60 | self.root_ppn.0
        }

        // os/src/mm/memory_set.rs

        impl MemorySet {
            pub fn activate(&self) {
                let satp = self.page_table.token();
                unsafe {
                    satp::write(satp);
                    core::arch::asm!("sfence.vma");
                }
            }
        }

  ``PageTable::token`` 会按照 :ref:`satp CSR 格式要求 <satp-layout>` 构造一个无符号 64 位无符号整数，使得其
  分页模式为 SV39 ，且将当前多级页表的根节点所在的物理页号填充进去。在 ``activate`` 中，我们将这个值写入当前 CPU 的
  satp CSR ，从这一刻开始 SV39 分页模式就被启用了，而且 MMU 会使用内核地址空间的多级页表进行地址转换。

  我们必须注意切换 satp CSR 是否是一个 *平滑* 的过渡：其含义是指，切换 satp 的指令及其下一条指令这两条相邻的指令的
  虚拟地址是相邻的（由于切换 satp 的指令并不是一条跳转指令， pc 只是简单的自增当前指令的字长），
  而它们所在的物理地址一般情况下也是相邻的，但是它们所经过的地址转换流程却是不同的——切换 satp 导致 MMU 查的多级页表
  是不同的。这就要求前后两个地址空间在切换 satp 的指令 *附近* 的映射满足某种意义上的连续性。

  幸运的是，我们做到了这一点。这条写入 satp 的指令及其下一条指令都在内核内存布局的代码段中，在切换之后是一个恒等映射，
  而在切换之前是视为物理地址直接取指，也可以将其看成一个恒等映射。这完全符合我们的期待：即使切换了地址空间，指令仍应该
  能够被连续的执行。

注意到在 ``activate`` 的最后，我们插入了一条汇编指令 ``sfence.vma`` ，它又起到什么作用呢？

让我们再来回顾一下多级页表：它相比线性表虽然大量节约了内存占用，但是却需要 MMU 进行更多的隐式访存。如果是一个线性表，
MMU 仅需单次访存就能找到页表项并完成地址转换，而多级页表（以 SV39 为例，不考虑大页）最顺利的情况下也需要三次访存。这些
额外的访存和真正访问数据的那些访存在空间上并不相邻，加大了多级缓存的压力，一旦缓存缺失将带来巨大的性能惩罚。如果采用
多级页表实现，这个问题会变得更为严重，使得地址空间抽象的性能开销过大。

.. _term-tlb:

为了解决性能问题，一种常见的做法是在 CPU 中利用部分硬件资源额外加入一个 **快表**
(TLB, Translation Lookaside Buffer) ， 它维护了部分虚拟页号到页表项的键值对。当 MMU 进行地址转换的时候，首先
会到快表中看看是否匹配，如果匹配的话直接取出页表项完成地址转换而无需访存；否则再去查页表并将键值对保存在快表中。一旦
我们修改了 satp 切换了地址空间，快表中的键值对就会失效，因为它还表示着上个地址空间的映射关系。为了 MMU 的地址转换
能够及时与 satp 的修改同步，我们可以选择立即使用 ``sfence.vma`` 指令将快表清空，这样 MMU 就不会看到快表中已经
过期的键值对了。

.. _term-trampoline:

跳板的实现
------------------------------------

上一小节我们看到无论是内核还是应用的地址空间，最高的虚拟页面都是一个跳板。同时应用地址空间的次高虚拟页面还被设置为用来
存放应用的 Trap 上下文。那么跳板究竟起什么作用呢？为何不直接把 Trap 上下文仍放到应用的内核栈中呢？

回忆曾在第二章介绍过的，当一个应用 Trap 到内核的时候，
``sscratch`` 已经指出了该应用内核栈的栈顶，我们用一条指令即可从用户栈切换到内核栈，然后直接将 Trap 上下文压入内核栈
栈顶。当 Trap 处理完毕返回用户态的时候，将 Trap 上下文中的内容恢复到寄存器上，最后将保存着应用用户栈顶的 ``sscratch``
与 sp 进行交换，也就从内核栈切换回了用户栈。在这个过程中， ``sscratch`` 起到了非常关键的作用，它使得我们可以在不破坏
任何通用寄存器的情况下完成用户栈和内核栈顶的 Trap 上下文这两个工作区域之间的切换。

然而，一旦使能了分页机制，一切就并没有这么简单了，我们必须在这个过程中同时完成地址空间的切换。
具体来说，当 ``__alltraps`` 保存 Trap 上下文的时候，我们必须通过修改 satp 从应用地址空间切换到内核地址空间，
因为 trap handler 只有在内核地址空间中才能访问；
同理，在 ``__restore`` 恢复 Trap 上下文的时候，我们也必须从内核地址空间切换回应用地址空间，因为应用的代码和
数据只能在它自己的地址空间中才能访问，内核地址空间是看不到的。
进而，地址空间的切换不能影响指令的连续执行，这就要求应用和内核地址空间在切换地址空间指令附近是平滑的。

.. _term-meltdown:

.. note::

    **内核与应用地址空间的隔离**

    目前我们的设计是有一个唯一的内核地址空间存放内核的代码、数据，同时对于每个应用维护一个它们自己的地址空间，因此在
    Trap 的时候就需要进行地址空间切换，而在任务切换的时候无需进行（因为这个过程全程在内核内完成）。而教程前两版以及
    :math:`\mu` core 中的设计是每个应用都有一个地址空间，可以将其中的逻辑段分为内核和用户两部分，分别映射到内核和
    用户的数据和代码，且分别在 CPU 处于 S/U 特权级时访问。此设计中并不存在一个单独的内核地址空间。

    之前设计方式的优点在于： Trap 的时候无需切换地址空间，而在任务切换的时候才需要切换地址空间。由于后者比前者更容易
    实现，这降低了实现的复杂度。而且在应用高频进行系统调用的时候能够避免地址空间切换的开销，这通常源于快表或 cache
    的失效问题。但是这种设计方式也有缺点：即内核的逻辑段需要在每个应用的地址空间内都映射一次，这会带来一些无法忽略的
    内存占用开销，并显著限制了嵌入式平台的任务并发数。此外，这种做法无法应对处理器的 `熔断
    (Meltdown) 漏洞 <https://cacm.acm.org/magazines/2020/6/245161-meltdown/fulltext>`_ ，
    使得恶意应用能够以某种方式看到它本来无权访问的地址空间中内核部分的数据。将内核与地址空间隔离便是修复此漏洞的一种方法。

    经过权衡，在本教程中我们参考 MIT 的教学 OS `xv6 <https://github.com/mit-pdos/xv6-riscv>`_ ，
    采用内核和应用地址空间隔离的设计。

我们为何将应用的 Trap 上下文放到应用地址空间的次高页面而不是内核地址空间中的内核栈中呢？原因在于，假如我们将其放在内核栈
中，在保存 Trap 上下文之前我们必须先切换到内核地址空间，这就需要我们将内核地址空间的 token 写入 satp 寄存器，之后我们
还需要有一个通用寄存器保存内核栈栈顶的位置，这样才能以它为基址保存 Trap 上下文。在保存 Trap 上下文之前我们必须完成这
两项工作。然而，我们无法在不破坏任何一个通用寄存器的情况下做到这一点。因为事实上我们需要用到内核的两条信息：内核地址空间
的 token 还有应用内核栈顶的位置，硬件却只提供一个 ``sscratch`` 可以用来进行周转。所以，我们不得不将 Trap 上下文保存在
应用地址空间的一个虚拟页面中以避免切换到内核地址空间才能保存。

为了方便实现，我们在 Trap 上下文中包含更多内容（和我们关于上下文的定义有些不同，它们在初始化之后便只会被读取而不会被写入
，并不是每次都需要保存/恢复）：

.. code-block:: rust
    :linenos:
    :emphasize-lines: 8,9,10

    // os/src/trap/context.rs

    #[repr(C)]
    pub struct TrapContext {
        pub x: [usize; 32],
        pub sstatus: Sstatus,
        pub sepc: usize,
        pub kernel_satp: usize,
        pub kernel_sp: usize,
        pub trap_handler: usize,
    }

在多出的三个字段中：

- ``kernel_satp`` 表示内核地址空间的 token ；
- ``kernel_sp`` 表示当前应用在内核地址空间中的内核栈栈顶的虚拟地址；
- ``trap_handler`` 表示内核中 trap handler 入口点的虚拟地址。

它们在应用初始化的时候由内核写入应用地址空间中的 TrapContext 的相应位置，此后就不再被修改。

让我们来看一下现在的 ``__alltraps`` 和 ``__restore`` 各是如何在保存和恢复 Trap 上下文的同时也切换地址空间的：

.. code-block:: riscv
    :linenos:

    # os/src/trap/trap.S

        .section .text.trampoline
        .globl __alltraps
        .globl __restore
        .align 2
    __alltraps:
        csrrw sp, sscratch, sp
        # now sp->*TrapContext in user space, sscratch->user stack
        # save other general purpose registers
        sd x1, 1*8(sp)
        # skip sp(x2), we will save it later
        sd x3, 3*8(sp)
        # skip tp(x4), application does not use it
        # save x5~x31
        .set n, 5
        .rept 27
            SAVE_GP %n
            .set n, n+1
        .endr
        # we can use t0/t1/t2 freely, because they have been saved in TrapContext
        csrr t0, sstatus
        csrr t1, sepc
        sd t0, 32*8(sp)
        sd t1, 33*8(sp)
        # read user stack from sscratch and save it in TrapContext
        csrr t2, sscratch
        sd t2, 2*8(sp)
        # load kernel_satp into t0
        ld t0, 34*8(sp)
        # load trap_handler into t1
        ld t1, 36*8(sp)
        # move to kernel_sp
        ld sp, 35*8(sp)
        # switch to kernel space
        csrw satp, t0
        sfence.vma
        # jump to trap_handler
        jr t1

    __restore:
        # a0: *TrapContext in user space(Constant); a1: user space token
        # switch to user space
        csrw satp, a1
        sfence.vma
        csrw sscratch, a0
        mv sp, a0
        # now sp points to TrapContext in user space, start restoring based on it
        # restore sstatus/sepc
        ld t0, 32*8(sp)
        ld t1, 33*8(sp)
        csrw sstatus, t0
        csrw sepc, t1
        # restore general purpose registers except x0/sp/tp
        ld x1, 1*8(sp)
        ld x3, 3*8(sp)
        .set n, 5
        .rept 27
            LOAD_GP %n
            .set n, n+1
        .endr
        # back to user stack
        ld sp, 2*8(sp)
        sret

- 当应用 Trap 进入内核的时候，硬件会设置一些 CSR 并在 S 特权级下跳转到 ``__alltraps`` 保存 Trap 上下文。此时
  sp 寄存器仍指向用户栈，但 ``sscratch`` 则被设置为指向应用地址空间中存放 Trap 上下文的位置，实际在次高页面。
  随后，就像之前一样，我们 ``csrrw`` 交换 sp 和 ``sscratch`` ，并基于指向 Trap 上下文位置的 sp 开始保存通用
  寄存器和一些 CSR ，这个过程在第 28 行结束。到这里，我们就全程在应用地址空间中完成了保存 Trap 上下文的工作。

- 接下来该考虑切换到内核地址空间并跳转到 trap handler 了。第 30 行我们将内核地址空间的 token 载入到 t0 寄存器中，
  第 32 行我们将 trap handler 入口点的虚拟地址载入到 t1 寄存器中，第 34 行我们直接将 sp 修改为应用内核栈顶的地址。
  这三条信息均是内核在初始化该应用的时候就已经设置好的。第 36~37 行我们将 satp 修改为内核地址空间的 token 并使用
  ``sfence.vma`` 刷新快表，这就切换到了内核地址空间。最后在第 39 行我们通过 ``jr`` 指令跳转到 t1 寄存器所保存的
  trap handler 入口点的地址。注意这里我们不能像之前的章节那样直接 ``call trap_handler`` ，原因稍后解释。
- 当内核将 Trap 处理完毕准备返回用户态的时候会 *调用* ``__restore`` ，它有两个参数：第一个是 Trap 上下文在应用
  地址空间中的位置，这个对于所有的应用来说都是相同的，由调用规范在 a0 寄存器中传递；第二个则是即将回到的应用的地址空间
  的 token ，在 a1 寄存器中传递。由于 Trap 上下文是保存在应用地址空间中的，第 44~45 行我们先切换回应用地址空间。第
  46 行我们将传入的 Trap 上下文位置保存在 ``sscratch`` 寄存器中，这样 ``__alltraps`` 中才能基于它将 Trap 上下文
  保存到正确的位置。第 47 行我们将 sp 修改为 Trap 上下文的位置，后面基于它恢复各通用寄存器和 CSR。最后在第 64 行，
  我们通过 ``sret`` 指令返回用户态。

接下来还需要考虑切换地址空间前后指令能否仍能连续执行。可以看到我们将 ``trap.S`` 中的整段汇编代码放置在
``.text.trampoline`` 段，并在调整内存布局的时候将它对齐到代码段的一个页面中：

.. code-block:: diff
    :linenos:

    # os/src/linker.ld

        stext = .;
        .text : {
            *(.text.entry)
    +        . = ALIGN(4K);
    +        strampoline = .;
    +        *(.text.trampoline);
    +        . = ALIGN(4K);
            *(.text .text.*)
        }

这样，这段汇编代码放在一个物理页帧中，且 ``__alltraps`` 恰好位于这个物理页帧的开头，其物理地址被外部符号
``strampoline`` 标记。在开启分页模式之后，内核和应用代码都只能看到各自的虚拟地址空间，而在它们的视角中，这段汇编代码
被放在它们地址空间的最高虚拟页面上，由于这段汇编代码在执行的时候涉及到地址空间切换，故而被称为跳板页面。

那么在产生trap前后的一小段时间内会有一个比较 **极端** 的情况，即刚产生trap时，CPU已经进入了内核态（即Supervisor Mode），
但此时执行代码和访问数据还是在应用程序所处的用户态虚拟地址空间中，而不是我们通常理解的内核虚拟地址空间。在这段特殊的时间内，CPU指令
为什么能够被连续执行呢？这里需要注意：无论是内核还是应用的地址空间，跳板的虚拟页均位于同样位置，且它们也将会映射到同一个实际存放这段
汇编代码的物理页帧。也就是说，在执行 ``__alltraps`` 或 ``__restore`` 函数进行地址空间切换的时候，
应用的用户态虚拟地址空间和操作系统内核的内核态虚拟地址空间对切换地址空间的指令所在页的映射方式均是相同的，
这就说明了这段切换地址空间的指令控制流仍是可以连续执行的。

现在可以说明我们在创建用户/内核地址空间中用到的 ``map_trampoline`` 是如何实现的了：

.. code-block:: rust
    :linenos:

    // os/src/config.rs

    pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;

    // os/src/mm/memory_set.rs

    impl MemorySet {
        /// Mention that trampoline is not collected by areas.
        fn map_trampoline(&mut self) {
            self.page_table.map(
                VirtAddr::from(TRAMPOLINE).into(),
                PhysAddr::from(strampoline as usize).into(),
                PTEFlags::R | PTEFlags::X,
            );
        }
    }

这里我们为了实现方便并没有新增逻辑段 ``MemoryArea`` 而是直接在多级页表中插入一个从地址空间的最高虚拟页面映射到
跳板汇编代码所在的物理页帧的键值对，访问方式限制与代码段相同，即 RX 。

最后可以解释为何我们在 ``__alltraps`` 中需要借助寄存器 ``jr`` 而不能直接 ``call trap_handler`` 了。因为在
内存布局中，这条 ``.text.trampoline`` 段中的跳转指令和 ``trap_handler`` 都在代码段之内，汇编器（Assembler）
和链接器（Linker）会根据 ``linker.ld`` 的地址布局描述，设定电子指令的地址，并计算二者地址偏移量
并让跳转指令的实际效果为当前 pc 自增这个偏移量。但实际上我们知道由于我们设计的缘故，这条跳转指令在被执行的时候，
它的虚拟地址被操作系统内核设置在地址空间中的最高页面之内，加上这个偏移量并不能正确的得到 ``trap_handler`` 的入口地址。

**问题的本质可以概括为：跳转指令实际被执行时的虚拟地址和在编译器/汇编器/链接器进行后端代码生成和链接形成最终机器码时设置此指令的地址是不同的。**

加载和执行应用程序
------------------------------------

扩展任务控制块
^^^^^^^^^^^^^^^^^^^^^^^^^^^

为了让应用在运行时有一个安全隔离且符合编译器给应用设定的地址空间布局的虚拟地址空间，操作系统需要对任务进行更多的管理，所以任务控制块相比第三章也包含了更多内容：

.. code-block:: rust
    :linenos:
    :emphasize-lines: 6,7,8

    // os/src/task/task.rs

    pub struct TaskControlBlock {
        pub task_status: TaskStatus,
        pub task_cx: TaskContext,
        pub memory_set: MemorySet,
        pub trap_cx_ppn: PhysPageNum,
        pub base_size: usize,
    }

除了应用的地址空间 ``memory_set`` 之外，还有位于应用地址空间次高页的 Trap 上下文被实际存放在物理页帧的物理页号
``trap_cx_ppn`` ，它能够方便我们对于 Trap 上下文进行访问。此外， ``base_size`` 统计了应用数据的大小，也就是
在应用地址空间中从 :math:`\text{0x0}` 开始到用户栈结束一共包含多少字节。它后续还应该包含用于应用动态内存分配的
堆空间的大小，但我们暂不支持。



更新对任务控制块的管理
^^^^^^^^^^^^^^^^^^^^^^^^^^^

下面是任务控制块的创建：

.. code-block:: rust
    :linenos:

    // os/src/config.rs

    /// Return (bottom, top) of a kernel stack in kernel space.
    pub fn kernel_stack_position(app_id: usize) -> (usize, usize) {
        let top = TRAMPOLINE - app_id * (KERNEL_STACK_SIZE + PAGE_SIZE);
        let bottom = top - KERNEL_STACK_SIZE;
        (bottom, top)
    }

    // os/src/task/task.rs

    impl TaskControlBlock {
        pub fn new(elf_data: &[u8], app_id: usize) -> Self {
            // memory_set with elf program headers/trampoline/trap context/user stack
            let (memory_set, user_sp, entry_point) = MemorySet::from_elf(elf_data);
            let trap_cx_ppn = memory_set
                .translate(VirtAddr::from(TRAP_CONTEXT).into())
                .unwrap()
                .ppn();
            let task_status = TaskStatus::Ready;
            // map a kernel-stack in kernel space
            let (kernel_stack_bottom, kernel_stack_top) = kernel_stack_position(app_id);
            KERNEL_SPACE
                .exclusive_access()
                .insert_framed_area(
                    kernel_stack_bottom.into(),
                    kernel_stack_top.into(),
                    MapPermission::R | MapPermission::W,
            );
            let task_control_block = Self {
                task_status,
                task_cx: TaskContext::goto_trap_return(kernel_stack_top),
                memory_set,
                trap_cx_ppn,
                base_size: user_sp,
            };
            // prepare TrapContext in user space
            let trap_cx = task_control_block.get_trap_cx();
            *trap_cx = TrapContext::app_init_context(
                entry_point,
                user_sp,
                KERNEL_SPACE.exclusive_access().token(),
                kernel_stack_top,
                trap_handler as usize,
            );
            task_control_block
        }
    }

- 第 15 行，我们解析传入的 ELF 格式数据构造应用的地址空间 ``memory_set`` 并获得其他信息；
- 第 16 行，我们从地址空间 ``memory_set`` 中查多级页表找到应用地址空间中的 Trap 上下文实际被放在哪个物理页帧；
- 第 22 行，我们根据传入的应用 ID ``app_id`` 调用在 ``config`` 子模块中定义的 ``kernel_stack_position`` 找到
  应用的内核栈预计放在内核地址空间 ``KERNEL_SPACE`` 中的哪个位置，并通过 ``insert_framed_area`` 实际将这个逻辑段
  加入到内核地址空间中；

.. _trap-return-intro:

- 我们在应用的内核栈顶压入一个跳转到 ``trap_return`` 而不是 ``__restore`` 的任务上下文，
  这主要是为了能够支持对该应用的启动并顺利切换到用户地址空间执行。在构造方式上，只是将 ra 寄存器的值设置为
  ``trap_return`` 的地址。 ``trap_return`` 是我们后面要介绍的新版的 Trap 处理的一部分。
- 初始化该应用的 Trap 上下文，由于它是在应用地址空间而不是在内核地址空间中，我们只能手动查页表找到
  Trap 上下文实际被放在的物理页帧，再获得在用户空间的 Trap 上下文的可变引用用于初始化：

  .. code-block:: rust

    // os/src/task/task.rs

    impl TaskControlBlock {
        pub fn get_trap_cx(&self) -> &'static mut TrapContext {
            self.trap_cx_ppn.get_mut()
        }
    }

  此处需要说明的是，返回 ``'static`` 的可变引用和之前一样可以看成一个绕过 unsafe 的裸指针；而 ``PhysPageNum::get_mut``
  是一个泛型函数，由于我们已经声明了总体返回 ``TrapContext`` 的可变引用，则Rust编译器会给 ``get_mut`` 泛型函数针对具体类型 ``TrapContext``
  的情况生成一个特定版本的 ``get_mut`` 函数实现。在 ``get_trap_cx`` 函数中则会静态调用``get_mut`` 泛型函数的特定版本实现。

  .. code-block:: rust
      :linenos:
      :emphasize-lines: 8,9,10,18,19,20

      // os/src/trap/context.rs

      impl TrapContext {
          pub fn set_sp(&mut self, sp: usize) { self.x[2] = sp; }
          pub fn app_init_context(
              entry: usize,
              sp: usize,
              kernel_satp: usize,
              kernel_sp: usize,
              trap_handler: usize,
          ) -> Self {
              let mut sstatus = sstatus::read();
              sstatus.set_spp(SPP::User);
              let mut cx = Self {
                  x: [0; 32],
                  sstatus,
                  sepc: entry,
                  kernel_satp,
                  kernel_sp,
                  trap_handler,
              };
              cx.set_sp(sp);
              cx
          }
      }

  和之前相比 ``TrapContext::app_init_context`` 需要补充上让应用在 ``__alltraps`` 能够顺利进入到内核地址空间
  并跳转到 trap handler 入口点的相关信息。

在内核初始化的时候，需要将所有的应用加载到全局应用管理器中：

.. code-block:: rust
    :linenos:

    // os/src/task/mod.rs

    struct TaskManagerInner {
        tasks: Vec<TaskControlBlock>,
        current_task: usize,
    }

    lazy_static! {
        pub static ref TASK_MANAGER: TaskManager = {
            info!("init TASK_MANAGER");
            let num_app = get_num_app();
            info!("num_app = {}", num_app);
            let mut tasks: Vec<TaskControlBlock> = Vec::new();
            for i in 0..num_app {
                tasks.push(TaskControlBlock::new(get_app_data(i), i));
            }
            TaskManager {
                num_app,
                inner: unsafe {
                    UPSafeCell::new(TaskManagerInner {
                        tasks,
                        current_task: 0,
                    })
                },
            }
        };
    }

可以看到，在 ``TaskManagerInner`` 中我们使用向量 ``Vec`` 来保存任务控制块。在全局任务管理器 ``TASK_MANAGER``
初始化的时候，只需使用 ``loader`` 子模块提供的 ``get_num_app`` 和 ``get_app_data`` 分别获取链接到内核的应用
数量和每个应用的 ELF 文件格式的数据，然后依次给每个应用创建任务控制块并加入到向量中即可。我们还将 ``current_task`` 设置
为 0 ，于是将从第 0 个应用开始执行。

回过头来介绍一下应用构建器 ``os/build.rs`` 的改动：

- 首先，我们在 ``.incbin`` 中不再插入清除全部符号的应用二进制镜像 ``*.bin`` ，而是将构建得到的 ELF 格式文件直接链接进来；
- 其次，在链接每个 ELF 格式文件之前我们都加入一行 ``.align 3`` 来确保它们对齐到 8 字节，这是由于如果不这样做，
  ``xmas-elf`` crate 可能会在解析 ELF 的时候进行不对齐的内存读写，例如使用 ``ld`` 指令从内存的一个没有对齐到 8 字节的地址加载一个 64 位的值到一个通用寄存器。

为了方便后续的实现，全局任务管理器还需要提供关于当前应用与地址空间有关的一些信息。通过 ``current_user_token`` 和
``current_trap_cx`` 分别可以获得当前正在执行的应用的地址空间的 token 和可以在
内核地址空间中修改位于该应用地址空间中的 Trap 上下文的可变引用。

改进 Trap 处理的实现
------------------------------------

为了能够支持地址空间，让我们来看现在 ``trap_handler`` 的改进实现：

.. code-block:: rust
    :linenos:

    // os/src/trap/mod.rs

    fn set_kernel_trap_entry() {
        unsafe {
            stvec::write(trap_from_kernel as usize, TrapMode::Direct);
        }
    }

    #[no_mangle]
    pub fn trap_from_kernel() -> ! {
        panic!("a trap from kernel!");
    }

    #[no_mangle]
    pub fn trap_handler() -> ! {
        set_kernel_trap_entry();
        let cx = current_trap_cx();
        let scause = scause::read();
        let stval = stval::read();
        match scause.cause() {
            ...
        }
        trap_return();
    }

由于应用的 Trap 上下文不在内核地址空间，因此我们调用 ``current_trap_cx`` 来获取当前应用的 Trap 上下文的可变引用
而不是像之前那样作为参数传入 ``trap_handler`` 。至于 Trap 处理的过程则没有发生什么变化。

注意到，在 ``trap_handler`` 的开头还调用 ``set_kernel_trap_entry`` 将 ``stvec`` 修改为同模块下另一个函数
``trap_from_kernel`` 的地址。这就是说，一旦进入内核后再次触发到 S 的 Trap，则会在硬件设置一些 CSR 之后跳过寄存器
的保存过程直接跳转到 ``trap_from_kernel`` 函数，在这里我们直接 ``panic`` 退出。这是因为内核和应用的地址空间分离
之后，从 U 还是从 S Trap 到 S 的 Trap 上下文保存与恢复实现方式和 Trap 处理逻辑有很大差别，我们不得不实现两遍而
不太可能将二者整合起来。这里简单起见我们弱化了从 S 到 S 的 Trap ，省略了 Trap 上下文保存过程而直接 ``panic`` 。

在 ``trap_handler`` 完成 Trap 处理之后，我们需要调用 ``trap_return`` 返回用户态：

.. code-block:: rust
    :linenos:

    // os/src/trap/mod.rs

    fn set_user_trap_entry() {
        unsafe {
            stvec::write(TRAMPOLINE as usize, TrapMode::Direct);
        }
    }

    #[no_mangle]
    pub fn trap_return() -> ! {
        set_user_trap_entry();
        let trap_cx_ptr = TRAP_CONTEXT;
        let user_satp = current_user_token();
        extern "C" {
            fn __alltraps();
            fn __restore();
        }
        let restore_va = __restore as usize - __alltraps as usize + TRAMPOLINE;
        unsafe {
                core::arch::asm!(
                "fence.i",
                "jr {restore_va}",
                restore_va = in(reg) restore_va,
                in("a0") trap_cx_ptr,
                in("a1") user_satp,
                options(noreturn)
            );
        }
        panic!("Unreachable in back_to_user!");
    }

- 第 11 行，在 ``trap_return`` 的开头我们调用 ``set_user_trap_entry`` 来让应用 Trap 到 S 的时候可以跳转到
  ``__alltraps`` 。注意我们把 ``stvec`` 设置为内核和应用地址空间共享的跳板页面的起始地址 ``TRAMPOLINE`` 而不是
  编译器在链接时看到的 ``__alltraps`` 的地址，因为启用分页模式之后我们只能通过跳板页面上的虚拟地址来实际取得
  ``__alltraps`` 和 ``__restore`` 的汇编代码。
- 之前介绍的时候提到过 ``__restore`` 需要两个参数：分别是 Trap 上下文在应用地址空间中的虚拟地址和要继续执行的应用
  地址空间的 token 。第 12 和第 13 行则分别准备好这两个参数。
- 最后我们需要跳转到 ``__restore`` 切换到应用地址空间从 Trap 上下文中恢复通用寄存器并 ``sret`` 继续执行应用。它的
  关键在于如何找到 ``__restore`` 在内核/应用地址空间中共同的虚拟地址。第 18 行我们展示了计算它的过程：由于
  ``__alltraps`` 是对齐到地址空间跳板页面的起始地址 ``TRAMPOLINE`` 上的， 则 ``__restore`` 的虚拟地址只需在
  ``TRAMPOLINE`` 基础上加上 ``__restore`` 相对于 ``__alltraps`` 的偏移量即可。这里 ``__alltraps`` 和
  ``__restore`` 都是指编译器在链接时看到的内核内存布局中的地址。我们使用 ``jr`` 指令完成了跳转的任务。
- 在开始执行应用之前，我们需要使用 ``fence.i`` 指令清空指令缓存 i-cache 。这是因为，在内核中进行的一些操作
  可能导致一些原先存放某个应用代码的物理页帧如今用来存放数据或者是其他应用的代码，i-cache 中可能还保存着该物理页帧的
  错误快照。因此我们直接将整个 i-cache 清空避免错误。

改进 sys_write 的实现
------------------------------------

同样由于内核和应用地址空间的隔离， ``sys_write`` 不再能够直接访问位于应用空间中的数据，而需要手动查页表才能知道那些
数据被放置在哪些物理页帧上并进行访问。

为此，页表模块 ``page_table`` 提供了将应用地址空间中一个缓冲区转化为在内核空间中能够直接访问的形式的辅助函数：

.. code-block:: rust
    :linenos:

    // os/src/mm/page_table.rs

    pub fn translated_byte_buffer(
        token: usize,
        ptr: *const u8,
        len: usize
    ) -> Vec<&'static [u8]> {
        let page_table = PageTable::from_token(token);
        let mut start = ptr as usize;
        let end = start + len;
        let mut v = Vec::new();
        while start < end {
            let start_va = VirtAddr::from(start);
            let mut vpn = start_va.floor();
            let ppn = page_table
                .translate(vpn)
                .unwrap()
                .ppn();
            vpn.step();
            let mut end_va: VirtAddr = vpn.into();
            end_va = end_va.min(VirtAddr::from(end));
            v.push(&ppn.get_bytes_array()[start_va.page_offset()..end_va.page_offset()]);
            start = end_va.into();
        }
        v
    }

参数中的 ``token`` 是某个应用地址空间的 token ， ``ptr`` 和 ``len`` 则分别表示该地址空间中的一段缓冲区的起始地址
和长度。 ``translated_byte_buffer`` 会以向量的形式返回一组可以在内核空间中直接访问的字节数组切片，具体实现在这里
不再赘述。

进而，我们完成对 ``sys_write`` 系统调用的改造：

.. code-block:: rust

    // os/src/syscall/fs.rs

    pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
        match fd {
            FD_STDOUT => {
                let buffers = translated_byte_buffer(current_user_token(), buf, len);
                for buffer in buffers {
                    print!("{}", core::str::from_utf8(buffer).unwrap());
                }
                len as isize
            },
            _ => {
                panic!("Unsupported fd in sys_write!");
            }
        }
    }

我们尝试将每个字节数组切片转化为字符串 ``&str`` 然后输出即可。

