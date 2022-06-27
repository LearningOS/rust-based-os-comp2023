.. _term-trap-handle:

实现特权级的切换
===========================

.. toctree::
   :hidden:
   :maxdepth: 5

RISC-V特权级切换
---------------------------------------

特权级切换的起因
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
批处理操作系统为了建立好应用程序的执行环境，需要在执行应用程序前进行一些初始化工作，
并监控应用程序的执行，具体体现在：

- 启动应用程序时，需要初始化应用程序的用户态上下文，并能切换到用户态执行应用程序；
- 应用程序发起系统调用后，需要切换到批处理操作系统中进行处理；
- 应用程序执行出错时，批处理操作系统要杀死该应用并加载运行下一个应用；
- 应用程序执行结束时，批处理操作系统要加载运行下一个应用。

这些处理都涉及到特权级切换，因此都需要硬件和操作系统协同提供的特权级切换机制。


特权级切换相关的控制状态寄存器
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

本章中我们仅考虑当 CPU 在 U 特权级运行用户程序的时候触发 Trap，
并切换到 S 特权级的批处理操作系统进行处理。

.. list-table:: 进入 S 特权级 Trap 的相关 CSR
   :header-rows: 1
   :align: center
   :widths: 30 100

   * - CSR 名
     - 该 CSR 与 Trap 相关的功能
   * - sstatus
     - ``SPP`` 等字段给出 Trap 发生之前 CPU 处在哪个特权级（S/U）等信息
   * - sepc
     - 当 Trap 是一个异常的时候，记录 Trap 发生之前执行的最后一条指令的地址
   * - scause
     - 描述 Trap 的原因
   * - stval
     - 给出 Trap 附加信息
   * - stvec
     - 控制 Trap 处理代码的入口地址

特权级切换的具体过程一部分由硬件直接完成，另一部分则需要由操作系统来实现。

.. _trap-hw-mechanism:

特权级切换的硬件控制机制
-------------------------------------

当 CPU 执行完一条指令并准备从用户特权级 陷入（ ``Trap`` ）到 S 特权级的时候，硬件会自动完成如下这些事情：

- ``sstatus`` 的 ``SPP`` 字段会被修改为 CPU 当前的特权级（U/S）。
- ``sepc`` 会被修改为 Trap 处理完成后默认会执行的下一条指令的地址。
- ``scause/stval`` 分别会被修改成这次 Trap 的原因以及相关的附加信息。
- CPU 会跳转到 ``stvec`` 所设置的 Trap 处理入口地址，并将当前特权级设置为 S ，然后从Trap 处理入口地址处开始执行。

.. note::

   **stvec 相关细节**

   在 RV64 中， ``stvec`` 是一个 64 位的 CSR，在中断使能的情况下，保存了中断处理的入口地址。它有两个字段：

   - MODE 位于 [1:0]，长度为 2 bits；
   - BASE 位于 [63:2]，长度为 62 bits。

   当 MODE 字段为 0 的时候， ``stvec`` 被设置为 Direct 模式，此时进入 S 模式的 Trap 无论原因如何，处理 Trap 的入口地址都是 ``BASE<<2``
   ， CPU 会跳转到这个地方进行异常处理。本书中我们只会将 ``stvec`` 设置为 Direct 模式。而 ``stvec`` 还可以被设置为 Vectored 模式，

而当 CPU 完成 Trap 处理准备返回的时候，需要通过一条 S 特权级的特权指令 ``sret`` 来完成，这一条指令具体完成以下功能：

- CPU 会将当前的特权级按照 ``sstatus`` 的 ``SPP`` 字段设置为 U 或者 S ；
- CPU 会跳转到 ``sepc`` 寄存器指向的那条指令，然后继续执行。

用户栈与内核栈
--------------------------------

在 Trap 触发的一瞬间， CPU 会切换到 S 特权级并跳转到 ``stvec`` 所指示的位置。
但是在正式进入 S 特权级的 Trap 处理之前，我们必须保存原控制流的寄存器状态，这一般通过栈来完成。
但我们需要用专门为操作系统准备的内核栈，而不是应用程序运行时用到的用户栈。

我们声明两个类型 ``KernelStack`` 和 ``UserStack`` 分别表示用户栈和内核栈，它们都只是字节数组的简单包装：

.. code-block:: rust
    :linenos:

    // os/src/batch.rs

  #[repr(align(4096))]
  struct KernelStack {
      data: [u8; KERNEL_STACK_SIZE],
  }

  #[repr(align(4096))]
  struct UserStack {
      data: [u8; USER_STACK_SIZE],
  }

  static KERNEL_STACK: KernelStack = KernelStack {
      data: [0; KERNEL_STACK_SIZE],
  };
  static USER_STACK: UserStack = UserStack {
      data: [0; USER_STACK_SIZE],
  };


两个栈以全局变量的形式实例化在批处理操作系统的 ``.bss`` 段中。

我们为两个类型实现了 ``get_sp`` 方法来获取栈顶地址。由于在 RISC-V 中栈是向下增长的，
我们只需返回包裹的数组的结尾地址，以用户栈类型 ``UserStack`` 为例：

.. code-block:: rust
    :linenos:

    impl UserStack {
        fn get_sp(&self) -> usize {
            self.data.as_ptr() as usize + USER_STACK_SIZE
        }
    }

换栈是非常简单的，只需将 ``sp`` 寄存器的值修改为 ``get_sp`` 的返回值即可。

.. _term-trap-context:

接下来是 Trap 上下文，即在 Trap 发生时需要保存的物理资源内容，定义如下：

.. code-block:: rust
    :linenos:

    // os/src/trap/context.rs

    #[repr(C)]
    pub struct TrapContext {
        pub x: [usize; 32],
        pub sstatus: Sstatus,
        pub sepc: usize,
    }

可以看到里面包含所有的通用寄存器 ``x0~x31`` ，还有 ``sstatus`` 和 ``sepc`` 。

- 对于通用寄存器而言，两条控制流（应用程序控制流和内核控制流）运行在不同的特权级，所属的软件也可能由不同的编程语言编写，虽然在 Trap 控制流中只是会执行 Trap 处理
  相关的代码，但依然可能直接或间接调用很多模块，因此很难甚至不可能找出哪些寄存器无需保存。既然如此我们就只能全部保存了。但这里也有一些例外，
  如 ``x0`` 被硬编码为 0 ，它自然不会有变化；还有 ``tp(x4)`` 寄存器，除非我们手动出于一些特殊用途使用它，否则一般也不会被用到。虽然它们无需保存，
  但我们仍然在 ``TrapContext`` 中为它们预留空间，主要是为了后续的实现方便。
- 对于 CSR 而言，我们知道进入 Trap 的时候，硬件会立即覆盖掉 ``scause/stval/sstatus/sepc`` 的全部或是其中一部分。``scause/stval``
  的情况是：它总是在 Trap 处理的第一时间就被使用或者是在其他地方保存下来了，因此它没有被修改并造成不良影响的风险。
  而对于 ``sstatus/sepc`` 而言，它们会在 Trap 处理的全程有意义（在 Trap 控制流最后 ``sret`` 的时候还用到了它们），而且确实会出现
  Trap 嵌套的情况使得它们的值被覆盖掉。所以我们需要将它们也一起保存下来，并在 ``sret`` 之前恢复原样。


Trap 管理
-------------------------------

Trap 上下文的保存与恢复
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

首先是具体实现 Trap 上下文保存和恢复的汇编代码。

.. _trap-context-save-restore:


在批处理操作系统初始化时，我们需要修改 ``stvec`` 寄存器来指向正确的 Trap 处理入口点。

.. code-block:: rust
    :linenos:

    // os/src/trap/mod.rs

    core::arch::global_asm!(include_str!("trap.S"));

    pub fn init() {
        extern "C" { fn __alltraps(); }
        unsafe {
            stvec::write(__alltraps as usize, TrapMode::Direct);
        }
    }

这里我们引入了一个外部符号 ``__alltraps`` ，并将 ``stvec`` 设置为 Direct 模式指向它的地址。我们在 ``os/src/trap/trap.S``
中实现 Trap 上下文保存/恢复的汇编代码，分别用外部符号 ``__alltraps`` 和 ``__restore`` 标记为函数，并通过 ``global_asm!`` 宏将 ``trap.S`` 这段汇编代码插入进来。

Trap 处理的总体流程如下：首先通过 ``__alltraps`` 将 Trap 上下文保存在内核栈上，然后跳转到使用 Rust 编写的 ``trap_handler`` 函数
完成 Trap 分发及处理。当 ``trap_handler`` 返回之后，使用 ``__restore`` 从保存在内核栈上的 Trap 上下文恢复寄存器。最后通过一条
``sret`` 指令回到应用程序执行。

首先是保存 Trap 上下文的 ``__alltraps`` 的实现：

.. code-block:: riscv
    :linenos:

    # os/src/trap/trap.S

    .macro SAVE_GP n
        sd x\n, \n*8(sp)
    .endm

    .align 2
    __alltraps:
        csrrw sp, sscratch, sp
        # now sp->kernel stack, sscratch->user stack
        # allocate a TrapContext on kernel stack
        addi sp, sp, -34*8
        # save general-purpose registers
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
        # we can use t0/t1/t2 freely, because they were saved on kernel stack
        csrr t0, sstatus
        csrr t1, sepc
        sd t0, 32*8(sp)
        sd t1, 33*8(sp)
        # read user stack from sscratch and save it on the kernel stack
        csrr t2, sscratch
        sd t2, 2*8(sp)
        # set input argument of trap_handler(cx: &mut TrapContext)
        mv a0, sp
        call trap_handler

- 第 7 行我们使用 ``.align`` 将 ``__alltraps`` 的地址 4 字节对齐，这是 RISC-V 特权级规范的要求；
- 第 9 行的 ``csrrw`` 原型是 :math:`\text{csrrw rd, csr, rs}` 可以将 CSR 当前的值读到通用寄存器 :math:`\text{rd}` 中，然后将
  通用寄存器 :math:`\text{rs}` 的值写入该 CSR 。因此这里起到的是交换 sscratch 和 sp 的效果。在这一行之前 sp 指向用户栈， sscratch
  指向内核栈（原因稍后说明），现在 sp 指向内核栈， sscratch 指向用户栈。
- 第 12 行，我们准备在内核栈上保存 Trap 上下文，于是预先分配 :math:`34\times 8` 字节的栈帧，这里改动的是 sp ，说明确实是在内核栈上。
- 第 13~24 行，保存 Trap 上下文的通用寄存器 x0~x31，跳过 x0 和 tp(x4)，原因之前已经说明。我们在这里也不保存 sp(x2)，因为它在第 9 行
  后指向的是内核栈。用户栈的栈指针保存在 sscratch 中，必须通过 ``csrr`` 指令读到通用寄存器中后才能使用，因此我们先考虑保存其它通用寄存器，腾出空间。

  我们要基于 sp 来找到每个寄存器应该被保存到的正确的位置。实际上，在栈帧分配之后，我们可用于保存 Trap 上下文的地址区间为 :math:`[\text{sp},\text{sp}+8\times34)` ，
  按照  ``TrapContext`` 结构体的内存布局，基于内核栈的位置（sp所指地址）来从低地址到高地址分别按顺序放置 x0~x31这些通用寄存器，最后是 sstatus 和 sepc 。因此通用寄存器 xn
  应该被保存在地址区间 :math:`[\text{sp}+8n,\text{sp}+8(n+1))` 。

  为了简化代码，x5~x31 这 27 个通用寄存器我们通过类似循环的 ``.rept`` 每次使用 ``SAVE_GP`` 宏来保存，其实质是相同的。注意我们需要在
  ``trap.S`` 开头加上 ``.altmacro`` 才能正常使用 ``.rept`` 命令。
- 第 25~28 行，我们将 CSR sstatus 和 sepc 的值分别读到寄存器 t0 和 t1 中然后保存到内核栈对应的位置上。指令
  :math:`\text{csrr rd, csr}`  的功能就是将 CSR 的值读到寄存器 :math:`\text{rd}` 中。这里我们不用担心 t0 和 t1 被覆盖，
  因为它们刚刚已经被保存了。
- 第 30~31 行专门处理 sp 的问题。首先将 sscratch 的值读到寄存器 t2 并保存到内核栈上，注意： sscratch 的值是进入 Trap 之前的 sp 的值，指向
  用户栈。而现在的 sp 则指向内核栈。
- 第 33 行令 :math:`\text{a}_0\leftarrow\text{sp}`，让寄存器 a0 指向内核栈的栈指针也就是我们刚刚保存的 Trap 上下文的地址，
  这是由于我们接下来要调用 ``trap_handler`` 进行 Trap 处理，它的第一个参数 ``cx`` 由调用规范要从 a0 中获取。而 Trap 处理函数
  ``trap_handler`` 需要 Trap 上下文的原因在于：它需要知道其中某些寄存器的值，比如在系统调用的时候应用程序传过来的 syscall ID 和
  对应参数。我们不能直接使用这些寄存器现在的值，因为它们可能已经被修改了，因此要去内核栈上找已经被保存下来的值。


.. _term-atomic-instruction:

.. note::

    **CSR 相关原子指令**

    RISC-V 中读写 CSR 的指令是一类能不会被打断地完成多个读写操作的指令。这种不会被打断地完成多个操作的指令被称为 **原子指令** (Atomic Instruction)。这里的 **原子** 的含义是“不可分割的最小个体”，也就是说指令的多个操作要么都不完成，要么全部完成，而不会处于某种中间状态。

当 ``trap_handler`` 返回之后会从调用 ``trap_handler`` 的下一条指令开始执行，也就是从栈上的 Trap 上下文恢复的 ``__restore`` ：

.. _code-restore:

.. code-block:: riscv
    :linenos:

    .macro LOAD_GP n
        ld x\n, \n*8(sp)
    .endm

    __restore:
        # case1: start running app by __restore
        # case2: back to U after handling trap
        mv sp, a0
        # now sp->kernel stack(after allocated), sscratch->user stack
        # restore sstatus/sepc
        ld t0, 32*8(sp)
        ld t1, 33*8(sp)
        ld t2, 2*8(sp)
        csrw sstatus, t0
        csrw sepc, t1
        csrw sscratch, t2
        # restore general-purpuse registers except sp/tp
        ld x1, 1*8(sp)
        ld x3, 3*8(sp)
        .set n, 5
        .rept 27
            LOAD_GP %n
            .set n, n+1
        .endr
        # release TrapContext on kernel stack
        addi sp, sp, 34*8
        # now sp->kernel stack, sscratch->user stack
        csrrw sp, sscratch, sp
        sret

- 第 8 行比较奇怪，我们暂且不管，假设它从未发生，那么 sp 仍然指向内核栈的栈顶。
- 第 11~24 行负责从内核栈顶的 Trap 上下文恢复通用寄存器和 CSR 。注意我们要先恢复 CSR 再恢复通用寄存器，这样我们使用的三个临时寄存器
  才能被正确恢复。
- 在第 26 行之前，sp 指向保存了 Trap 上下文之后的内核栈栈顶， sscratch 指向用户栈栈顶。我们在第 26 行在内核栈上回收 Trap 上下文所
  占用的内存，回归进入 Trap 之前的内核栈栈顶。第 27 行，再次交换 sscratch 和 sp，现在 sp 重新指向用户栈栈顶，sscratch 也依然保存
  进入 Trap 之前的状态并指向内核栈栈顶。
- 在应用程序控制流状态被还原之后，第 28 行我们使用 ``sret`` 指令回到 U 特权级继续运行应用程序控制流。

Trap 分发与处理
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Trap 在使用 Rust 实现的 ``trap_handler`` 函数中完成分发和处理：

.. code-block:: rust
    :linenos:

    // os/src/trap/mod.rs

    #[no_mangle]
    pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
        let scause = scause::read();
        let stval = stval::read();
        match scause.cause() {
            Trap::Exception(Exception::UserEnvCall) => {
                cx.sepc += 4;
                cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
            }
            Trap::Exception(Exception::StoreFault) |
            Trap::Exception(Exception::StorePageFault) => {
                println!("[kernel] PageFault in application, core dumped.");
                run_next_app();
            }
            Trap::Exception(Exception::IllegalInstruction) => {
                println!("[kernel] IllegalInstruction in application, core dumped.");
                run_next_app();
            }
            _ => {
                panic!("Unsupported trap {:?}, stval = {:#x}!", scause.cause(), stval);
            }
        }
        cx
    }

- 第 4 行声明返回值为 ``&mut TrapContext`` 并在第 25 行实际将传入的 ``cx`` 原样返回，因此在 ``__restore`` 的时候 ``a0`` 寄存器在调用
  ``trap_handler`` 前后并没有发生变化，仍然指向分配 Trap 上下文之后的内核栈栈顶，和此时 ``sp`` 的值相同，我们 :math:`\text{sp}\leftarrow\text{a}_0`
  并不会有问题；
- 第 7 行根据 ``scause`` 寄存器所保存的 Trap 的原因进行分发处理。这里我们无需手动操作这些 CSR ，而是使用 Rust 第三方库 riscv 。
- 第 8~11 行，发现触发 Trap 的原因是来自 U 特权级的 Environment Call，也就是系统调用。这里我们首先修改保存在内核栈上的 Trap 上下文里面
  sepc，让其增加 4。这是因为我们知道这是一个由 ``ecall`` 指令触发的系统调用，在进入 Trap 的时候，硬件会将 sepc 设置为这条 ``ecall``
  指令所在的地址（因为它是进入 Trap 之前最后一条执行的指令）。而在 Trap 返回之后，我们希望应用程序控制流从 ``ecall`` 的下一条指令
  开始执行。因此我们只需修改 Trap 上下文里面的 sepc，让它增加 ``ecall`` 指令的码长，也即 4 字节。这样在 ``__restore`` 的时候 sepc
  在恢复之后就会指向 ``ecall`` 的下一条指令，并在 ``sret`` 之后从那里开始执行。

  用来保存系统调用返回值的 a0 寄存器也会同样发生变化。我们从 Trap 上下文取出作为 syscall ID 的 a7 和系统调用的三个参数 a0~a2 传给
  ``syscall`` 函数并获取返回值。 ``syscall`` 函数是在 ``syscall`` 子模块中实现的。 这段代码是处理正常系统调用的控制逻辑。
- 第 12~20 行，分别处理应用程序出现访存错误和非法指令错误的情形。此时需要打印错误信息并调用 ``run_next_app`` 直接切换并运行下一个
  应用程序。
- 第 21 行开始，当遇到目前还不支持的 Trap 类型的时候，批处理操作系统整个 panic 报错退出。

对于系统调用而言， ``syscall`` 函数并不会实际处理系统调用，而只是根据 syscall ID 分发到具体的处理函数：

.. code-block:: rust
    :linenos:

    // os/src/syscall/mod.rs

    pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
        match syscall_id {
            SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
            SYSCALL_EXIT => sys_exit(args[0] as i32),
            _ => panic!("Unsupported syscall_id: {}", syscall_id),
        }
    }

这里我们会将传进来的参数 ``args`` 转化成能够被具体的系统调用处理函数接受的类型。它们的实现都非常简单：

.. code-block:: rust
    :linenos:

    // os/src/syscall/fs.rs

    const FD_STDOUT: usize = 1;

    pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
        match fd {
            FD_STDOUT => {
                let slice = unsafe { core::slice::from_raw_parts(buf, len) };
                let str = core::str::from_utf8(slice).unwrap();
                print!("{}", str);
                len as isize
            },
            _ => {
                panic!("Unsupported fd in sys_write!");
            }
        }
    }

    // os/src/syscall/process.rs

    pub fn sys_exit(xstate: i32) -> ! {
        println!("[kernel] Application exited with code {}", xstate);
        run_next_app()
    }

- ``sys_write`` 我们将传入的位于应用程序内的缓冲区的开始地址和长度转化为一个字符串 ``&str`` ，然后使用批处理操作系统已经实现的 ``print!``
  宏打印出来。这里我们并没有检查传入参数的安全性，存在安全隐患。
- ``sys_exit`` 打印退出的应用程序的返回值并同样调用 ``run_next_app`` 切换到下一个应用程序。

.. _ch2-app-execution:

执行应用程序
-------------------------------------

当批处理操作系统初始化完成，或者是某个应用程序运行结束或出错的时候，我们要调用 ``run_next_app`` 函数切换到下一个应用程序。此时 CPU 运行在
S 特权级，而它希望能够切换到 U 特权级。在 RISC-V 架构中，唯一一种能够使得 CPU 特权级下降的方法就是通过 Trap 返回系列指令，比如
``sret`` 。事实上，在运行应用程序之前要完成如下这些工作：

- 跳转到应用程序入口点 ``0x80400000``；
- 将使用的栈切换到用户栈；
- 在 ``__alltraps`` 时我们要求 ``sscratch`` 指向内核栈，这个也需要在此时完成；
- 从 S 特权级切换到 U 特权级。

它们可以通过复用 ``__restore`` 的代码来更容易的实现上述工作。我们只需要在内核栈上压入一个为启动应用程序而特殊构造的 Trap 上下文，再通过 ``__restore`` 函数，就能
让这些寄存器到达启动应用程序所需要的上下文状态。

.. code-block:: rust
    :linenos:

    // os/src/trap/context.rs

    impl TrapContext {
        pub fn set_sp(&mut self, sp: usize) { self.x[2] = sp; }
        pub fn app_init_context(entry: usize, sp: usize) -> Self {
            let mut sstatus = sstatus::read();
            sstatus.set_spp(SPP::User);
            let mut cx = Self {
                x: [0; 32],
                sstatus,
                sepc: entry,
            };
            cx.set_sp(sp);
            cx
        }
    }

为 ``TrapContext`` 实现 ``app_init_context`` 方法，修改其中的 sepc 寄存器为应用程序入口点 ``entry``， sp 寄存器为我们设定的
一个栈指针，并将 sstatus 寄存器的 ``SPP`` 字段设置为 User 。

在 ``run_next_app`` 函数中我们能够看到：

.. code-block:: rust
    :linenos:

    // os/src/batch.rs

    pub fn run_next_app() -> ! {
        let mut app_manager = APP_MANAGER.exclusive_access();
        let current_app = app_manager.get_current_app();
        unsafe {
            app_manager.load_app(current_app);
        }
        app_manager.move_to_next_app();
        drop(app_manager);
        // before this we have to drop local variables related to resources manually
        // and release the resources
        extern "C" {
            fn __restore(cx_addr: usize);
        }
        unsafe {
            __restore(KERNEL_STACK.push_context(TrapContext::app_init_context(
                APP_BASE_ADDRESS,
                USER_STACK.get_sp(),
            )) as *const _ as usize);
        }
        panic!("Unreachable in batch::run_current_app!");
    }


``__restore`` 所做的事情是在内核栈上压入一个 Trap 上下文，其 ``sepc`` 是应用程序入口地址 ``0x80400000`` ，其 ``sp`` 寄存器指向用户栈，其 ``sstatus``
的 ``SPP`` 字段被设置为 User 。
``push_context`` 的返回值是内核栈压入 Trap 上下文之后的栈顶，它会被作为 ``__restore`` 的参数（
回看 :ref:`__restore 代码 <code-restore>` ，这时我们可以理解为何 ``__restore`` 函数的起始部分会完成
:math:`\text{sp}\leftarrow\text{a}_0` ），这使得在 ``__restore`` 函数中 ``sp`` 仍然可以指向内核栈的栈顶。这之后，就和执行一次普通的
``__restore`` 函数调用一样了。

.. note::

    有兴趣的读者可以思考： sscratch 是何时被设置为内核栈顶的？



..
   马老师发生甚么事了？
   --
   这里要说明目前只考虑从 U Trap 到 S ，而实际上 Trap 的要素就有：Trap 之前在哪个特权级，Trap 在哪个特权级处理。这个对于中断和异常
   都是如此，只不过中断可能跟特权级的关系稍微更紧密一点。毕竟中断的类型都是跟特权级挂钩的。但是对于 Trap 而言有一点是共同的，也就是触发
   Trap 不会导致优先级下降。从中断/异常的代理就可以看出从定义上就不允许代理到更低的优先级。而且代理只能逐级代理，目前我们能操作的只有从
   M 代理到 S，其他代理都基本只出现在指令集拓展或者硬件还不支持。中断的情况是，如果是属于某个特权级的中断，不能在更低的优先级处理。事实上
   这个中断只可能在 CPU 处于不会更高的优先级上收到（否则会被屏蔽），而 Trap 之后优先级不会下降（Trap 代理机制决定），这样就自洽了。
   --
   之前提到异常是说需要执行环境功能的原因与某条指令的执行有关。而 Trap 的定义更加广泛一些，就是在执行某条指令之后发现需要执行环境的功能，
   如果是中断的话 Trap 回来之后默认直接执行下一条指令，如果是异常的话硬件会将 sepc 设置为 Trap 发生之前最后执行的那条指令，而异常发生
   的原因不一定和这条指令的执行有关。应该指出的是，在大多数情况下都是和最后这条指令的执行有关。但在缓存的作用下也会出现那种特别极端的情况。
   --
   然后是 Trap 到 S，就有 S 模式的一些相关 CSR，以及从 U Trap 到 S，硬件会做哪些事情（包括触发异常的一瞬间，以及处理完成调用 sret
   之后）。然后指出从用户的视角来看，如果是 ecall 的话， Trap 回来之后应该从 ecall 的下一条指令开始执行，且执行现场不能发生变化。
   所以就需要将应用执行环境保存在内核栈上（还需要换栈！）。栈存在的原因可能是 Trap handler 是一条新的运行在 S 特权级的执行流，所以
   这个可以理解成跨特权级的执行流切换，确实就复杂一点，要保存的内容也相对多一点。而下一章多任务的任务切换是全程发生在 S 特权级的执行流
   切换，所以会简单一点，保存的通用寄存器大概率更少（少在调用者保存寄存器），从各种意义上都很像函数调用。从不同特权级的角度来解释换栈
   是出于安全性，应用不应该看到 Trap 执行流的栈，这样做完之后，虽然理论上可以访问，但应用不知道内核栈的位置应该也有点麻烦。
   --
   然后是 rust_trap 的处理，尤其是奇妙的参数传递，内部处理逻辑倒是非常简单。
   --
   最后是如何利用 __restore 初始化应用的执行环境，包括如何设置入口点、用户栈以及保证在 U 特权级执行。





