chapter8 练习
=======================================

Lab5 编程作业
--------------------------------------

.. warning::

   本次实验框架变动较大，且改动较为复杂，为降低同学们的工作量，本次实验不要求合并之前的实验内容，
   只需通过 ch8 的全部测例和其他章节的基础测例即可。你可以参考  `lab5(os8)参考框架: <https://github.com/LearningOS/rust-based-os-comp2022/tree/main/os8-ref>`_ 上完成以下作业。

.. note::

   本次实验的工作量约为 100 行代码。


死锁检测
+++++++++++++++++++++++++++++++

目前的 mutex 和 semaphore 相关的系统调用不会分析资源的依赖情况，用户程序可能出现死锁。
我们希望在系统中加入死锁检测机制，当发现可能发生死锁时拒绝对应的资源获取请求。
一种检测死锁的算法如下：

定义如下三个数据结构：

- 可利用资源向量 Available ：含有 m 个元素的一维数组，每个元素代表可利用的某一类资源的数目，
  其初值是该类资源的全部可用数目，其值随该类资源的分配和回收而动态地改变。
  Available[j] = k，表示第 j 类资源的可用数量为 k。
- 分配矩阵 Allocation：n * m 矩阵，表示每类资源已分配给每个线程的资源数。
  Allocation[i,j] = g，则表示线程 i 当前己分得第 j 类资源的数量为 g。
- 需求矩阵 Need：n * m 的矩阵，表示每个线程还需要的各类资源数量。
  Need[i,j] = d，则表示线程 i 还需要第 j 类资源的数量为 d 。

算法运行过程如下：

1. 设置两个向量: 工作向量 Work，表示操作系统可提供给线程继续运行所需的各类资源数目，它含有
   m 个元素。初始时，Work = Available ；结束向量 Finish，表示系统是否有足够的资源分配给线程，
   使之运行完成。初始时 Finish[0..n-1] = false，表示所有线程都没结束；当有足够资源分配给线程时，
   设置 Finish[i] = true。
2. 从线程集合中找到一个能满足下述条件的线程

.. code-block:: Rust
   :linenos:

   Finish[i] == false;
   Need[i,j] ≤ Work[j];

若找到，执行步骤 3，否则执行步骤 4。

3. 当线程 thr[i] 获得资源后，可顺利执行，直至完成，并释放出分配给它的资源，故应执行:

.. code-block:: Rust
   :linenos:

   Work[j] = Work[j] + Allocation[i, j];
   Finish[i] = true;

跳转回步骤2

4. 如果 Finish[0..n-1] 都为 true，则表示系统处于安全状态；否则表示系统处于不安全状态，即出现死锁。

出于兼容性和灵活性考虑，我们允许进程按需开启或关闭死锁检测功能。为此我们将实现一个新的系统调用：
``sys_enable_deadlock_detect`` 。

**enable_deadlock_detect**：

* syscall ID:  469
* 功能：为当前进程启用或禁用死锁检测功能。
* C 接口： ``int enable_deadlock_detect(int is_enable)``
* Rust 接口： ``fn enable_deadlock_detect(is_enable: i32) -> i32``
* 参数：
    * is_enable: 为 1 表示启用死锁检测， 0 表示禁用死锁检测。
* 说明：
    * 开启死锁检测功能后， ``mutex_lock`` 和 ``semaphore_down`` 如果检测到死锁，
      应拒绝相应操作并返回 -0xDEAD (十六进制值)。
    * 简便起见可对 mutex 和 semaphore 分别进行检测，无需考虑二者 (以及 ``waittid`` 等)
      混合使用导致的死锁。
* 返回值：如果出现了错误则返回 -1，否则返回 0。
* 可能的错误
    * 参数不合法
    * 死锁检测开启失败


实验要求
+++++++++++++++++++++++++++++++++++++++++

-   `lab5(os8)参考框架: <https://github.com/LearningOS/rust-based-os-comp2022/tree/main/os8-ref>`_ 
- 实验目录在 ``os8`` 。
- 通过所有测例。

问答作业
--------------------------------------------

1. 在我们的多线程实现中，当主线程 (即 0 号线程) 退出时，视为整个进程退出，
   此时需要结束该进程管理的所有线程并回收其资源。
   - 需要回收的资源有哪些？
   - 其他线程的 TaskControlBlock 可能在哪些位置被引用，分别是否需要回收，为什么？
2. 对比以下两种 ``Mutex.unlock`` 的实现，二者有什么区别？这些区别可能会导致什么问题？

.. code-block:: Rust
    :linenos:

    impl Mutex for Mutex1 {
        fn unlock(&self) {
            let mut mutex_inner = self.inner.exclusive_access();
            assert!(mutex_inner.locked);
            mutex_inner.locked = false;
            if let Some(waking_task) = mutex_inner.wait_queue.pop_front() {
                add_task(waking_task);
            }
        }
    }

    impl Mutex for Mutex2 {
        fn unlock(&self) {
            let mut mutex_inner = self.inner.exclusive_access();
            assert!(mutex_inner.locked);
            if let Some(waking_task) = mutex_inner.wait_queue.pop_front() {
                add_task(waking_task);
            } else {
                mutex_inner.locked = false;
            }
        }
    }


报告要求
-------------------------------

- 简单总结你实现的功能（200字以内，不要贴代码）及你完成本次实验所用的时间。
- 完成问答题。
- (optional) 你对本次实验设计及难度/工作量的看法，以及有哪些需要改进的地方，欢迎畅所欲言。
