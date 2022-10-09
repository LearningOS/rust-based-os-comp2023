## 2022年开源操作系统训练营：第二阶段

## 相关信息
- **注意：** 为及时了解和指导同学的学习和实践情况并推动学生相互帮助，本次活动要求学生把每日学习实践的过程记录在自己在github的公开repo上。可参考：
- [徐文浩的2020开源操作系统训练营的过程记录](https://github.com/LearningOS/record)
- [2021开源操作系统训练营第二阶段总结列表](https://rcore-os.github.io/blog/categories/report/)
- [2020开源操作系统训练营第二阶段总结：K210组RustSBI开发总结 -- 洛佳](https://rcore-os.github.io/blog/2020/09/06/os-report-final-luojia65/)
- [2020开源操作系统训练营第二阶段总结：zCore Summer of Code 2020 报告 -- 车春池](https://rcore-os.github.io/blog/2020/09/02/zcore_report_chechunchi/)
- [2020开源操作系统训练营第二阶段总结：移植rCore-Tutorial 到k210 -- 徐文浩](https://rcore-os.github.io/blog/2020/08/30/report-k210/)
- [2020开源操作系统训练营第二阶段总结：rCore 到 zCore 功能迁移组报告 -- 郑昱笙、李宇](https://rcore-os.github.io/blog/2020/08/30/report-of-rCore-to-zCore/)
- [陈兴的每日学习实践的过程记录](https://github.com/GCYYfun/DailySchedule)


## 日程：第一周：OS相关技术讲解与学习（7天）
- 题目：CoreDebugger：支持Rust语言的源代码级内核调试工具，报告人：陈志扬，时间 2022.07.31 9:00(周日上午)
- 题目：zCore结构和核心组件，报告人：张译仁，时间：2022.08.01 20:30(周一晚上)
- 题目：操作系统多核启动，报告人：杨德睿，时间：2022.08.02 20:30(周二晚上)
- 题目：实现zCore syscall，报告人：董峰，时间：2022.08.03 20:30(周三晚上)
- 题目：zCore网络框架，报告人：萧络元，时间：2022.08.03 20:30(周四晚上)
- 题目：zCore单元测试框架，报告人：石振兴，时间：2022.08.05 20:30(周五晚上)

## 日程：第二~第四周：操作系统研究与探索项目 (34天)
目前设置了多个项目，学生可选择参加下面的项目，指导老师制定以周为单位的项目研究计划。

**注意：参加操作系统研究与探索项目的前提先完成第一阶段的实验要求（写了总结报告，完成了lab0~lab5的实验），为此，晚了一些时间是可以的** 

#### 模块化的 rCore-Tutorial-v3

1. 项目标题：模块化的 rCore-Tutorial-v3
2. [模块化的 rCore-Tutorial-v3: GitHub Classroom](https://classroom.github.com/a/Oc792plB)
3. 项目描述：rCore-Tutorial-v3 是一套简洁，易于上手的教程，但是目前代码将不同章节用分支隔离的组织形式导致前一章实验的成果很难迁移到后一章，且若要修改某一章的实现，就需要手动同步到后续所有章节。我们希望能发挥 Rust 语言 workspace/crates/traits 的先进设计理念，重构并形成模块化rCore-Tutorial，将学习时对仓库的操作变为以下形式：

   | 操作          | 使用 git 分支        | 使用 crate
   | ------------ | ------------------- | ---------------
   | 学习另一个章节 | 切换分支             | 修改 workspace（即切换相关 crates）
   | 做课后实验     | 切换到 lab 分支写代码 | 封装一个 crate 加入 workspace

   这样操作系统将以模块化/Traits的形式呈现给同学，同学按照模块化/Traits实现的方法来完成实验。实验的内容也可随之灵活调整。

4. 项目难度：中（适合完成了lab1-5，熟悉 Rust workspace/crates/traits 的同学）
5. 项目社区导师：杨德睿 github id:YdrMaster weichat id: ydrdwx ； 许善朴 github id: xushanpu123   weichat id: bitmeet520
6. 项目产出要求：

   项目应该将现有的 rCore-Tutorial-v3 变得模块化。形式包括：
   - 章节模块化：所有章节不再被 git 分支隔离开，而是只有逻辑上的隔离关系，后一章节能够以依赖库的形式继承前一章节的成果
   - 实现模块化：能在所有章节中复用的代码形成单独的 crate 甚至 package，crates之间在调用方面有层次依赖关系, crates的粒度尽量小。
   - 系统调用接口模块化：系统调用的分发封装到一个 crate。使得添加系统调用的模式不是为某个 match 增加分支，而是实现一个分发库要求的 trait 并将实例传递给分发库

7. 相关的开源软件仓库列表：
   - https://github.com/theseus-os/Theseus （OS的目标不同，但在OS的设计上有部分内容与此相近）

      Theseus is a modern OS written from scratch in Rust that explores 𝐢𝐧𝐭𝐫𝐚𝐥𝐢𝐧𝐠𝐮𝐚𝐥 𝐝𝐞𝐬𝐢𝐠𝐧, novel OS structure, and state management. It strives to close the semantic gap between compiler and hardware to maximally leverage the power of language safety, and thus shift OS responsibilities like resource management into the compiler.

### rCore-Tutorial-v3 进一步进阶/扩展
1. 项目标题：rCore-Tutorial-v3 进一步扩展
2. [rCore-Tutorial-v3 进一步扩展 rCore-Tutorial-v3: GitHub Classroom](https://classroom.github.com/a/bAc-5319)
3. 项目描述：rCore-Tutorial-v3 的一个重要目标是以简洁的设计实现对应到操作系统的核心知识点上，所以还有很多可以扩展的地方。我们希望通过设计一个一个相对独立的实验，来展现操作系统的核心设计思想，在操作系统实现与操作系统原理之间建立桥梁。学生自己设计实现新OS功能后，其系统能力也随之提高。简言之：做多，错多，收获多
4. 项目难度：低/中 （适合完成了lab1-5，且希望能够一小步一小步地继续提升自己OS编程能力/系统能力的同学）
5. 项目社区导师：陈渝 github id: chyyuu     weichat id: chyyuu 
6. 项目社区导师：吴一凡 github id: wyfcyx     weichat id: yifanwu1998
7. 项目社区导师：许善朴 github id: xushanpu123   weichat id: bitmeet520
8. 项目社区导师：杨德睿 github id:YdrMaster  weichat id: ydrdwx 
9. 项目社区导师：陈乐 github id: yuoo655    weichat id: Endagorion_
10. 项目产出要求：
   - 补充完善 rCore-Tutorial-v3

11. 项目技术要求：
   - 具备一定的 Rust 语言基础，能看懂并模仿现有代码即可
   - 熟悉操作系统原理与简易实现
   - 具有基础英文阅读和写作能力

11. 相关的开源软件仓库列表：
  - https://github.com/rcore-os/rCore-Tutorial-v3
  - https://rcore-os.github.io/rCore-Tutorial-Book-v3/final-lab.html

10. 一个月可行的进阶步骤（当然也是一种挑战） 
      - 选择1：内核支持中断响应 --> 内核支持 读取设备树，支持virtio外设，支持串口外设 （rcore tutorial v3的ch9已经完成） --> 内核支持多核结构  --> 内核支持Linux syscall（即支持Linux应用）
      - 选择2：内核支持中断响应 --> 内核支持 读取设备树，支持virtio外设，支持串口外设 --> 图形界面支持 （rcore tutorial v3的ch9已经完成） --> 改进图形界面支持
      - 选择3：内核支持中断响应 --> 内核支持 读取设备树，支持virtio外设，支持串口外设 （rcore tutorial v3的ch9已经完成） --> 实现对TUI界面的支持（即可以在字符界面实现窗口界面和各种基于TUI界面的游戏）

选择2/3的OS参考

- [带TUI界面的Lateral OS](https://github.com/carterisonline/lateral)
- [有snake/2048图形界面游戏的SnakeOS](https://github.com/trusch/snakeos)

### 操作系统课程的有趣大实验
1. 项目标题：操作系统课程的有趣大实验
2. 项目描述：操作系统课程的大实验的重要目标基于学生兴趣来开展各种操作系统技术的探索，拓展学生的视野，培养未来操作系统人才。
3. 项目难度：中/高 （适合完成了lab1-5，并希望能够较快进入更实际一些的OS能力训练或做OS方向研究的同学）
4. 项目社区导师：向勇 github id: xyongcn     weichat id: xyongcn 
5. 项目产出要求：
   - 各种大实验的设计与实现

6. 项目技术要求：
   - 具备一定的 Rust 语言基础，能看懂并模仿现有代码即可
   - 熟悉操作系统原理与简易实现
   - 具有基础英文阅读和写作能力

7. 2022春季OS课相关大实验的开源软件仓库列表(鼓励进一步改进和扩展)：
  - [类似ebpf的rcore trace](https://github.com/latte-c/rvjit)
  - [硬件级用户态中断模拟与系统软件支持](https://github.com/OS-F-4/usr-intr)
  - [支持异步协程的zCore](https://github.com/orgs/OSLab-zCore/)
  - [用rust重新实现FreeRTOS](https://github.com/LDYang694/RFREERTOS)
  - [用rust重新实现Linux的KVM](https://github.com/KaitoD/linux) 
  - [基于x86_64的rCore-Tutorial-v3](https://github.com/rcore-os/rCore-Tutorial-v3-x86_64)
  - [基于AARCH64的rCore-Tutorial-v3](https://github.com/rcore-os/rCore-Tutorial-v3-arm64)
  - rCore-Tutorial-v3/zCore直接支持Rust std标准库：已完成的本科毕设，有初步结果
### 异步操作系统
1. 项目标题：异步操作系统设计和完善
2. 项目描述：在RISC-V平台上设计并实现一个基于Rust语言的异步操作系统。最终目标是，利用Rust语言和开源工具链的特征，在操作系统内核中实现细粒度的并发安全、模块化和可定制特征；利用Rust语言的异步机制，优化操作系统内核的并发性能；向应用程序提供的异步系统调用接口，优化操作系统的系统调用访问性能；结合LLVM中Rust语言编译器的异步支持技术，完善操作系统的进程、线程和协程概念，统一进程、线程和协程的调度机制；利用RISC-V平台的用户态中断技术，优化操作系统的信号和进程通信性能；开发原型系统，设计用户态测试用例库和操作系统动态分析跟踪工具，对异步操作系统的特征进行定量性的评估。
3. 项目难度：中/高 （适合完成了lab1-5，并希望能够较快进入更实际一些的OS能力训练或做OS方向研究的同学）
4. 项目社区导师：向勇 github id: xyongcn     weichat id: xyongcn 
5. 项目产出要求：
   - 参与正在进行的异步操作系统开发项目，提供力所能及改进和完善；
   - 针对异步操作系统中还没有开工的模块，提供初始的参考实现；
6. 项目技术要求：
   - 具备一定的 Rust 语言基础，能看懂并模仿现有代码即可
   - 熟悉操作系统原理与简易实现
   - 具有基础英文阅读和写作能力

7. 已有相关工作：
  - [异步操作系统设计](https://github.com/async-kernel/documents/blob/main/design/design.md)
  - 用户态中断扩展
      - 尤予阳，[软硬协同的用户态中断](https://gallium70.github.io/rv-n-ext-impl/intro.html)：RISC-V的用户态中断规范建议；
      - [软硬协同的用户态中断机制研究](https://github.com/Gallium70/final-project/blob/master/论文.pdf)：本科毕设论文；
      - 王之栋、项晨东、孙迅，[基于Intel x86用户态中断的高效进程间通信](https://github.com/OS-F-4/usr-intr/blob/main/ppt/2022-6-11.pptx)：Intel用户态中断的QEMU支持、基于用户态中断的异步系统调用实现
  - 内核模块的异步通信框架：
      - 吴一凡，[Async-modules extension of zCore](https://github.com/wyfcyx/zCore/tree/async-modules)：异步模块间的通信框架实现；
  - 用户态的异步支持库：
      - 车春池，[基于Rust的io-uring实现](https://github.com/SKTT1Ryze/Graduation2022/releases/download/v0.1.0/thesis.pdf)：Rust和C++的异步支持库，本科毕设论文；
  - 异步操作系统调度
      - 车春池、蒋周奇：共享调度器 https://github.com/HUST-OS/tornado-os#如何运行
      - 刘松铭、于子淳：[zCore多核异步调度器](https://github.com/OSLab-zCore/OSLab-Docs)
      - 王文智：[线程与协程的统一调度](https://github.com/AmoyCherry/UnifieldScheduler/blob/02f3d17cafd6b3ca4091df6a47798f67a44e7788/%E5%BC%82%E6%AD%A5OS%E8%AE%BE%E8%AE%A1%E6%96%87%E6%A1%A3.md)
### zCore 的文档与单元测试完善

1. 项目标题：zCore 的文档与单元测试完善
2. [zCore 的文档与单元测试完善](https://github.com/LearningOS/zCore)（注：由于有Github LFS，不能设置为classroom）
3. 项目描述：zCore 是用 Rust 实现的支持协程/异步等新机制的操作系统，目前zCore的实现已经初步支持 x86_64/RISC-V 64/AArch64。然而，随着后期开发进程的加快，大量代码缺少文档描述，并且没有实现单元测试，只能在 QEMU 中运行用户程序以检验代码正确性。本项目的目标是完善 zCore 的文档及单元测试，使其成为一个高质量的 Rust 社区项目。
4. 项目难度：低 （适合完成了lab1-5，并希望在OS领域找到一份测试/研发工作，或提升软件工程能力的同学）
5. 项目社区导师：石振兴  github id: shzhxh    weichat id: szx-bj
6. 项目社区导师：董峰  github id: workerwork  weichat id: Mr_dong-feng
7. 项目产出要求：
   - 补充完善 zCore 各模块的代码文档。

     目标是通过 `#![deny(missing_docs)]` 编译，并且能够让开发者通过阅读文档，快速理解 zCore 的代码结构和各部分功能。
   - 参考 Fuchsia 官方文档及测试代码，为 zCore 中的内核对象补充单元测试。

     目标让 `zircon-object` 模块的测试覆盖率提高到 90% 以上。
   - （可选）在 CI 中支持运行集成测试。
     目标是最大化整体的测试覆盖率。
   - （可选）在zCore中添加zircon/linux的syscall。
     目标：完善添加zCore内核功能，让zCore通过更多的zircon tests(基于zircon的coretest)或Linux tests（基于musl libc的libc test）或相关应用

8. 项目技术要求：
   - 具备一定的 Rust 语言基础，能看懂并模仿现有代码即可
   - 熟悉操作系统原理与简易实现
   - 具有基础英文阅读和写作能力
9. 相关的开源软件仓库列表：
   - https://github.com/rcore-os/zCore （zCore 仓库）
   - https://rcore-os.github.io/zCore/zircon_object （zCore 代码文档）
   - https://fuchsia.dev/fuchsia-src/reference （Zircon 官方文档）


### zCore 的星光/SiFive/树莓派等开发板和外设支持

1. 项目标题：zCore 的星光/SiFive开发板支持
2. [zCore 的星光/SiFive开发板支持:项目模板](https://github.com/LearningOS/zCore)（注：由于有Github LFS，不能设置为classroom）
3. 项目描述：zCore 是用 Rust 实现的支持协程/异步等新机制的操作系统，目前zCore的实现已经初步支持 x86_64/RISC-V 64/AArch64。zCore需要继续完善相关驱动和系统调用。
4. 项目难度：中（适合完成了lab1-5，并希望在系统类企业找到一份OS底层研发工作，或提升软件工程能力的同学）
5. 项目社区导师：肖络元 github id: shzhxh    weichat id: xiaoxiaoluckyard
6. 项目社区导师：杨德睿 github id:YdrMaster  weichat id: ydrdwx
7. 项目产出要求：
   - 能够在某开发板上比较稳定地运行 zCore
   - （可选）能够在 Linux 系统上运行 zCore libos
     这个相对比较简单，因为基于 Linux host OS，不会涉及太多指令级别的改动。可以作为入门练手。

8. 项目技术要求：
   - 熟悉 Rust 语言
   - 熟悉 RISC-V64 指令集，或做过其它平台的底层移植工作
   - 熟悉K210的开发经验
9. 相关的开源软件仓库列表：
   - https://github.com/rcore-os/zCore （zCore 仓库）
   - https://github.com/rcore-os/rCore （rCore 仓库，支持RISC-V）
   - https://github.com/kendryte/kendryte-doc-datasheet
   - https://github.com/sipeed/MaixPy


### rCore 到 zCore 的功能迁移

1. 项目标题：rCore 到 zCore 的功能迁移
2. [rCore 到 zCore 的功能迁移:项目模板](https://github.com/LearningOS/zCore)（注：由于有Github LFS，不能设置为classroom）
3. 项目描述：rCore 是用 Rust 语言实现的兼容 Linux 内核。它支持四种指令集，能够运行比较丰富的应用程序。但是随着时间的积累，rCore 的代码越堆越多，很多内部实现缺乏推敲，需要优化和重构。后来我们从头开始实现了 zCore 项目，采用了更加清晰的分层结构，同时复用 Zircon 微内核的内核对象实现了 Linux 内核的部分功能（如内存管理和进程管理）。目前 zCore 中的 linux 模块已经能够运行基础的 Busybox 等小程序，但仍有大量原本 rCore 支持的功能没有实现。本项目希望将 rCore 的功能迁移到 zCore 当中，并借此机会进行重构。其中一些代码可以直接搬过来，剩下的可能需要调整适配（例如涉及到 async），还有一些可以直接基于 Zircon 内核对象进行实现（例如 epoll）。
4. 项目难度：中（适合完成了lab1-5，并希望在系统类企业找到一份OS底层研发工作，或提升软件工程能力的同学）
5. 项目社区导师：董峰  github id: workerwork  weichat id: Mr_dong-feng
6. 项目社区导师：陈乐 github id: yuoo655    weichat id: Endagorion_
7. 项目产出要求：
   - 能够在 zCore 上运行 rCore 支持的 Linux 程序：GCC，Nginx，Rustc 等
   - 对新迁移过来的代码，要求补充代码文档和必要的单元测试
8. 项目技术要求：
   - 熟悉 Rust 语言
   - 熟悉 Linux 系统调用
9. 相关的开源软件仓库列表：
   - https://github.com/rcore-os/zCore （zCore 仓库）
   - https://github.com/rcore-os/rCore （rCore 仓库）
   
### 改进 RVM 虚拟机

1. 项目标题：改进 RVM 虚拟机
2. 项目描述：RVM 是在 rCore 中实现的一个简易 Hypervisor。目前只支持 x86_64，利用 VT-x 硬件虚拟化技术，已经能够在 rCore 上运行 uCore（C 语言版本的 x86 教学操作系统）。我们希望继续完善 RVM，使其能够支持运行完整的 Linux 系统。此外，还希望将 RVM 从 rCore 中分离出来，作为独立项目。未来不但能够接入 rCore、zCore，还能作为 Linux 内核模块运行。
3. 项目难度：高 （适合完成了lab1-5，并希望能够较快进入更实际一些的OS能力训练或做OS方向研究的同学）
4. 项目社区导师：贾越凯
5. 导师联系方式：github id: equation314    weichat id: equation314
6. 项目产出要求：
   - 将 RVM 从 rCore 中分离出来作为独立项目。
   - 能够在 rCore RVM 上运行完整的 Linux 系统。
   - （可选）能够把 RVM 作为一个 Linux module，在 Linux 启动后以 kernel module 的形式加载运行，然后把自己设定成 host 态，把 Linux 设定成 guest 态。

     这个有参考，即基于 C 语言实现的 Jailhouse 和 Blue Pill
   - （可选）支持 ARM64 的硬件虚拟化，能够在树莓派上运行虚拟机。
7. 项目技术要求：
   - 熟悉 Rust 语言
   - 熟悉虚拟化技术
   - 熟悉 x86_64 指令集，理解 VMX 的原理和基于 VMX 的开发
8. 相关的开源软件仓库列表：
   - https://github.com/rcore-os/rCore （rCore 仓库）
   - https://github.com/equation314/rCore/tree/rvm （正在开发中的包含 RVM 的 rCore 仓库）
   - https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials （树莓派上的 Rust OS 教程）
   - https://github.com/siemens/jailhouse （基于 Linux module 的 Hypervisor）


### 面向 Rust-based OS的vscode 动态调试工具

1. 项目标题：面向 Rust-based OS的vscode 动态调试工具
2. 项目描述：方便的源代码级调试工具，对监测程序运行状态和理解程序的逻辑十分重要；高效的Rust语言跟踪能力，是Rust操作系统内核开发的必要工具，对基于Rust的操作系统教学和实验很有帮助。然而现有RISC-V、Rust实验环境搭建成本高，上手难度大，不利于学习与开发工作。本项目拟实现一种基于网页访问的在线实验系统，提供方便、高效的手段实现在QEMU和RISC-V开发板上的Rust教学操作系统的源代码调试。
3. 项目难度：中（适合完成了lab1-5，熟悉typescript, 了解vscode plugin，并希望能够较快进入更实际一些的OS能力训练或做OS方向研究的同学）
4. 项目社区导师：
   陈志扬：github id: chenzhiy2001    weichat id: untilthedayibecome
   吴竞邦：github id: wujingbang   weichat id: wujingb33

5. . 项目产出要求：
   - 能够在vscode上动态调试Rust-based OS && APP in OS(比如rcore，zcore等)
   - 形成vscode插件
   - 提供使用说明和设计实现文档
6. 项目技术要求：
   - 熟悉 vscode 插件开发
   - 了解 VS Code的 Debug Adapter 协议
   - 了解QEMU 中的 gdbserver 与 gdb之间的通信协议
   - 熟悉 typescript开发
7. 相关的开源软件仓库列表：
   - https://github.com/chenzhiy2001/code-debug

注：欢迎进入第二阶段的同学提出自己感兴趣的题目，并尽快联系助教或老师进行立项。
