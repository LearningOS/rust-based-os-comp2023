# 2023秋冬季开源操作系统训练营第一阶段环境配置与学习资料

前提条件： 要求有基本数据结构，算法基础，相对了解或熟悉C语言等编程.

1. 自学基础知识：[阅读书籍/课程/视频等资源汇总](https://github.com/rcore-os/rCore/wiki/study-resource-of-system-programming-in-RUST)
   
   - 推荐：[Rust语言圣经(Rust教程 Rust Course和配套练习)](https://course.rs/)
   - 推荐：[半小时快速了解Rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust)
   - 推荐：[Rust速查表（cheatsheet）](https://cheats.rs/) 该项目不仅提供了基础的语法速查，还有执行顺序详解和编写时需要关注的注意事项。项目还包含了示例代码（EX）、书籍（BK）、标准（STD）等相关资料的扩展。
   - 推荐：[清华计算机系大一学生2022暑期课程：Rust程序设计训练（有课程视频）](https://lab.cs.tsinghua.edu.cn/rust/)

2. 自学编程
   
   - [2023秋
     
     季OS训练营--rustling训练](https://classroom.github.com/a/gbr5Jk7V)（采用Github Classroom模式的Rustling小练习，点击上述链接，形成自己的练习用repo）
     
     - 要求：**必须完成** 。完成所有练习后，执行 ``git add; git commit -m"update"; git push`` 命令，把更新提交到GithubClassroom的CI进行自动评测。要求小练习全部通过GithubClassroom的CI自动评测。
     
     - [学习系列视频：Rust中文社群线上学习室--通过 Rustlings 学 Rust](https://space.bilibili.com/24917186/video)
       **提示：基于github classroom的开发方式**
       基于github classroom，可方便建立开发用的git repository，并可基于github的 codespace（在线版ubuntu +vscode）在线开发使用。整个开发环境仅仅需要一个网络浏览器。
       
       > codespace 不是必须的。如果是本地的ubuntu中建立开发环境，可在shell中执行 `make ubuntu_local_setenv` 来自动安装配置开发环境（执行需要 `sudo` root 权限，仅需要执行一次）。
       
       1. 在网络浏览器中用自己的 github id 登录 github.com。
       
       2. 接收 [Rust-lang Lab Test based on Rustlings 的github classroom在线邀请](https://classroom.github.com/a/gbr5Jk7V)  ，根据提示一路选择OK即可。
       
       3. 完成第二步后，你的rustings实验练习 的 github repository 会被自动建立好，点击此github repository的链接，就可看到你要完成的实验了。
       
       4. 在你的第一个实验练习的网页的中上部可以看到一个醒目的 `code`  绿色按钮，点击后，可以进一步看到 `codespace` 标签和醒目的 `create codesapce on edu` 绿色按钮。请点击这个绿色按钮，就可以进入到在线的ubuntu +vscode环境中
       
       5. 再按照下面的环境安装提示在vscode的 `console` 中安装配置开发环境：rustc等工具。
       
       6. 然后就可以基于在线vscode进行测试 (执行命令 `rustlings watch` ），编辑代码的循环实验过程了。
       
       7. 如果使用本地的环境进行rustlings的练习，请按照接下来的步骤进行：首先需要安装一个Linux的环境，对于windows的用户，推荐使用wsl2，也可以使用vmware等虚拟机进行安装。如果在这一步存在问题，请联系助教。
       
       8. 创建ssh key。在linux环境下，使用` ssh-keygen -t rsa -b 4096 -C "你的邮箱" `命令，创建ssh key，下面的选项全部直接敲回车即可。
          随后使用` cat ~/.ssh/id_rsa.pub` 命令查看生成的公钥，并完整的复制下来。
          在github仓库界面点击自己的头像，选择`settings`。进入到设置页面后，点击左侧的`SSH and GPG keys`选项。点击`New SSH key`选项，并将复制下来的内容粘贴上去，添加该ssh key的描述。随后点击`Add SSH key`，并一路点击确认即可。
       
       9. 在本地安装rust。进入linux环境下，参考rcore 教程 http://rcore-os.cn/rCore-Tutorial-Book-v3/ 中， 第零章操作系统概述，实验环境配置的内容，找到Rust 开发环境配置的章节，相应配置即可，你可以同时将后续需要的环境也配置好。
       
       10. clone实验仓库到本地。在前面点击链接生成的仓库中，同样点击醒目的 `code` 绿色按钮，选择`local`下的`ssh`选项，复制下面的链接。随后回到本地linux环境下，使用`git clone 复制的链接`的方式，将目标仓库clone到本地。随后，使用`ls`命令查看自己clone下来的文件夹，再使用`cd`命令进入到该文件夹下，使用`cargo install --force --path .`安装rustlings。
       
       11. 练习rustlings。使用vscode等编辑器，进入clone下来的目录下的`exercises`文件夹，依次完成对应的练习。使用`rustlings run 练习名称`去运行对应练习，也可以使用`rustlings hint 练习名称`查看题解。
       
       12. 提交。当做完部分或所有练习之后，执行 ``git add; git commit -m "update"; git push`` 命令，把更新提交到GithubClassroom的CI进行自动评测。你可以在github仓库页面的actions页面，看到你的CI提交结果，或者 http://os2edu.cn/2023-autumn-rust-ranking上面查看自己的评分。
       
       13. 上述步骤有任何问题都可以找助教。
   
   - （Option）[32 Rust Quizes](https://dtolnay.github.io/rust-quiz/1)
     
     - 要求：小练习全部通过。（**非必须完成**）
   
   - （Option）[exercisms.io 快速练习(88+道题目的中文详细描述)](http://llever.com/exercism-rust-zh/index.html)
     
     - 要求：大部分练习会做或能读懂。（**非必须完成**）
     - [exercism.io官方站点](https://exercism.io/)
