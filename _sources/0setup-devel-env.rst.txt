第零章：实验环境配置
================================

.. toctree::
   :hidden:
   :maxdepth: 4

本节我们将完成环境配置并成功运行 rCore-Tutorial 。整个流程分为下面几个部分：

- OS 环境配置
- Rust 开发环境配置
- Qemu 模拟器安装
- 其他工具安装
- 试运行 rCore-Tutorial

如果你在环境配置中遇到了无法解决的问题，请在本节讨论区留言，我们会尽力提供帮助。



目前，实验主要支持 Ubuntu18.04/20.04 操作系统。使用 Windows10 和 macOS 的读者，可以安装一台 Ubuntu18.04 虚拟机或 Docker
进行实验。也可基于 **github classroom with codespaces** 进行开发。


Github Classroom方式进行在线OS 环境配置
-----------------------------------------------------------

.. note::

   **基于github classroom的在线开发方式**
   
   基于github classroom，可方便建立开发用的git repository，并可基于github的 codespace （在线版ubuntu +vscode）在线开发使用。整个开发环境仅仅需要一个网络浏览器。

   1. 在网络浏览器中用自己的 github  id 登录 github.com
   2. 接收 `第一个实验练习 setup-env-run-os1 的github classroom在线邀请 <https://classroom.github.com/a/hnoWuKGF>`_  ，根据提示一路选择OK即可。
   3. 完成第二步后，你的第一个实验练习 setup-env-run-os1 的 github repository 会被自动建立好，点击此github repository的链接，就可看到你要完成的第一个实验了。
   4. 在你的第一个实验练习的网页的中上部可以看到一个醒目的 `code`  绿色按钮，点击后，可以进一步看到  `codespace` 标签和醒目的 `create codesapce on main` 绿色按钮。请点击这个绿色按钮，就可以进入到在线的ubuntu +vscode环境中
   5. 再按照下面的环境安装提示在vscode的 `console` 中安装配置开发环境：rustc，qemu等工具。注：也可在vscode的 `console` 中执行 ``make codespaces_setenv`` 来自动安装配置开发环境（执行``sudo``需要root权限，仅需要执行一次）。
   6. **重要：** 在vscode的 `console` 中执行 `make setupclassroom_testX`  （该命令仅执行一次，X的范围为 1-8）配置githubclassroom 自动评分功能。
   7. 然后就可以基于在线vscode进行开发、运行、提交等完整的实验过程了。

   上述的3，4，5步不是必须的，你也可以仅仅基于 ``Github Classromm`` 生成git repository，并进行本地开发。



Docker方式进行本地OS开发环境配置
-------------------------------------------------

.. note::

   **Docker 开发环境**

 

   使用方法如下（以 Ubuntu20.04 为例）：

   1.  以 ``lab0-0`` 为例，下载克隆本次的实验repo 或 https://github.com/LearningOS/rust-based-os-comp2022.git ； 在 ``lab0-0`` repo的根目录下，执行 ``make build_docker`` 来建立基于docker的开发环境;
   2. 在 ``lab0-0`` repo的根目录下执行 ``make docker`` 进入到 Docker 开发环境；
   3. 进入 Docker 之后，会发现当前处于根目录 ``/`` ，我们通过 ``cd os1`` 将当前工作路径切换到  ``lab0-0`` repo的根目录下；
   4. 接下来就可以在这个环境下进行实验了。例如 ``LOG=DEBUG make run`` 。

   大致操作和输出如下：

   .. code-block:: bash

      $ make build_docker
      $ make docker
      docker$ cd os1
      docker$ LOG=DEBUG make run
      ...
      [rustsbi] RustSBI version 0.2.2, adapting to RISC-V SBI v1.0.0
      .______       __    __      _______.___________.  _______..______   __
      |   _  \     |  |  |  |    /       |           | /       ||   _  \ |  |
      |  |_)  |    |  |  |  |   |   (----`---|  |----`|   (----`|  |_)  ||  |
      |      /     |  |  |  |    \   \       |  |      \   \    |   _  < |  |
      |  |\  \----.|  `--'  |.----)   |      |  |  .----)   |   |  |_)  ||  |
      | _| `._____| \______/ |_______/       |__|  |_______/    |______/ |__|
      [rustsbi] Implementation     : RustSBI-QEMU Version 0.1.1
      [rustsbi] Platform Name      : riscv-virtio,qemu
      [rustsbi] Platform SMP       : 1
      [rustsbi] Platform Memory    : 0x80000000..0x88000000
      [rustsbi] Boot HART          : 0
      [rustsbi] Device Tree Region : 0x87000000..0x87000ef2
      [rustsbi] Firmware Address   : 0x80000000
      [rustsbi] Supervisor Address : 0x80200000
      [rustsbi] pmp01: 0x00000000..0x80000000 (-wr)
      [rustsbi] pmp02: 0x80000000..0x80200000 (---)
      [rustsbi] pmp03: 0x80200000..0x88000000 (xwr)
      Hello, world!
      [DEBUG] .rodata [0x80203000, 0x80205000)
      [ INFO] .data [0x80205000, 0x80206000)
      [ WARN] boot_stack [0x80206000, 0x80216000)
      [ERROR] .bss [0x80216000, 0x80217000)

   注：

      - 感谢 qobilidop， dinghao188 和张汉东老师帮忙配置好的 Docker 开发环境，进入 Docker 开发环境之后不需要任何软件工具链的安装和配置，可以直接将 tutorial 运行起来，目前应该仅支持将 本次实验 运行在 Qemu-7.0.0 模拟器上。
      - 目前的Docker开发环境主要是建立好了实验用的开发环境，还没有设置国内crates源（可选） 、配置基于github classroom的自动测试环境等 。所以还需要参考上面或下面的步骤进行部分配置和部分安装。


手动方式进行本地OS开发环境配置
----------------------------------------------

 注：如果是本地的ubuntu中建立开发环境，可在shell中执行 ``make ubuntu_local_setenv`` 来快速安装配置开发环境（执行``sudo``需要root权限，仅需要执行一次）。

 当然，也可以通过如下详细介绍，一步一步地手动配置开发环境。


Windows10 用户可以通过系统内置的 **WSL2** 虚拟机（请不要使用 WSL1）来安装 Ubuntu 18.04 / 20.04 。读者请自行在互联网上搜索相关安装教程，或 `适用于 Linux 的 Windows 子系统安装指南 (Windows 10) <https://docs.microsoft.com/zh-cn/windows/wsl/install-win10#step-4---download-the-linux-kernel-update-package>`_ 。




使用 macOS 进行实验理论上也是可行的，但本章节仅介绍 Ubuntu 下的环境配置方案。

.. note::

   经初步测试，使用 M1 芯片的 macOS 也可以运行本实验的框架，即我们的实验对平台的要求不是很高。但我们仍建议同学配置 Ubuntu 环境，以避免未知的环境问题。


Rust 开发环境配置
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

首先安装 Rust 版本管理器 rustup 和 Rust 包管理器 cargo，可以使用官方安装脚本：

.. code-block:: bash

   curl https://sh.rustup.rs -sSf | sh

如果因网络问题通过命令行下载脚本失败了，可以在浏览器地址栏中输入 `<https://sh.rustup.rs>`_ 将脚本下载到本地运行。或者使用字节跳动提供的镜像源。

建议将 rustup 的镜像地址修改为中科大的镜像服务器，以加速安装：

.. code-block:: bash

   export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
   export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
   curl https://sh.rustup.rs -sSf | sh

或者使用 tuna 源来加速（建议清华同学在校园网中使用） `参见 rustup 帮助 <https://mirrors.tuna.tsinghua.edu.cn/help/rustup/>`_：

.. code-block:: bash

   export RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup
   export RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup
   curl https://sh.rustup.rs -sSf | sh

也可以设置科学上网代理：

.. code-block:: bash

   # e.g. Shadowsocks 代理，请根据自身配置灵活调整下面的链接
   export https_proxy=http://127.0.0.1:1080
   export http_proxy=http://127.0.0.1:1080
   export ftp_proxy=http://127.0.0.1:1080

安装中全程选择默认选项即可。

安装完成后，我们可以重新打开一个终端来让新设置的环境变量生效，也可以手动将环境变量设置应用到当前终端，
只需输入以下命令：

.. code-block:: bash

   source $HOME/.cargo/env

确认一下我们正确安装了 Rust 工具链：

.. code-block:: bash

   rustc --version

最好把 Rust 包管理器 cargo 镜像地址 crates.io 也替换成中国科学技术大学的镜像服务器，来加速三方库的下载。
打开或新建 ``~/.cargo/config`` 文件，并把内容修改为：

.. code-block:: toml

   [source.crates-io]
   registry = "https://github.com/rust-lang/crates.io-index"
   replace-with = 'ustc'
   [source.ustc]
   registry = "git://mirrors.ustc.edu.cn/crates.io-index"

同样，也可以使用tuna源 `参见 crates.io 帮助 <https://mirrors.tuna.tsinghua.edu.cn/help/crates.io-index.git/>`_：

.. code-block:: toml

   [source.crates-io]
   replace-with = 'tuna'

   [source.tuna]
   registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"


推荐 Visual Studio Code 搭配 rust-analyzer 和 RISC-V Support 插件 进行代码阅读和开发。

也可采用 JetBrains Clion + Rust插件进行代码阅读和开发。

.. note::

   * JetBrains Clion是付费商业软件，但对于学生和教师，只要在 JetBrains 网站注册账号，可以享受一定期限（半年左右）的免费使用的福利。
   * Visual Studio Code 是开源软件。
   * 当然，采用 VIM，Emacs 等传统的编辑器也是没有问题的。

Qemu 模拟器安装
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

我们需要使用 Qemu 7.0.0 以上版本进行实验，为此，从源码手动编译安装 Qemu 模拟器：


.. code-block:: bash

   # 安装编译所需的依赖包
   sudo apt install autoconf automake autotools-dev curl libmpc-dev libmpfr-dev libgmp-dev \
                 gawk build-essential bison flex texinfo gperf libtool patchutils bc \
                 zlib1g-dev libexpat-dev pkg-config  libglib2.0-dev libpixman-1-dev git tmux python3 ninja-build
   # 下载源码包
   # 如果下载速度过慢可以使用我们提供的百度网盘链接：https://pan.baidu.com/s/1z-iWIPjxjxbdFS2Qf-NKxQ
   # 提取码 8woe
   wget https://download.qemu.org/qemu-7.0.0.tar.xz
   # 解压
   tar xvJf qemu-7.0.0.tar.xz
   # 编译安装并配置 RISC-V 支持
   cd qemu-7.0.0
   ./configure --target-list=riscv64-softmmu,riscv64-linux-user
   make -j$(nproc)

.. note::

   注意，上面的依赖包可能并不完全，比如在 Ubuntu 18.04 上：

   - 出现 ``ERROR: pkg-config binary 'pkg-config' not found`` 时，可以安装 ``pkg-config`` 包；
   - 出现 ``ERROR: glib-2.48 gthread-2.0 is required to compile QEMU`` 时，可以安装
     ``libglib2.0-dev`` 包；
   - 出现 ``ERROR: pixman >= 0.21.8 not present`` 时，可以安装 ``libpixman-1-dev`` 包。

   另外一些 Linux 发行版编译 Qemu 的依赖包可以从 `这里 <https://risc-v-getting-started-guide.readthedocs.io/en/latest/linux-qemu.html#prerequisites>`_
   找到。请自行选择合适的编译器版本正常编译 Qemu。

之后我们可以在同目录下 ``sudo make install`` 将 Qemu 安装到 ``/usr/local/bin`` 目录下，但这样经常会引起
冲突。个人来说更习惯的做法是，编辑 ``~/.bashrc`` 文件（如果使用的是默认的 ``bash`` 终端），在文件的末尾加入
几行：

.. code-block:: bash

   # 请注意，qemu-7.0.0 的父目录可以随着你的实际安装位置灵活调整
   export PATH=$PATH:/home/shinbokuow/Downloads/built/qemu-7.0.0
   export PATH=$PATH:/home/shinbokuow/Downloads/built/qemu-7.0.0/riscv64-softmmu
   export PATH=$PATH:/home/shinbokuow/Downloads/built/qemu-7.0.0/riscv64-linux-user

随后即可在当前终端 ``source ~/.bashrc`` 更新系统路径，或者直接重启一个新的终端。

确认 Qemu 的版本：

.. code-block:: bash

   qemu-system-riscv64 --version
   qemu-riscv64 --version

试运行 rCore-Tutorial
------------------------------------------------------------

基于Github Classroom 模式
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: bash

   $ git clone  ``Github-Classroom帮你生成的某个OS实验的仓库``
   $ cd  ``刚克隆的本地某个OS实验的仓库``
   $ make setupclassroom_``实验编号``  //注意：这一步很重要，是用于github classroom自动评测你的工作。这一步只需在首次克隆项目仓库时执行一次，以后一般就不用执行了，除非 .github/workflows/classroom.yml发生了变化。实验编号是与某个实验匹配的编号


.. note::

   实验名称 ：实验编号 
   
   -  lab0-0 : test1
   -  lab0-1：test2 
   -  lab1：test3
   -  lab2：test4
   -  lab3：test5
   -  lab4：test6
   -  lab5：test8
 
   
我们先运行不需要处理用户代码的裸机操作系统 ``os1`` ：

.. code-block:: bash

   cd os1
   LOG=DEBUG make run

如果你的环境配置正确，你应当会看到如下输出：

.. code-block:: bash

   [rustsbi] RustSBI version 0.2.2, adapting to RISC-V SBI v1.0.0
   .______       __    __      _______.___________.  _______..______   __
   |   _  \     |  |  |  |    /       |           | /       ||   _  \ |  |
   |  |_)  |    |  |  |  |   |   (----`---|  |----`|   (----`|  |_)  ||  |
   |      /     |  |  |  |    \   \       |  |      \   \    |   _  < |  |
   |  |\  \----.|  `--'  |.----)   |      |  |  .----)   |   |  |_)  ||  |
   | _| `._____| \______/ |_______/       |__|  |_______/    |______/ |__|
   [rustsbi] Implementation     : RustSBI-QEMU Version 0.1.1
   [rustsbi] Platform Name      : riscv-virtio,qemu
   [rustsbi] Platform SMP       : 1
   [rustsbi] Platform Memory    : 0x80000000..0x88000000
   [rustsbi] Boot HART          : 0
   [rustsbi] Device Tree Region : 0x87000000..0x87000ef2
   [rustsbi] Firmware Address   : 0x80000000
   [rustsbi] Supervisor Address : 0x80200000
   [rustsbi] pmp01: 0x00000000..0x80000000 (-wr)
   [rustsbi] pmp02: 0x80000000..0x80200000 (---)
   [rustsbi] pmp03: 0x80200000..0x88000000 (xwr)
   Hello, world!
   [DEBUG] .rodata [0x80203000, 0x80205000)
   [ INFO] .data [0x80205000, 0x80206000)
   [ WARN] boot_stack [0x80206000, 0x80216000)
   [ERROR] .bss [0x80216000, 0x80217000)
   Panicked at src/main.rs:48 Shutdown machine!

通常 rCore 会自动关闭 Qemu 。如果在某些情况下需要强制结束，可以先按下 ``Ctrl+A`` ，再按下 ``X`` 来退出 Qemu。

.. attention::

   请务必执行 ``make run``，这将为你安装一些上文没有提及的 Rust 包依赖。

   如果卡在了

   .. code-block::

      Updating git repository `https://github.com/rcore-os/riscv`

   请通过更换 hosts 等方式解决科学上网问题，或者将 riscv 项目下载到本地，并修改 os/Cargo.toml 中的 riscv 包依赖路径

   .. code-block::

      [dependencies]
      riscv = { path = "YOUR riscv PATH", features = ["inline-asm"] }

恭喜你完成了实验环境的配置，可以开始阅读教程的正文部分了！

GDB 调试支持*
------------------------------

.. attention::

   使用 GDB debug 并不是必须的，你可以暂时跳过本小节。



在 ``os`` 目录下 ``make debug`` 可以调试我们的内核，这需要安装终端复用工具 ``tmux`` ，还需要基于 riscv64 平台的 gdb 调试器 ``riscv64-unknown-elf-gdb`` 。该调试器包含在 riscv64 gcc 工具链中，工具链的预编译版本可以在如下链接处下载：

- `Ubuntu 平台 <https://static.dev.sifive.com/dev-tools/riscv64-unknown-elf-gcc-8.3.0-2020.04.1-x86_64-linux-ubuntu14.tar.gz>`_
- `macOS 平台 <https://static.dev.sifive.com/dev-tools/riscv64-unknown-elf-gcc-8.3.0-2020.04.1-x86_64-apple-darwin.tar.gz>`_
- `Windows 平台 <https://static.dev.sifive.com/dev-tools/riscv64-unknown-elf-gcc-8.3.0-2020.04.1-x86_64-w64-mingw32.zip>`_
- `CentOS 平台 <https://static.dev.sifive.com/dev-tools/riscv64-unknown-elf-gcc-8.3.0-2020.04.1-x86_64-linux-centos6.tar.gz>`_

解压后在 ``bin`` 目录下即可找到 ``riscv64-unknown-elf-gdb`` 以及另外一些常用工具 ``objcopy/objdump/readelf`` 等。
