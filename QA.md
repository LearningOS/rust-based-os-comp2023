# 常见问题解答

## Q0：GitHub classroom 是啥？如何使用？

### A：

     - [B 站的 GitHub Classroom 视频介绍](https://www.bilibili.com/video/BV12L41147r7?spm_id_from=333.337.search-card.all.click&vd_source=8e19ee6e49f598fda8c17e306d8b3726)
     - [Youtube 的 GitHub Classroom 视频介绍](https://www.youtube.com/playlist?list=PLIRjfNq867bewk3ZGV6Z7a16YDNRCpK3u)
     - [GitHub 文档：使用 GitHub Classroom 教学](https://docs.github.com/cn/education/manage-coursework-with-github-classroom/get-started-with-github-classroom)

## Q1：已经在 classroom 中建立了自己的仓库（例如“LearningOS/lab0-0-setup-env-run-os1-chyyuu2022”），但是源仓库“LearningOS/rust-based-os-comp2022”更新了，如何处理？

### A：

**方法一：**

    重新点击加入课程的链接，在页面下方会有一行字“We've configured the repository associated with this assignment (update)”，“update”是一个链接，点击 update 就可以把自己的仓库更新到与最新状态的 repository template 一致。

**方法二：**

在自己构建的仓库根目录下执行以下命令：

```bash
git remote add upstream "https://github.com/LearningOS/rust-based-os-comp2022.git"
git fetch upstream
git checkout -b foo
git branch -D main
git checkout -t upstream/main
git reset --hard origin/main
git push -f
```

**方法三：**

    向管理员“助教许善朴”申请删除已生成仓库，再点击 链接重新创建仓库。

## Q2：在 classroom 中建立了自己的仓库中，进行提交 `git push` 后，触发 CI 后，出现 Annotations 错误“The job was not stared because recent account payments have failed or your spending limit needs to be increased. Please check the 'Billing & plans' section in your settings”，无法完成自动 CI 功能，比如 `Autograding` 等。

### A：

**方法一：**

    这是由于对用户的私有仓库进行 CI 相关的 GitHub Action 是需要付费的。用户可通过给自己的 github 账户充值来解决。https://docs.github.com/cn/billing/managing-billing-for-github-actions/about-billing-for-github-actions 给出了具体信息。

**方法二：**

    对用户的公开仓库进行 CI GitHub Action 是不需要付费的。在项目的 `Settings` -> `Change visibility` 将项目改成 Public, 重新触发 Action。
    目前设置了让用户具有修改自己的项目从 private --> public 的能力。
    如果用户还是发现自己的权限不够，或看不到  `Settings`  这个选项，可以通过联系助教帮助来解决。

## Q3：我刚开始准备学习 Rust，是 Rust 新手，我应该如何入门？

### A：

- [Rust 大佬给初学者的学习建议](https://github.com/rustlang-cn/Rustt/blob/main/Articles/%5B2022-04-02%5D%20Rust%20%E5%A4%A7%E4%BD%AC%E7%BB%99%E5%88%9D%E5%AD%A6%E8%80%85%E7%9A%84%E5%AD%A6%E4%B9%A0%E5%BB%BA%E8%AE%AE.md)
- [张汉东：学习 Rust 你需要一个认知框架](https://zhuanlan.zhihu.com/p/494001676)
- [Rust 语言圣经（Rust Course）](https://course.rs/)
- [Rust 速查表（cheatsheet）](https://cheats.rs/) 该项目不仅提供了基础的语法速查，还有执行顺序详解和编写时需要关注的注意事项。项目还包含了示例代码（EX）、书籍（BK）、标准（STD）等相关资料的扩展。

## Q4：我不熟悉 GitHub 和 Git，有啥快速入门的资源吗？

### A：

- [包括：从 0 开始学习 GitHub 系列 1-7](https://jtxiao.com/main/categories/%E5%B7%A5%E5%85%B7/)
- [超级简单的 Git 入门](https://backlog.com/git-tutorial/cn/)
- [git - 简明指南](https://rogerdudler.github.io/git-guide/index.zh.html)
- [中文 git-tips](https://github.com/521xueweihan/git-tips)
- [GitHub 官方制作的 Git 速查表](https://education.github.com/git-cheat-sheet-education.pdf)

## Q5：我不熟悉 Linux 的各种命令，有啥快速入门的资源吗？

### A：

- [中文 Linux 命令（linux-command）搜索引擎](https://wangchujiang.com/linux-command/)：随用随搜 Linux 命令，而且还支持中文搜索
- [新版 Linux 命令百科全书》（英文）](https://github.com/tldr-pages/tldr)

## Q6：我碰到一些命令/应用（比如 vim, curl）、操作（比如 vscode）或语言用法（比如 Makefile）等不知到哪里能快速查找，怎么办？

### A：

- [Rico's cheatsheets](https://devhints.io/) 开源、全面的速查表网站，涵盖了前端、后端、运维、IDE 多个方面，而且界面友好简洁支持在线查看
- [所有与命令行相关的 cheatsheet](http://cheat.sh/)：号称「你唯一需要的命令行相关速查表」

## Q7：我可以正常 `make run`，但使用 `make test` 命令后，构件过程报了许多错（`asm!` not found in scope），Autograding 也无法通过，怎么办？

### A：

这是由于内置的 `ci-user/riscv` 代码有错误，在 Autograding 时远程的 `riscv` 依赖被替换，导致编译失败。

**方法一：**

    替换内置的 riscv 至正常版本，直接删除本地 `ci-user/riscv` 文件夹，替换为 [ Yakkhini / rust-based-os-comp2022](https://github.com/Yakkhini/rust-based-os-comp2022/tree/main/ci-user) 同位置的修复版本 `/riscv`。

**方法二：**

    删除 `ci-user/overwrite.py` 21 行以下部分的依赖替换脚本。

**方法三：**

    替换你实验文件夹中 `Cargo.toml` 的 riscv 依赖网址为 `https://GitHub.com/rcore-os/riscv`（修改了网址的大小写）或 `https://gitee.com/rcore-os/riscv`（改为 Gitee 源），使脚本中的替换匹配失效。

**方法四：**

    如果你能看到这个 QA，说明相关 Pull request 已被 merge，可以按 QA1 中方法更新仓库。

## Q8：在用vscode的esbonio （for Sphinx：rst-->html,...）和rust-analyzer插件时，不能正常工作。请问如何配置？

### A：

**esbonio error：找不到 conf.py 文件**

**解决方法**

    在 .vscode/settings.json中添加如下内容：

    "esbonio.sphinx.confDir": "${workspaceFolder}/guide/source"

**rust-analyzer插件无法正常解析源代码中的相关定义**

    比如 os/main.rs中，有如下代码：

    #[cfg(feature = "board_k210")]
    #[path = "boards/k210.rs"]
    mod board;
    #[cfg(not(any(feature = "board_k210")))]
    #[path = "boards/qemu.rs"]
    mod board;

    这使得 mod board 可能是 k210.rs中的代码，也可能是qemu.rs中的代码，取决于编译时的参数，即 os/Makefile 中的：

    @cargo build --release --features "board_$(BOARD)"

    rust-analyzer会报错：unresolved import `crate::drivers::chardev::UART` 等错误

**解决方法**

    在 .vscode/settings.json中添加如下内容：

    // Prevent "can't find crate for `test`" error on no_std
    // Ref: https://github.com/rust-lang/vscode-rust/issues/729
    // For vscode-rust plugin users:
    "rust.target": "riscv64gc-unknown-none-elf",
    "rust.all_targets": false,
    // For Rust Analyzer plugin users:
    "rust-analyzer.cargo.target": "riscv64gc-unknown-none-elf",
    "rust-analyzer.checkOnSave.allTargets": false,
    "rust-analyzer.cargo.features": [
        "board_qemu"
    ]

**rust-analyzer插件无法正常解析repo中多个不同projects中的代码**

**解决方法**

    以本repo为例，在 .vscode/settings.json中添加如下内容：

    "rust-analyzer.linkedProjects": [
        "guide-code/ch1-3mini-rt-usrland/Cargo.toml",
        "os1-ref/Cargo.toml",
        "os2-ref/Cargo.toml",
        "os3-ref/Cargo.toml",
        "os4-ref/Cargo.toml",
        "os5-ref/Cargo.toml",
        "os6-ref/Cargo.toml",
        "os7-ref/Cargo.toml",
        "os8-ref/Cargo.toml",
        "easy-fs/Cargo.toml",
        "easy-fs-fuse/Cargo.toml",
        "user/Cargo.toml",
      ]

    如果还有新的projects想要rust-analyzer分析，参考上面的例子，把projects对应路径加入即可。

**rust-analyzer插件无法正常解析rustlings repo中不同源码**

**解决方法**

访问 <https://crates.io/crates/rustlings-fix>  安装并允许rustlings-fix工具，它会生成一个配置文件  rust-project.json ，然后就可以看了

    # Install rustlings-fix from cargo
    cargo install rustlings-fix

    # Change directory into wherever rustlings is cloned
    cd ~/src/rustlings

    # Run the binary
    rustlings-fix

## Q9：在用vscode中能否像一般应用一样，源码级调试rcore-tutorial-v3？如果可以，如何做？

**方法一：（适合vscode 一般用户）**

请看 [VSCode 可视化调试支持](https://learningos.github.io/rust-based-os-comp2022/0setup-devel-env.html#vscode)。
感谢  @myrfy 米明恒的贡献！

**方法二：（适合vscode熟手）**

目前 @chenzhiy2001 已经有了一个初步的实现方案，<https://github.com/chenzhiy2001/code-debug> 请访问 [coredebugger安装与使用](https://github.com/chenzhiy2001/code-debug#%E5%AE%89%E8%A3%85%E4%B8%8E%E4%BD%BF%E7%94%A8) 了解具体操作过程。@chyyuu 试用后，觉得很不错！

感谢  @chenzhiy2001 陈志扬的贡献！

提示：

1. 目前项目在开发中，还没有设计得对用户特别友好，建议新手慎用。
2. 因为opt-level被设置成0，rCore Tutorial v3在qemu中会比较慢（在耗时的for循环等情况下，要有耐心等待）

**方法三：（适合Linux 命令行熟手）**

请看 [GDB 调试支持](https://learningos.github.io/rust-based-os-comp2022/0setup-devel-env.html#gdb)
