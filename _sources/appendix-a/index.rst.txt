附录 A：Rust 系统编程资料
=============================

.. toctree::
   :hidden:
   :maxdepth: 4


.. .. note::

..     **Rust 语法卡片：外部符号引用**

..     extern "C" 可以引用一个外部的 C 函数接口（这意味着调用它的时候要遵从目标平台的 C 语言调用规范）。但我们这里只是引用位置标志
..     并将其转成 usize 获取它的地址。由此可以知道 ``.bss`` 段两端的地址。

..     **Rust 语法卡片：迭代器与闭包**

..     代码第 7 行用到了 Rust 的迭代器与闭包的语法，它们在很多情况下能够提高开发效率。如读者感兴趣的话也可以将其改写为等价的 for 
..     循环实现。

.. .. _term-raw-pointer:
.. .. _term-dereference:
.. .. warning::

..     **Rust 语法卡片：Unsafe**

..     代码第 8 行，我们将 ``.bss`` 段内的一个地址转化为一个 **裸指针** (Raw Pointer)，并将它指向的值修改为 0。这在 C 语言中是
..     一种司空见惯的操作，但在 Rust 中我们需要将他包裹在 unsafe 块中。这是因为，Rust 认为对于裸指针的 **解引用** (Dereference) 
..     是一种 unsafe 行为。

..     相比 C 语言，Rust 进行了更多的语义约束来保证安全性（内存安全/类型安全/并发安全），这在编译期和运行期都有所体现。但在某些时候，
..     尤其是与底层硬件打交道的时候，在 Rust 的语义约束之内没法满足我们的需求，这个时候我们就需要将超出了 Rust 语义约束的行为包裹
..     在 unsafe 块中，告知编译器不需要对它进行完整的约束检查，而是由程序员自己负责保证它的安全性。当代码不能正常运行的时候，我们往往也是
..     最先去检查 unsafe 块中的代码，因为它没有受到编译器的保护，出错的概率更大。

..     C 语言中的指针相当于 Rust 中的裸指针，它无所不能但又太过于灵活，程序员对其不谨慎的使用常常会引起很多内存不安全问题，最常见的如
..     悬垂指针和多次回收的问题，Rust 编译器没法确认程序员对它的使用是否安全，因此将其划到 unsafe Rust 的领域。在 safe Rust 中，我们
..     有引用 ``&/&mut`` 以及各种功能各异的智能指针 ``Box<T>/RefCell<T>/Rc<T>`` 可以使用，只要按照 Rust 的规则来使用它们便可借助
..     编译器在编译期就解决很多潜在的内存不安全问题。

Rust编程相关
--------------------------------

- `OS Tutorial Summer of Code 2020：Rust系统编程入门指导 <https://github.com/rcore-os/rCore/wiki/os-tutorial-summer-of-code#step-0-%E8%87%AA%E5%AD%A6rust%E7%BC%96%E7%A8%8B%E5%A4%A7%E7%BA%A67%E5%A4%A9>`_
- `Stanford 新开的一门很值得学习的 Rust 入门课程 <https://reberhardt.com/cs110l/spring-2020/>`_
- `一份简单的 Rust 入门介绍 <https://zhuanlan.zhihu.com/p/298648575>`_
- `《RustOS Guide》中的 Rust 介绍部分 <https://simonkorl.gitbook.io/r-z-rustos-guide/dai-ma-zhi-qian/ex1>`_
- `一份简单的Rust宏编程新手指南 <http://blog.hubwiz.com/2020/01/30/rust-macro/>`_


Rust系统编程pattern
---------------------------------

- `Arc<Mutex<_>> in Rust <https://aeshirey.github.io/code/2020/12/23/arc-mutex-in-rust.html>`_
- `Understanding Closures in Rust <https://medium.com/swlh/understanding-closures-in-rust-21f286ed1759>`_
- `Closures in Rust <https://zhauniarovich.com/post/2020/2020-12-closures-in-rust/>`_