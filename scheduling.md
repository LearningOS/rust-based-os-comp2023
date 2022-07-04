
## 2022年开源操作系统训练营：第一阶段

欢迎在校学生在2022年暑假参加清华大学、CSDN、毛豆教育等共同举办的**2022年开源操作系统训练营**活动（7月1日～9月10日），本次活动分为两个阶段：线上自学OS基础[基于Rust语言学习和实践操作系统内核](https://github.com/LearningOS/rust-based-os-comp2022)（7月1日～7月31日）和线上实践OS拓展[基于Rust语言的操作系统内核拓展实践](https://github.com/LearningOS/rust-based-os-comp2022/blob/main/stage2-sched.md)（8月1日～9月10日） ,主要是对用**Rust语言进行OS研发开展学习、交流与探索**。

如有兴趣参加，请在2022年7月5日前上传个人简历到清华云盘“简历收件箱[CV-inbox](https://cloud.tsinghua.edu.cn/u/d/3c62302864fd4ff090a1/)”，并填写[2022年开源操作系统训练营报名表](http://oscourse2019.mikecrm.com/yKVchGm)，获得邀请后，将开始参与本次训练营活动。完成本次活动第一阶段（7月1日～7月31日）的同学如果通过review，将可在8月1日~9月10日，开展第二阶段[基于Rust语言的操作系统内核拓展实践](https://rcore-os.github.io/rCore-Tutorial-Book-v3/final-lab.html)。训练营结束后，部分表现突出的同学将获得训练营优秀证书。鼓励同学继续以开源社区的方式参与rCore/zCore等相关的活动。

**注：我们也在改进开源操作系统训练营，即这个活动不仅仅局限在2022年7月1日~9月10日。我们希望建立的是一种长期持续发展的操作系统训练营模式，即各种学习资源都开源并整理集中在一起，导师/助教和学生/爱好者之间基于要做的实验或项目不定期/定期的进行交流。学生/爱好者完成了一定程度的学习和训练后，除了自身得到能力的提升外，还可获得相关证书和就业/学习等机会和相关推荐等，推动他在未来的进一步发展。**

### 目标：

 **培养合作者，搭建合作平台。**

 **探索把现代系统语言Rust和灵活开放的系统结构RISC-V带入到操作系统的架构与设计的创新中来，思考未来的操作系统应该是什么样。**

### 宗旨：
 **希望本活动的组织，能为操作系统爱好者提供一个活跃的开源社区环境，为对Rust、RISC-V和操作系统感兴趣的人士营造一个平等的学习与交流空间，吸引更多对操作系统感兴趣的人士参与。**

### 相关信息：
- [参加2020/2021 OS训练营学生的blog](https://rcore-os.github.io/blog/)，鼓励参加2022 OS训练营的同学把自己在学习过程中的感悟/收获等写成blog，生成pr，并提交到 https://github.com/rcore-os/blog 上，让更多人看到你的进步！

- **注意：** 提交申请+个人简历的截止时间是7月5日。本次实习分二个阶段，第一阶段（7月1日～7月31日）以线上自学为主；第二阶段（8月1日～9月10日）以线上实践为主；

- **注意：** 为及时了解和指导同学的学习和实践情况并推动学生相互帮助，本次活动要求学生把每周学习实践的过程记录(Markdown格式)放在github上自己的公开repo中。可参见[每日学习实践的具体例子](https://github.com/GCYYfun/DailySchedule)和[2020年OS训练营同学的每日学习情况汇总](https://github.com/rcore-os/rCore-Tutorial/issues/18 ) 。请参加实习的同学把记录每天的进展的git repo网址 更新到[2022年OS训练营同学的每日学习情况汇总](https://github.com/LearningOS/rust-based-os-comp2022/issues/1) 中。要求每位同学在自己的git repo中记录自己的每周进展，其他同学也可以参考学习。

- **注意：** 第一阶段学习中的技术问题，建议基于github issues （比如都发到 https://github.com/LearningOS/rust-based-os-comp2022/issues 上，建立自己的issue ）发出并讨论。

- 如果已经掌握RUST编程，可以跳过step 0；如果已经掌握RISC-V，可以跳过step 1；如果没学过OS课，建议选择一门在线OS课程学习一下。但需要完成第一阶段实习要求的练习。

- 如果不是学生，但有Rust基础且对用Rust开发OS感兴趣，也欢迎申请参加，只是无法去启元实验室本地实习，主要通过远程方式（github/电邮/微信）进行交流与合作。

### 第一阶段活动安排

#### 总体学习要求和成绩考核方式

- 在[学习实践过程记录表](https://github.com/LearningOS/rust-based-os-comp2022/issues/1)上登记自己每日/周学习记录情况的repo网址，并在这个repo上记录每日/周学习记录情况  (成绩分数：20%)
   - [学习记录的标杆](https://kiprey.github.io/tags/uCore/)，这是一位本科生的自学ucore for x86的过程记录，是大家学习的榜样，供大家学习参考。

- 在[第一阶段学习issues](https://github.com/LearningOS/rust-based-os-comp2022/issues/)上的提问和回答问题情况，在[第一阶段OS学习项目](https://github.com/LearningOS/rust-based-os-comp2022/) 、 [rCore Tutorial v3的详细实验指导内容](https://rcore-os.github.io/rCore-Tutorial-Book-v3/) 上的Pull Request提交情况（代码改进、文档改进、文档错误等） (成绩分数：15%)
- step 0 要求的[Rust-lang Lab Test based on Rustlings（采用Github Classroom模式的Rustling小练习）](https://classroom.github.com/a/YTNg1dEH) 的完成情况 (成绩分数：15%)
- step 2 [第一阶段OS学习的5个实验](https://github.com/LearningOS/rust-based-os-comp2022#kernel-labs)的完成情况 (成绩分数：50%)

#### step 0 自学rust编程（大约7~14天）

前提条件： 要求有基本数据结构，算法基础，相对了解或熟悉C语言等编程.

1. 自学：[阅读书籍/课程/视频等资源汇总](https://github.com/rcore-os/rCore/wiki/study-resource-of-system-programming-in-RUST)

   - 推荐：[Rust语言圣经(Rust教程 Rust Course和配套练习)](https://course.rs/)

2. 自学编程

   - [Rust-lang Lab Test based on Rustlings](https://classroom.github.com/a/YTNg1dEH)（采用Github Classroom模式的Rustling小练习，点击上述链接，形成自己的练习用repo） 
     - 要求：**必须完成** 。每完成几个小练习，就执行 ``git add; git commit -m"update"; git push`` 命令，把更新提交到GithubClassroom的CI进行自动评测。要求小练习全部通过GithubClassroom的CI自动评测。
     - [学习系列视频：Rust中文社群线上学习室--通过 Rustlings 学 Rust](https://space.bilibili.com/24917186/video) 
        
          **提示：基于github classroom的开发方式**
            
          基于github classroom，可方便建立开发用的git repository，并可基于github的 codespace （在线版ubuntu +vscode）在线开发使用。整个开发环境仅仅需要一个网络浏览器。

          1.） 在网络浏览器中用自己的 github  id 登录 github.com

          2.） 接收 [Rust-lang Lab Test based on Rustlings 的github classroom在线邀请](https://classroom.github.com/a/YTNg1dEH)  ，根据提示一路选择OK即可。
          
          3.） 完成第二步后，你的rustings实验练习 的 github repository 会被自动建立好，点击此github repository的链接，就可看到你要完成的实验了。
          
          4.） 在你的第一个实验练习的网页的中上部可以看到一个醒目的 `code`  绿色按钮，点击后，可以进一步看到  `codespace` 标签和醒目的 `create codesapce on main` 绿色按钮。请点击这个绿色按钮，就可以进入到在线的ubuntu +vscode环境中
          
          5.） 再按照下面的环境安装提示在vscode的 `console` 中安装配置开发环境：rustc等工具。注：也可在vscode的 `console` 中执行 ``make codespaces_setenv`` 来自动安装配置开发环境（执行``sudo``需要root权限，仅需要执行一次）。
          
          6.） 在vscode的 `console` 中执行 `make setupclassroom`  （该命令仅执行一次）配置githubclassroom 自动评分功能。
          
          7.） 然后就可以基于在线vscode进行测试 (执行命令 `rustlings watch` ），编辑代码的循环实验过程了。

          上述的3，4，5步不是必须的，你也可以线下本地开发。

          注：如果是本地的ubuntu中建立开发环境，可在shell中执行 ``make ubuntu_local_setenv`` 来自动安装配置开发环境（执行``sudo``需要root权限，仅需要执行一次）。

   - [32 Rust Quizes](https://dtolnay.github.io/rust-quiz/1)
     - 要求：小练习全部通过。
   - [exercisms.io 快速练习(88+道题目的中文详细描述)](http://llever.com/exercism-rust-zh/index.html)
     - 要求：大部分练习会做或能读懂。
     - [exercism.io官方站点](https://exercism.io/)


#### step 1 自学risc-v系统结构（大约2~7天）

前提条件：要求有基本计算机组成原理，计算机系统结构基础。

1.阅读《计算机组成与设计（RISC-V版）》第一、二章，可以在整体结构上对 RISC-V 体系建立基本认知。 再进行后面的学习比较有效果。（半天）

###### 自学材料和练习要求: 
1. 阅读书籍和在线课程
 - 自学[PPT for RISC-V特权指令级架构](https://content.riscv.org/wp-content/uploads/2018/05/riscv-privileged-BCN.v7-2.pdf)
 - 自学[RISC-V手册：一本开源指令集的指南](http://riscvbook.com/chinese/RISC-V-Reader-Chinese-v2p1.pdf) 重点是第10章
 - （Option）自学[RIS-V特权指令级规范](https://riscv.org/technical/specifications/) 重点是与OS相关的特权硬件访问的规范内容（Privileged Spec）
 - （Option）自学[RISC-V汇编手册](https://github.com/riscv-non-isa/riscv-asm-manual/blob/master/riscv-asm.md)
 - （Option）[计算机组成与设计：RISC-V 教材](https://item.jd.com/12887758.html) 这是完整的课程教材，不要求全部看完，请根据自己的需求选择。
 - （Option）[计算机组成与设计：RISC-V 浙大在线课程](http://www.icourse163.org/course/ZJU-1452997167) 这是完整的一门课，不要求全部看完，请根据自己的需求选择。

2. 其他参考学习信息
 - （Option）[Berkeley CS61C: Great Ideas in Computer Architecture (Machine Structures)](http://www-inst.eecs.berkeley.edu/~cs61c/sp18/)


注：Option的含义是：如果有足够的时间建议看看，否则在后续要用到时或需要查询进一步信息时再查阅这些内容。

3. 通过要求

- 掌握RUST编程，能修改下面的rCore tutorial的代码，理解RISC-V与OS相关的硬件特性（中断，异常，系统调用，寄存器，特权级，MMU...）。

#### step 2 基于Rust语言进行操作系统内核实验--based on qemu （大约14~31天）

前提条件：要求有操作系统的基础，基本理解RISC-V与OS相关的硬件特性

##### 学习理解
- [OS课程slides](https://learningos.github.io/os-lectures/)
- [rCore Tutorial v3的详细实验指导内容](https://rcore-os.github.io/rCore-Tutorial-Book-v3/)
- [rCore Tutorial v3的实验代码](https://github.com/rcore-os/rCore-Tutorial-v3)
- [2022年春季OS课程讲解和OS实验讲解](./relatedinfo.md)
- [2022年春季OS课程实验中 ``cargo doc`` 生成的各章参考OS的API文档](./relatedinfo.md)

##### 具备步骤

根据[rust-based-os-comp2022](https://github.com/LearningOS/rust-based-os-comp2022)中的各个实验的具体实验要求在自己的仓库中完成5个实验，并通过基于GitHub Classroom的CI测试。

具体步骤
- 登录github.com
- 访问 [训练营 kernel labs](https://github.com/LearningOS/rust-based-os-comp2022#kernel-labs) ，点击如下链接，形成各个自己实验专用 repos：

    - [lab0-0实践](https://classroom.github.com/a/hnoWuKGF) 
    - [lab0-1实践](https://classroom.github.com/a/UEOvz4qO)
    - [lab1实验](https://classroom.github.com/a/s1v7GyJM) 
    - [lab2 实验](https://classroom.github.com/a/ghbB1wYX)
    - [lab3 实验](https://classroom.github.com/a/RxB6h4-x) 
    - [lab4 实验](https://classroom.github.com/a/94eMW8zi)
    - [lab5 实验](https://classroom.github.com/a/zqGJEPK-) 

请注意各个实践或实验的具体初始化设置：
 - [lab0-0实践初始化设置](https://learningos.github.io/rust-based-os-comp2022/chapter1/0intro.html#id4)
 - [lab0-1实践初始化设置](https://learningos.github.io/rust-based-os-comp2022/chapter2/0intro.html#id3)
 - [lab1实验初始化设置](https://learningos.github.io/rust-based-os-comp2022/chapter3/0intro.html#id3)
 - [lab2 实验初始化设置](https://learningos.github.io/rust-based-os-comp2022/chapter4/0intro.html#id3)
 - [lab3 实验初始化设置](https://learningos.github.io/rust-based-os-comp2022/chapter5/0intro.html#id3)
 - [lab4 实验初始化设置](https://learningos.github.io/rust-based-os-comp2022/chapter6/0intro.html#id3)
 - [lab5 实验初始化设置](https://learningos.github.io/rust-based-os-comp2022/chapter8/0intro.html#id5)

然后就可以根据 [OS训练营实验指导](https://learningos.github.io/rust-based-os-comp2022/) 开始具体的实践和实验了。


在完成每个实验中的OS代码后，通过执行 `git push` 命令来更新自己的实验专用 repos，并触发GitHub Classroom的CI测试。等一小会后，会看到自己的实验专用 repos 上面会有一个绿色的小勾，表示最新的提交通过了CI测试。如果看到的是红色的小叉，表面没有通过CI测试，你可以点击小叉，并进一步点击 GitHub Classroom Workflow/Autograding 的 details，进入自动测试的详细log记录页面，查看测试中具体在哪一步出现了问题，并尝试自己修复bug/更新功能，争取下一次通过测试。

## 技术指导委员会
- 陈向群 
- 吴庆波
- 张汉东
- 赵霞
- 向勇  
- 陈渝
- 李国良
- 任炬

## 助教
- 许善朴
- 尤予阳
- 杨德睿
- 陈   乐
- 吴一凡
- 张译仁
- 卢   军
- 黄   旺

## 事务管理负责人
- 姓名：许善朴  微信号id： bitmeet520

## 训练营支持与合作单位
- [rcore-os 开源社区](https://github.com/rcore-os)
- [木兰开源社区](https://portal.mulanos.cn)
- CCF开源发展委员会
- CCF系统软件专业技术委员会
- 清华大学
- 北京工商大学
- 国防科技大学
- CSDN
- 毛豆教育
- 鹏城实验室
- 启元实验室
- 阿里云
- 华为

