.. rCore-Tutorial-Guide-2022S documentation master file, created by
   sphinx-quickstart on Thu Oct 29 22:25:54 2020.
   You can adapt this file completely to your liking, but it should at least
   contain the root `toctree` directive.

2022年开源操作系统训练营 
==================================================

.. toctree::
   :maxdepth: 2
   :caption: 正文
   :hidden:

   0setup-devel-env
   chapter1/index
   chapter2/index
   chapter3/index
   chapter4/index
   chapter5/index
   chapter6/index
   chapter7/index
   chapter8/index

.. toctree::
   :maxdepth: 2
   :caption: 附录
   :hidden:

   appendix-a/index
   appendix-b/index
   appendix-c/index
   appendix-d/index

.. toctree::
   :maxdepth: 2
   :caption: 开发注记
   :hidden:

   setup-sphinx
   rest-example


项目简介
---------------------

本教程展示了如何 **从零开始** 用 **Rust** 语言写一个基于 **RISC-V** 架构的 **类 Unix 内核** 。

用于 2022年开源操作系统训练营。

导读
---------------------

请先阅读 :doc:`0setup-devel-env` 完成环境配置。

以下是读者为了完成实验需掌握的技术，你可以在实操中熟悉它们。

- 阅读简单的 Makefile 文件；
- 阅读简单的 RISC-V 汇编代码；
- git 的基本功能，解决 git merge 冲突的办法；
- Rust 基本语法和一些进阶语法，包括 **Cargo 项目结构、Trait、函数式编程、Unsafe Rust、错误处理等** 。

鸣谢
----------------------
本项目基于 `2022 年春季学期操作系统实验指导书 <https://github.com/LearningOS/rCore-Tutorial-Code-2022S>`_ ，重构的目标是在保留结构的基础上屏蔽不必要的细节，缩短篇幅，优化语言，降低阅读成本。

如果你觉得本教程某些章节不够细致或不够连贯，可以参考 `2022春季OS课程讲义 rCore Tutorial v3 Guide <https://rcore-os.github.io/rCore-Tutorial-Book-v3/>`_ 。

.. note::

   这是一个注解，以这种方式出现的卡片提供了非必要的背景知识，你可以选择忽略。


.. attention::

   虽然实验本身在总评中占比有限，但根据往届经验，考试中可能大量出现与编程作业、思考题、代码实现思路直接相关的题目。


项目协作
----------------------

- :doc:`/setup-sphinx` 介绍了如何基于 Sphinx 框架配置文档开发环境，之后可以本地构建并渲染 html 或其他格式的文档；
- :doc:`/rest-example` 给出了目前编写文档才用的 ReStructuredText 标记语言的一些基础语法及用例；
- 时间仓促，本项目还有很多不完善之处，欢迎大家积极在每一个章节的评论区留言，或者提交 Issues 或 Pull Requests，让我们
  一起努力让这本书变得更好！

