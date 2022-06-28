修改和构建本项目
====================================

.. toctree::
   :hidden:
   :maxdepth: 4
   
TL;DR: ``python -m venv .venv`` 创建一个虚拟环境（你也可以使用 conda 等工具），activate 后 ``pip install -r requirements.txt``。

1. 参考 `这里 <https://www.sphinx-doc.org/en/master/usage/installation.html>`_ 安装 Sphinx。
2. ``pip install sphinx_rtd_theme`` 安装 Read The Docs 主题。
3. ``pip install jieba`` 安装中文分词。
4. ``pip install sphinx-comments`` 安装 Sphinx 讨论区插件。
5. :doc:`/rest-example` 是 ReST 的一些基本语法，也可以参考已完成的文档。
6. 修改之后，在项目根目录下 ``make clean && make html`` 即可在 ``build/html/index.html`` 查看本地构建的主页。请注意在修改章节目录结构之后需要 ``make clean`` 一下，不然可能无法正常更新。
7. 确认修改无误之后，将更改提交到自己的仓库，然后向项目仓库提交 Pull Request。如有问题，可直接提交 Issue 或课程微信群内联系助教。
