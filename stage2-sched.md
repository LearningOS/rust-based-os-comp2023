## 2022年开源操作系统训练营：第二阶段

## 相关信息
- **注意：** 为及时了解和指导同学的学习和实践情况并推动学生相互帮助，本次活动要求学生把每日学习实践的过程记录在自己在github的公开repo上。可参见[每日学习实践的具体例子](https://github.com/GCYYfun/DailySchedule)。



## 日程：第一周：zCore讲解与学习（7天）
- zCore结构和核心组件
- zCore系统调用
- zCore硬件移植与驱动开发
- zCore应用开发

## 日程：第二~第四周：rCore/zCore研究项目 (34天)
目前设置了多个项目，学生可选择参加下面的项目，指导老师制定以周为单位的项目研究计划。


### rCore-Tutorial-v3 进一步扩展
1. 项目标题：rCore-Tutorial-v3 进一步扩展
2. 项目描述：rCore-Tutorial-v3 的一个重要目标是以简洁的设计实现对应到操作系统的核心知识点上，所以还有很多可以扩展的地方。我们希望通过设计一个一个相对独立的实验，来展现操作系统的核心设计思想，在操作系统实现与操作系统原理之间建立桥梁。
3. 项目难度：低/中
4. 项目社区导师：陈渝 github id: chyyuu     weichat id: chyyuu 
5. 项目社区导师：吴一凡 github id: wyfcyx     weichat id: yifanwu1998
6. 项目产出要求：
   - 补充完善 rCore-Tutorial-v3

7. 项目技术要求：
   - 具备一定的 Rust 语言基础，能看懂并模仿现有代码即可
   - 熟悉操作系统原理与简易实现
   - 具有基础英文阅读和写作能力

8. 相关的开源软件仓库列表：
  - https://github.com/rcore-os/rCore-Tutorial-v3
  - https://rcore-os.github.io/rCore-Tutorial-Book-v3/final-lab.html

### 操作系统课程的有趣大实验
1. 项目标题：操作系统课程的有趣大实验
2. 项目描述：操作系统课程的大实验的重要目标基于学生兴趣来开展各种操作系统技术的探索，拓展学生的视野，培养未来操作系统人才。
3. 项目难度：中/高
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
### zCore 的文档与单元测试完善

1. 项目标题：zCore 的文档与单元测试完善
2. 项目描述：zCore 是用 Rust 实现的支持协程/异步等新机制的操作系统，目前zCore的实现已经初步支持 x86_64/RISC-V 64/AArch64。然而，随着后期开发进程的加快，大量代码缺少文档描述，并且没有实现单元测试，只能在 QEMU 中运行用户程序以检验代码正确性。本项目的目标是完善 zCore 的文档及单元测试，使其成为一个高质量的 Rust 社区项目。
3. 项目难度：低
4. 项目社区导师：石振兴
5. 导师联系方式：github id: shzhxh    weichat id: szx-bj
6. 项目产出要求：
   - 补充完善 zCore 各模块的代码文档。

     目标是通过 `#![deny(missing_docs)]` 编译，并且能够让开发者通过阅读文档，快速理解 zCore 的代码结构和各部分功能。
   - 参考 Fuchsia 官方文档及测试代码，为 zCore 中的内核对象补充单元测试。

     目标让 `zircon-object` 模块的测试覆盖率提高到 90% 以上。
   - （可选）在 CI 中支持运行集成测试。
     目标是最大化整体的测试覆盖率。
   - （可选）在zCore中添加zircon/linux的syscall。
     目标：完善添加zCore内核功能，让zCore通过更多的zircon tests(基于zircon的coretest)或Linux tests（基于musl libc的libc test）或相关应用

7. 项目技术要求：
   - 具备一定的 Rust 语言基础，能看懂并模仿现有代码即可
   - 熟悉操作系统原理与简易实现
   - 具有基础英文阅读和写作能力
8. 相关的开源软件仓库列表：
   - https://github.com/rcore-os/zCore （zCore 仓库）
   - https://rcore-os.github.io/zCore/zircon_object （zCore 代码文档）
   - https://fuchsia.dev/fuchsia-src/reference （Zircon 官方文档）


### zCore 的星光/SiFive/树莓派等开发板和外设支持

1. 项目标题：zCore 的星光/SiFive开发板支持
2. 项目描述：zCore 是用 Rust 实现的支持协程/异步等新机制的操作系统，目前zCore的实现已经初步支持 x86_64/RISC-V 64/AArch64。zCore需要继续完善相关驱动和系统调用。
3. 项目难度：中
4. 项目社区导师：肖络元
5. 导师联系方式：github id: shzhxh    weichat id: xiaoxiaoluckyard
6. 项目产出要求：
   - 能够在某开发板上比较稳定地运行 zCore
   - （可选）能够在 Linux 系统上运行 zCore libos
     这个相对比较简单，因为基于 Linux host OS，不会涉及太多指令级别的改动。可以作为入门练手。

7. 项目技术要求：
   - 熟悉 Rust 语言
   - 熟悉 RISC-V64 指令集，或做过其它平台的底层移植工作
   - 熟悉K210的开发经验
8. 相关的开源软件仓库列表：
   - https://github.com/rcore-os/zCore （zCore 仓库）
   - https://github.com/rcore-os/rCore （rCore 仓库，支持RISC-V）
   - https://github.com/kendryte/kendryte-doc-datasheet
   - https://github.com/sipeed/MaixPy


### rCore 到 zCore 的功能迁移

1. 项目标题：rCore 到 zCore 的功能迁移
2. 项目描述：rCore 是用 Rust 语言实现的兼容 Linux 内核。它支持四种指令集，能够运行比较丰富的应用程序。但是随着时间的积累，rCore 的代码越堆越多，很多内部实现缺乏推敲，需要优化和重构。后来我们从头开始实现了 zCore 项目，采用了更加清晰的分层结构，同时复用 Zircon 微内核的内核对象实现了 Linux 内核的部分功能（如内存管理和进程管理）。目前 zCore 中的 linux 模块已经能够运行基础的 Busybox 等小程序，但仍有大量原本 rCore 支持的功能没有实现。本项目希望将 rCore 的功能迁移到 zCore 当中，并借此机会进行重构。其中一些代码可以直接搬过来，剩下的可能需要调整适配（例如涉及到 async），还有一些可以直接基于 Zircon 内核对象进行实现（例如 epoll）。
3. 项目难度：中
4. 项目社区导师：陈乐
5. 导师联系方式：github id: yuoo655    weichat id: Endagorion_
6. 项目产出要求：
   - 能够在 zCore 上运行 rCore 支持的 Linux 程序：GCC，Nginx，Rustc 等
   - 对新迁移过来的代码，要求补充代码文档和必要的单元测试
7. 项目技术要求：
   - 熟悉 Rust 语言
   - 熟悉 Linux 系统调用
8. 相关的开源软件仓库列表：
   - https://github.com/rcore-os/zCore （zCore 仓库）
   - https://github.com/rcore-os/rCore （rCore 仓库）
   
### 改进 RVM 虚拟机

1. 项目标题：改进 RVM 虚拟机
2. 项目描述：RVM 是在 rCore 中实现的一个简易 Hypervisor。目前只支持 x86_64，利用 VT-x 硬件虚拟化技术，已经能够在 rCore 上运行 uCore（C 语言版本的 x86 教学操作系统）。我们希望继续完善 RVM，使其能够支持运行完整的 Linux 系统。此外，还希望将 RVM 从 rCore 中分离出来，作为独立项目。未来不但能够接入 rCore、zCore，还能作为 Linux 内核模块运行。
3. 项目难度：高
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

