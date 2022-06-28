
## 2022年开源操作系统训练营：第一阶段

欢迎在校学生在2022年暑假参加清华大学、CSDN、毛豆教育等共同举办的**2022年开源操作系统训练营**活动（7月1日～9月10日），本次活动分为三个阶段：线上自学OS基础[基于Rust语言学习和实践操作系统内核](https://github.com/LearningOS/rust-based-os-comp2022)（7月1日～7月31日）和线上实践OS拓展[基于Rust语言的操作系统内核拓展实践](https://rcore-os.github.io/rCore-Tutorial-Book-v3/final-lab.html)（8月1日～9月10日） ,主要是对用**Rust语言进行OS研发开展学习、交流与探索**。

如有兴趣参加，请在2022年7月5日前上传个人简历到[清华云盘](https://cloud.tsinghua.edu.cn/u/d/486dc66fc8054e878b51/)，并填写[调查问卷-2022](http://oscourse2019.mikecrm.com/vzZqxgM)，获得邀请后，将开始参与本次训练营活动。完成本次活动第一阶段（7月5日～7月31日）的同学如果通过review，将可在8月1日~9月10日，开展第二阶段[基于Rust语言的操作系统内核拓展实践](https://rcore-os.github.io/rCore-Tutorial-Book-v3/final-lab.html)。训练营结束后，部分表现突出的同学将获得训练营优秀证书。鼓励同学继续以开源社区的方式参与rCore/zCore等相关的活动。

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

#### 考核成绩

- 每周学习记录情况  (25%)
- 在issues上的提问和回答问题情况，Pull Request提交情况 (25%)
- step 0 要求的编程代码的完成情况 (25%)
- step 2 rcore tutorial的通过要求完成情况 (25%)

#### step 0 自学rust编程（大约7~14天）

前提条件： 要求有基本数据结构，算法基础，相对了解或熟悉C语言等编程.

1. 自学：[阅读书籍/课程/视频等资源汇总](https://github.com/rcore-os/rCore/wiki/study-resource-of-system-programming-in-RUST)

- 推荐[Rust语言圣经(Rust教程 Rust Course)](https://course.rs/)

2. 自学编程

 - [Small exercises to get you used to reading and writing Rust code!](https://github.com/rust-lang/rustlings)
   - 要求：小练习全部通过。代码和README提交在自己在github的公开repo上。
 - [32 Rust Quizes](https://dtolnay.github.io/rust-quiz/1)
   - 要求：小练习全部通过。
 - [exercisms.io 快速练习(88+道题目的中文详细描述)](http://llever.com/exercism-rust-zh/index.html)
   - 要求：大部分练习会做或能读懂。
   - [exercism.io官方站点](https://exercism.io/)


#### step 1 自学risc-v系统结构（大约7天）

前提条件：要求有基本计算机组成原理，计算机系统结构基础。

1.阅读《计算机组成与设计（RISC-V版）》第一、二章，可以在整体结构上对 RISC-V 体系建立基本认知。 再进行后面的学习比较有效果。（半天）

###### 自学材料和练习要求: 
1. 阅读书籍和在线课程
 - 自学[PPT for RISC-V特权指令级架构](https://content.riscv.org/wp-content/uploads/2018/05/riscv-privileged-BCN.v7-2.pdf)
 - 自学[RISC-V手册：一本开源指令集的指南](http://crva.io/documents/RISC-V-Reader-Chinese-v2p1.pdf) 重点是第10章
 - 自学[RIS-V特权指令级规范](https://riscv.org/specifications/privileged-isa/) 重点是与OS相关的特权硬件访问的内容
 - [计算机组成与设计：RISC-V 教材](https://item.jd.com/12887758.html) 这是完整的课程教材，不要求全部看完，请根据自己的需求选择。
 - [计算机组成与设计：RISC-V 浙大在线课程](http://www.icourse163.org/course/ZJU-1452997167) 这是完整的一门课，不要求全部看完，请根据自己的需求选择。

2. 其他参考学习信息
 - [Berkeley CS61C: Great Ideas in Computer Architecture (Machine Structures)](http://www-inst.eecs.berkeley.edu/~cs61c/sp18/)

3. 通过要求

- 掌握RUST编程，能修改下面的rCore tutorial的代码，理解RISC-V与OS相关的硬件特性（中断，异常，系统调用，寄存器，特权级，MMU...）。

#### step 2 基于Rust语言进行操作系统内核实验--based on qemu （大约14~31天）

前提条件：要求有操作系统的基础，基本理解RISC-V与OS相关的硬件特性


- 学习理解[OS课程slides](https://learningos.github.io/os-lectures/)
- 学习了解 [rCore Tutorial v3的详细实验指导内容](https://rcore-os.github.io/rCore-Tutorial-Book-v3/)
- 学习理解[rCore Tutorial v3的实验代码](https://github.com/rcore-os/rCore-Tutorial-v3)
  
### 通过要求
根据[rust-based-os-comp2022](https://github.com/LearningOS/rust-based-os-comp2022)中的[具体实验要求](https://learningos.github.io/rCore-Tutorial-Guide-2022S/)在自己的仓库中完成5个实验，并通过测试。

## 技术指导委员会
- 陈向群 
- 吴庆波
- 张汉东
- 赵霞
- 向勇  
- 陈渝

## 助教
- 许善朴
- 尤予阳
- 杨德睿
- 陈   乐
- 吴一凡
- 张译仁


## 事务管理负责人
- 姓名：许善朴  微信号id： bitmeet520

## 训练营支持与合作单位
- 清华大学
- CSDN
- 毛豆教育
