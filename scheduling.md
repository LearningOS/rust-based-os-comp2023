
# 2022年开源操作系统训练营：第一阶段

- [新闻与纪要](./news.md)
  - **2022.11.01：2022 秋冬季训练营启动交流会，会议时间：2022/11/01 20:00-21:00 ，#腾讯会议号：838-197-763。请报名的同学参加。**
- [常见问题解答](./QA.md)
- [Learning Resource](./relatedinfo.md) (训练营学习资源)
- Online Ranking （训练营在线排行榜）
  - [第一阶段排行：Rust Lang & rCore Kernel](https://learningos.github.io/classroom-grading/)
  - [第二阶段排行：OS Kernel Implementation in OSCOMP2022](https://os-autograding.github.io/classroom-grading-template/)

欢迎在校学生/工程师在2022年秋冬季参加清华大学、CSDN、毛豆教育等共同举办的**2022年秋冬季开源操作系统训练营**活动（2022.11.01～2023.02.01），本次活动分为两个阶段：

- 第一阶段：线上自学Rust编程和OS基础，并进行[Rust语言编程自学](https://github.com/LearningOS/rust-based-os-comp2022/blob/main/scheduling.md#step-0-%E8%87%AA%E5%AD%A6rust%E7%BC%96%E7%A8%8B%E5%A4%A7%E7%BA%A6714%E5%A4%A9)、[Rust语言编程实验](https://github.com/LearningOS/rustlings)、[RISC-V处理器学习](https://github.com/LearningOS/rust-based-os-comp2022/blob/main/scheduling.md#step-1-%E8%87%AA%E5%AD%A6risc-v%E7%B3%BB%E7%BB%9F%E7%BB%93%E6%9E%84%E5%A4%A7%E7%BA%A627%E5%A4%A9)和[Rust-based OS Kernel学习&实验](https://github.com/LearningOS/rust-based-os-comp2022/blob/main/scheduling.md#step-2-%E5%9F%BA%E4%BA%8Erust%E8%AF%AD%E8%A8%80%E8%BF%9B%E8%A1%8C%E6%93%8D%E4%BD%9C%E7%B3%BB%E7%BB%9F%E5%86%85%E6%A0%B8%E5%AE%9E%E9%AA%8C--based-on-qemu-%E5%A4%A7%E7%BA%A61431%E5%A4%A9)（2022.11.01～2022.12.15）
- 第二阶段：线上自学并挑战[OS Kernel supporting Linux Apps实验](https://github.com/LearningOS/oscomp-kernel-training)（2022.12.16～2023.02.01） ,主要是**用Rust语言设计实现支持Linux APP的OS Kernel**，大约要支持50个左右的Linux Syscalls，能通过上百个Linux App测试用例。如果有其它有趣的想法或愿意参加更有挑战的训练（比如实现支持Rust协程的OS，支持unikernel&微库形态的OS等），请与助教和老师联系。

如有兴趣参加，请在2022年10月30日前请在[报名登记处](https://github.com/LearningOS/rust-based-os-comp2022/issues/101)填写相关个人信息，并加入[相关微信群](./wechat1016.png)。获得邀请后，将开始参与本次训练营活动。完成本次活动第一阶段（2022.11.01～2022.12.15）的同学如果通过review，将可在2022.12.16～2023.02.01，开展第二阶段[OS Kernel supporting Linux Apps实验](https://github.com/LearningOS/oscomp-kernel-training)。训练营结束后，部分表现突出的同学将获得训练营优秀证书。鼓励同学继续以开源社区的方式参与到企业/科研院所的操作系统实习/实践/工作/学习等相关的活动。

> 我们也在持续探索和改进开源操作系统训练营，即这个活动不仅仅局限在 2022.11.01～2023.02.01。我们希望建立的是一种长期持续发展的操作系统训练营模式，即各种学习资源都开源并整理集中在一起，导师/助教和学生/爱好者之间基于要做的实验或项目不定期/定期的进行交流。学生/爱好者完成了一定程度的学习和训练后，除了自身得到能力的提升外，还可获得相关证书和就业/学习等机会和相关推荐等，推动他在未来的进一步发展。

## 目标：

**培养合作者，搭建合作平台。**

**探索把现代系统语言Rust和灵活开放的系统结构RISC-V带入到操作系统的架构与设计的创新中来，思考未来的操作系统应该是什么样。**

## 宗旨：

**希望本活动的组织，能为操作系统爱好者提供一个活跃的开源社区环境，为对Rust、RISC-V和操作系统感兴趣的人士营造一个平等的学习与交流空间，吸引更多对操作系统感兴趣的人士参与。**

## 相关信息：

- [参加2020/2021 OS训练营学生的blog](https://rcore-os.github.io/blog/)，鼓励参加2022 OS训练营的同学把自己在学习过程中的感悟/收获等写成blog，生成pr，并提交到 <https://github.com/rcore-os/blog> 上，让更多人看到你的进步！
- **注意** 本次实习分二个阶段，第一阶段（2022.11.01～2022.12.15）以线上自学为主；第二阶段（2022.12.16～2023.02.01）以线上实践为主；
- **注意** 为及时了解和指导同学的学习和实践情况并推动学生相互帮助，本次活动要求学生把每周学习实践的过程记录（Markdown格式）放在github上自己的公开repo中。可参见[每日学习实践的具体例子](https://github.com/GCYYfun/DailySchedule)和[2020年OS训练营同学的每日学习情况汇总](https://github.com/rcore-os/rCore-Tutorial/issues/18 ) 。请参加实习的同学把记录每天的进展的git repo网址 更新到[2022年OS训练营同学的每日学习情况汇总](https://github.com/LearningOS/rust-based-os-comp2022/issues/1) 中。要求每位同学在自己的git repo中记录自己的每周进展，其他同学也可以参考学习。
- **注意** 第一阶段学习中的技术问题，建议基于github issues（比如都发到 <https://github.com/LearningOS/rust-based-os-comp2022/issues> 上，建立自己的 issue）发出并讨论。

> 如果已经掌握RUST编程，可以跳过step 0；如果已经掌握RISC-V，可以跳过step 1；如果没学过OS课，建议选择一门在线OS课程学习一下。但需要完成第一阶段实习要求的练习。
>
> 如果不是学生，但有Rust基础且对用Rust开发OS感兴趣，也欢迎申请参加。

## 第一阶段活动安排

### 总体学习要求和成绩考核方式

- 在[学习实践过程记录表](https://github.com/LearningOS/rust-based-os-comp2022/issues/1)上登记自己每日/周学习记录情况的repo网址，并在这个repo上记录每日/周学习记录情况  (成绩分数：20%)
  - [学习记录的标杆1](https://github.com/LearningOS/record)，浙江大学本科生徐文浩的2020开源操作系统训练营的过程记录，是大家学习的榜样，供大家学习参考。
  - [学习记录的标杆2](https://kiprey.github.io/tags/uCore/)：湖南大学本科生肖政杭的自学ucore for x86的过程记录，是大家学习的榜样，供大家学习参考。

- 在[第一阶段学习issues](https://github.com/LearningOS/rust-based-os-comp2022/issues/)上的提问和回答问题情况，在[第一阶段OS学习项目](https://github.com/LearningOS/rust-based-os-comp2022/) 、 [rCore Tutorial v3的详细实验指导内容](https://rcore-os.github.io/rCore-Tutorial-Book-v3/) 上的Pull Request提交情况（代码改进、文档改进、文档错误等） (成绩分数：15%)
- step 0 要求的[Rust-lang Lab Test based on Rustlings（采用Github Classroom模式的Rustling小练习）](https://classroom.github.com/a/U37u3veU) 的完成情况 (成绩分数：15%)
- step 2 [第一阶段OS学习的5个实验](https://github.com/LearningOS/rust-based-os-comp2022#kernel-labs)的完成情况和总结报告 (成绩分数：50%)
  - [第一阶段总结报告的参考](https://rcore-os.github.io/blog/2021/07/29/rcore-summary-yangpan/): 杨攀同学的2021年开源操作系统训练营第一阶段总结报告

#### step 0 自学rust编程（大约7~14天）

前提条件： 要求有基本数据结构，算法基础，相对了解或熟悉C语言等编程.

1. 自学：[阅读书籍/课程/视频等资源汇总](https://github.com/rcore-os/rCore/wiki/study-resource-of-system-programming-in-RUST)

   - 推荐：[Rust语言圣经(Rust教程 Rust Course和配套练习)](https://course.rs/)
   - 推荐：[Rust速查表（cheatsheet）](https://cheats.rs/) 该项目不仅提供了基础的语法速查，还有执行顺序详解和编写时需要关注的注意事项。项目还包含了示例代码（EX）、书籍（BK）、标准（STD）等相关资料的扩展。
   - 推荐：[清华计算机系大一学生2022暑期课程：Rust程序设计训练（有课程视频）](https://lab.cs.tsinghua.edu.cn/rust/)

2. 自学编程

   - [Rust-lang Lab Test based on Rustlings](https://classroom.github.com/a/U37u3veU)（采用Github Classroom模式的Rustling小练习，点击上述链接，形成自己的练习用repo）
     - 要求：**必须完成** 。每完成几个小练习，就执行 ``git add; git commit -m"update"; git push`` 命令，把更新提交到GithubClassroom的CI进行自动评测。要求小练习全部通过GithubClassroom的CI自动评测。
     - [学习系列视频：Rust中文社群线上学习室--通过 Rustlings 学 Rust](https://space.bilibili.com/24917186/video)

       **提示：基于github classroom的开发方式**

       基于github classroom，可方便建立开发用的git repository，并可基于github的 codespace（在线版ubuntu +vscode）在线开发使用。整个开发环境仅仅需要一个网络浏览器。

       > codespace 不是必须的。如果是本地的ubuntu中建立开发环境，可在shell中执行 `make ubuntu_local_setenv` 来自动安装配置开发环境（执行需要 `sudo` root 权限，仅需要执行一次）。

       1. 在网络浏览器中用自己的 github id 登录 github.com。
       2. 接收 [Rust-lang Lab Test based on Rustlings 的github classroom在线邀请](https://classroom.github.com/a/U37u3veU)  ，根据提示一路选择OK即可。
       3. 完成第二步后，你的rustings实验练习 的 github repository 会被自动建立好，点击此github repository的链接，就可看到你要完成的实验了。
       4. 在你的第一个实验练习的网页的中上部可以看到一个醒目的 `code`  绿色按钮，点击后，可以进一步看到 `codespace` 标签和醒目的 `create codesapce on edu` 绿色按钮。请点击这个绿色按钮，就可以进入到在线的ubuntu +vscode环境中
       5. 再按照下面的环境安装提示在vscode的 `console` 中安装配置开发环境：rustc等工具。
       6. 然后就可以基于在线vscode进行测试 (执行命令 `rustlings watch` ），编辑代码的循环实验过程了。

   - （Option）[32 Rust Quizes](https://dtolnay.github.io/rust-quiz/1)
     - 要求：小练习全部通过。（**非必须完成**）
   - （Option）[exercisms.io 快速练习(88+道题目的中文详细描述)](http://llever.com/exercism-rust-zh/index.html)
     - 要求：大部分练习会做或能读懂。（**非必须完成**）
     - [exercism.io官方站点](https://exercism.io/)

#### step 1 自学risc-v系统结构（大约2~7天）

前提条件：要求有基本计算机组成原理，计算机系统结构基础。

阅读《计算机组成与设计（RISC-V版）》第一、二章，可以在整体结构上对 RISC-V 体系建立基本认知。再进行后面的学习比较有效果。

#### 自学材料和练习要求:

1. 阅读书籍和在线课程

   - 自学[PPT for RISC-V特权指令级架构](https://content.riscv.org/wp-content/uploads/2018/05/riscv-privileged-BCN.v7-2.pdf)
   - 自学[RISC-V手册：一本开源指令集的指南](http://riscvbook.com/chinese/RISC-V-Reader-Chinese-v2p1.pdf) 重点是第10章
   - （Option）自学[RIS-V特权指令级规范](https://riscv.org/technical/specifications/) 重点是与OS相关的特权硬件访问的规范内容（Privileged Spec）
   - （Option）自学[RISC-V汇编手册](https://github.com/riscv-non-isa/riscv-asm-manual/blob/master/riscv-asm.md)
   - （Option）[计算机组成与设计：RISC-V 教材](https://item.jd.com/12887758.html) 这是完整的课程教材，不要求全部看完，请根据自己的需求选择。
   - （Option）[计算机组成与设计：RISC-V 浙大在线课程](http://www.icourse163.org/course/ZJU-1452997167) 这是完整的一门课，不要求全部看完，请根据自己的需求选择。

2. 其他参考学习信息

   - （Option）[Berkeley CS61C: Great Ideas in Computer Architecture (Machine Structures)](http://www-inst.eecs.berkeley.edu/~cs61c/sp18/)

   > Option的含义是：如果有足够的时间建议看看，否则在后续要用到时或需要查询进一步信息时再查阅这些内容。

3. 通过要求

   - 掌握RUST编程，能修改下面的rCore tutorial的代码，理解RISC-V与OS相关的硬件特性（中断，异常，系统调用，寄存器，特权级，MMU...）。

#### step 2 基于Rust语言进行操作系统内核实验--based on qemu （大约14~31天）

前提条件：要求有操作系统的基础，基本理解RISC-V与OS相关的硬件特性

##### 学习理解

- [OS课程slides](https://learningos.github.io/os-lectures/)
- [rCore Tutorial v3的详细实验指导内容](https://rcore-os.github.io/rCore-Tutorial-Book-v3/)
- [rCore Tutorial v3的实验代码](https://github.com/rcore-os/rCore-Tutorial-v3)
- [视频：2022年春季OS课程讲解和OS实验讲解](./relatedinfo.md#课程视频)
- [2022年春季OS课程实验中 ``cargo doc`` 生成的各章参考OS的API文档](./relatedinfo.md)

##### 具体步骤

根据[rust-based-os-comp2022](https://github.com/LearningOS/rust-based-os-comp2022)中的各个实验的具体实验要求在自己的仓库中完成5个实验，通过基于GitHub Classroom的CI测试，并在最后写出总结报告。

访问 [训练营 kernel labs](https://github.com/LearningOS/rust-based-os-comp2022#kernel-labs)，点击下列链接，形成各个自己实验专用 repos：

- [lab0-0 实践](https://classroom.github.com/a/hnoWuKGF)
- [lab0-1 实践](https://classroom.github.com/a/UEOvz4qO)
- [lab1 实验](https://classroom.github.com/a/s1v7GyJM)
- [lab2 实验](https://classroom.github.com/a/ghbB1wYX)
- [lab3 实验](https://classroom.github.com/a/RxB6h4-x)
- [lab4 实验](https://classroom.github.com/a/94eMW8zi)
- [lab5 实验](https://classroom.github.com/a/zqGJEPK-)

请注意各个实践或实验的具体初始化设置：

- [lab0-0 实践初始化设置](https://learningos.github.io/rust-based-os-comp2022/chapter1/0intro.html#id4)
- [lab0-1 实践初始化设置](https://learningos.github.io/rust-based-os-comp2022/chapter2/0intro.html#id3)
- [lab1 实验初始化设置](https://learningos.github.io/rust-based-os-comp2022/chapter3/0intro.html#id3)
- [lab2 实验初始化设置](https://learningos.github.io/rust-based-os-comp2022/chapter4/0intro.html#id3)
- [lab3 实验初始化设置](https://learningos.github.io/rust-based-os-comp2022/chapter5/0intro.html#id3)
- [lab4 实验初始化设置](https://learningos.github.io/rust-based-os-comp2022/chapter6/0intro.html#id3)
- [lab5 实验初始化设置](https://learningos.github.io/rust-based-os-comp2022/chapter8/0intro.html#id5)

然后就可以开始具体的实践和实验了。

在完成每个实验中的OS代码后，通过执行 `git push` 命令来更新自己的实验专用 repos，并触发GitHub Classroom的CI测试。等一小会后，会看到自己的实验专用 repos 上面会有一个绿色的小勾，表示最新的提交通过了CI测试。如果看到的是红色的小叉，表面没有通过CI测试，你可以点击小叉，并进一步点击 GitHub Classroom Workflow/Autograding 的 details，进入自动测试的详细log记录页面，查看测试中具体在哪一步出现了问题，并尝试自己修复bug/更新功能，争取下一次通过测试。

One More Thing：当你看到这，感觉第一阶段还没开始，还在想下一步要干啥时，我们的建议是：**Just Do It NOW!**
