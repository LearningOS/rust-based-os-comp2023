分时多任务系统
===========================================================


现代的任务调度算法基本都是抢占式的，它要求每个应用只能连续执行一段时间，然后内核就会将它强制性切换出去。
一般将 **时间片** (Time Slice) 作为应用连续执行时长的度量单位，每个时间片可能在毫秒量级。
简单起见，我们使用 **时间片轮转算法** (RR, Round-Robin) 来对应用进行调度。


时钟中断与计时器
------------------------------------------------------------------

实现调度算法需要计时。RISC-V 要求处理器维护时钟计数器 ``mtime``，还有另外一个 CSR ``mtimecmp`` 。
一旦计数器 ``mtime`` 的值超过了 ``mtimecmp``，就会触发一次时钟中断。

运行在 M 特权级的 SEE 已经预留了相应的接口，基于此编写的 ``get_time`` 函数可以取得当前 ``mtime`` 计数器的值；

.. code-block:: rust

    // os/src/timer.rs

    use riscv::register::time;

    pub fn get_time() -> usize {
        time::read()
    }

在 10 ms 后设置时钟中断的代码如下：

.. code-block:: rust
    :linenos:

    // os/src/sbi.rs

    const SBI_SET_TIMER: usize = 0;

    pub fn set_timer(timer: usize) {
        sbi_call(SBI_SET_TIMER, timer, 0, 0);
    }

    // os/src/timer.rs

    use crate::config::CLOCK_FREQ;
    const TICKS_PER_SEC: usize = 100;

    pub fn set_next_trigger() {
        set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
    }

- 第 5 行， ``sbi`` 子模块有一个 ``set_timer`` 调用，用来设置 ``mtimecmp`` 的值。
- 第 14 行， ``timer`` 子模块的 ``set_next_trigger`` 函数对 ``set_timer`` 进行了封装，
  它首先读取当前 ``mtime`` 的值，然后计算出 10ms 之内计数器的增量，再将 ``mtimecmp`` 设置为二者的和。
  这样，10ms 之后一个 S 特权级时钟中断就会被触发。

  至于增量的计算方式， ``CLOCK_FREQ`` 是一个预先获取到的各平台不同的时钟频率，单位为赫兹，也就是一秒钟之内计数器的增量。
  它可以在 ``config`` 子模块中找到。10ms 的话只需除以常数 ``TICKS_PER_SEC`` 也就是 100 即可。

后面可能还有一些计时的需求，我们再设计一个函数：

.. code-block:: rust

    // os/src/timer.rs

    const MICRO_PER_SEC: usize = 1_000_000;

    pub fn get_time_us() -> usize {
        time::read() / (CLOCK_FREQ / MICRO_PER_SEC)
    }


``timer`` 子模块的 ``get_time_us`` 可以以微秒为单位返回当前计数器的值。

新增一个系统调用，使应用能获取当前的时间：

.. code-block:: rust
    :caption: 第三章新增系统调用（二）

    /// 功能：获取当前的时间，保存在 TimeVal 结构体 ts 中，_tz 在我们的实现中忽略
    /// 返回值：返回是否执行成功，成功则返回 0
    /// syscall ID：169
    fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize;

结构体 ``TimeVal`` 的定义如下，内核只需调用 ``get_time_us`` 即可实现该系统调用。

.. code-block:: rust

    // os/src/syscall/process.rs

    #[repr(C)]
    pub struct TimeVal {
        pub sec: usize,
        pub usec: usize,
    }

RISC-V 架构中的嵌套中断问题
-----------------------------------

默认情况下，当 Trap 进入某个特权级之后，在 Trap 处理的过程中同特权级的中断都会被屏蔽。

- 当 Trap 发生时，``sstatus.sie`` 会被保存在 ``sstatus.spie`` 字段中，同时 ``sstatus.sie`` 置零，
  这也就在 Trap 处理的过程中屏蔽了所有 S 特权级的中断；
- 当 Trap 处理完毕 ``sret`` 的时候， ``sstatus.sie`` 会恢复到 ``sstatus.spie`` 内的值。

也就是说，如果不去手动设置 ``sstatus`` CSR ，在只考虑 S 特权级中断的情况下，是不会出现 **嵌套中断** (Nested Interrupt) 的。

.. note::

    **嵌套中断与嵌套 Trap**

    嵌套中断可以分为两部分：在处理一个中断的过程中又被同特权级/高特权级中断所打断。默认情况下硬件会避免前一部分，
    也可以通过手动设置来允许前一部分的存在；而从上面介绍的规则可以知道，后一部分则是无论如何设置都不可避免的。

    嵌套 Trap 则是指处理一个 Trap 过程中又再次发生 Trap ，嵌套中断算是嵌套 Trap 的一种。


抢占式调度
-----------------------------------

有了时钟中断和计时器，抢占式调度就很容易实现了：

.. code-block:: rust

    // os/src/trap/mod.rs

    match scause.cause() {
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            suspend_current_and_run_next();
        }
    }

我们只需在 ``trap_handler`` 函数下新增一个分支，触发了 S 特权级时钟中断时，重新设置计时器，
调用 ``suspend_current_and_run_next`` 函数暂停当前应用并切换到下一个。

为了避免 S 特权级时钟中断被屏蔽，我们需要在执行第一个应用前调用 ``enable_timer_interrupt()`` 设置 ``sie.stie``，
使得 S 特权级时钟中断不会被屏蔽；再设置第一个 10ms 的计时器。

.. code-block:: rust
    :linenos:

    // os/src/main.rs

    #[no_mangle]
    pub fn rust_main() -> ! {
        // ...
        trap::enable_timer_interrupt();
        timer::set_next_trigger();
        // ...
    }

    // os/src/trap/mod.rs

    use riscv::register::sie;

    pub fn enable_timer_interrupt() {
        unsafe { sie::set_stimer(); }
    }

就这样，我们实现了时间片轮转任务调度算法。 ``power`` 系列用户程序可以验证我们取得的成果：这些应用并没有主动 yield，
内核仍能公平地把时间片分配给它们。

