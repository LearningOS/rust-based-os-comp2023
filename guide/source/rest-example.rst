reStructuredText 基本语法
=====================================================

.. toctree::
   :hidden:
   :maxdepth: 4
   
.. note::
   下面是一个注记。

   `这里 <https://www.sphinx-doc.org/en/master/usage/restructuredtext/basics.html#hyperlinks>`_ 给出了在 Sphinx 中
   外部链接的引入方法。注意，链接的名字和用一对尖括号包裹起来的链接地址之间必须有一个空格。链接最后的下划线和片段的后续内容之间也需要
   有一个空格。

   接下来是一个文档内部引用的例子。比如，戳 :doc:`chapter0/5setup-devel-env` 可以进入快速上手环节。

.. warning::

   下面是一个警告。

   .. code-block:: rust
      :linenos:
      :caption: 一段示例 Rust 代码

      // 我们甚至可以插入一段 Rust 代码！
      fn add(a: i32, b: i32) -> i32 { a + b }

   下面继续我们的警告。

.. attention:: Here is an attention.

.. caution:: please be cautious!

.. error::

   下面是一个错误。

.. danger:: it is dangerous!


.. tip:: here is a tip

.. important:: this is important!

.. hint:: this is a hint.



这里是一行数学公式 :math:`\sin(\alpha+\beta)=\sin\alpha\cos\beta+\cos\alpha\sin\beta`。

基本的文本样式：这是 *斜体* ，这是 **加粗** ，接下来的则是行间公式 ``a0`` 。它们的前后都需要有一个空格隔开其他内容，这个让人挺不爽的...

`这是 <https://docs.readthedocs.io/en/stable/guides/cross-referencing-with-sphinx.html#the-doc-role>`_ 一个全面展示
章节分布的例子，来自于 ReadTheDocs 的官方文档。事实上，现在我们也采用 ReadTheDocs 主题了，它非常美观大方。

下面是一个测试 gif。

.. image:: resources/test.gif

接下来是一个表格的例子。

.. list-table:: RISC-V 函数调用跳转指令
   :widths: 20 30
   :header-rows: 1
   :align: center

   * - 指令
     - 指令功能
   * - :math:`\text{jal}\ \text{rd},\ \text{imm}[20:1]`
     - :math:`\text{rd}\leftarrow\text{pc}+4`

       :math:`\text{pc}\leftarrow\text{pc}+\text{imm}`
   * - :math:`\text{jalr}\ \text{rd},\ (\text{imm}[11:0])\text{rs}`
     - :math:`\text{rd}\leftarrow\text{pc}+4`
       
       :math:`\text{pc}\leftarrow\text{rs}+\text{imm}`