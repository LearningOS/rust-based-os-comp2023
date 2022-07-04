# Open-Source OS Training Comp 2022

Welcome to Open-Source OS Training Comp 2022（2022年开源操作系统训练营）.
## Scheduling
- NOTICE current [**SCHEDULING**](./scheduling.md)（2022年开源操作系统训练营的计划安排）!
## History
- [open-source  os training comp 2021](https://github.com/rcore-os/rCore/wiki/os-tutorial-summer-of-code-2021)
- [open-source  os training  comp 2020](https://github.com/rcore-os/rCore/wiki/os-tutorial-summer-of-code-2020)

## Guide & Lab Test for Rust
- [learning rust guide](https://course.rs/)（Rust语言圣经）
- [Rust-lang Lab Test based on Rustlings](https://classroom.github.com/a/YTNg1dEH)（采用Github Classroom模式的Rustling小练习）
## Guide for OS
- Guide deployed version can be found [here](https://learningos.github.io/rust-based-os-comp2022/)（精简OS实验指导）
- [rCore Tutorial v3 Guide](https://rcore-os.github.io/rCore-Tutorial-Book-v3/) (2022春季OS课程讲义)
- [OS Course Slides in 2022 spring](https://learningos.github.io/os-lectures/)(2022春季OS课程Slides)
- [OS Course Videos in 2022 spring](./relatedinfo.md)(2022春季OS课程Videos)
- [OS API Docs in 2022 spring](./relatedinfo.md)(2022春季OS课程实验中 ``cargo doc`` 生成的各章参考OS的API文档)
 
First, you can read [Guide](https://learningos.github.io/rust-based-os-comp2022/) to:
- (Important) `git clone` this repo.
- (Important) setup development environment.
- (Important) setup classroom CI by `make setupclassroom`  (if you using githubclassroom to devel labs)
- analyze and try os1 and os2 labs.
- (Option) you can [use github classroom&code space to develop all os labs](https://learningos.github.io/rust-based-os-comp2022/chapter1/0intro.html#id4) just in a net browser.


## os reference framework
The 'os[1-8]-ref' are the 'os[1-8]'  reference frameworks.  You can read and copy some codes into os[1-8]

## kernel labs
There are two simple practice work, and five kernel labs.

注：登录github.com，点击并打开下面的lab0~lab5的github classroom的链接后，点击醒目绿色按钮"Accept this assignment"选择接受，github就为你创建了一个对应实验的git repo

According to the  [Guide](https://learningos.github.io/rust-based-os-comp2022/), write os codes for:
- [lab0-0 setup env in 'os1' DIR](https://classroom.github.com/a/hnoWuKGF) ：[Important NOTICE](https://learningos.github.io/rust-based-os-comp2022/chapter1/0intro.html#id4)
- [lab0-1 try something in os2 in 'os2' DIR](https://classroom.github.com/a/UEOvz4qO) ：[Important NOTICE](https://learningos.github.io/rust-based-os-comp2022/chapter2/0intro.html#id3)
- [lab1 in 'os3' DIR](https://classroom.github.com/a/s1v7GyJM) ：[Important NOTICE](https://learningos.github.io/rust-based-os-comp2022/chapter3/0intro.html#id3)
- [lab2 in 'os4' DIR](https://classroom.github.com/a/ghbB1wYX) ：[Important NOTICE](https://learningos.github.io/rust-based-os-comp2022/chapter4/0intro.html#id3)
- [lab3 in 'os5' DIR](https://classroom.github.com/a/RxB6h4-x) ：[Important NOTICE](https://learningos.github.io/rust-based-os-comp2022/chapter5/0intro.html#id3)
- [lab4 in 'os6' DIR](https://classroom.github.com/a/94eMW8zi) ：[Important NOTICE](https://learningos.github.io/rust-based-os-comp2022/chapter6/0intro.html#id3)
- [lab5 in 'os8' DIR](https://classroom.github.com/a/zqGJEPK-) ：[Important NOTICE](https://learningos.github.io/rust-based-os-comp2022/chapter8/0intro.html#id5)


## Check your results
- lab1: `make test3` for lab1
- lab2: `make test4`  for lab2 
- lab3: `make test5`  for lab3
- lab4: `make test6`  &&  `make test7` for lab4 
- lab5: `make test8`  for lab5 

## QA &Send issues

There are some methods for common questions & problems, please read [QA](./QA.md).

If  you meet any other new problems or some difficulties, please send issues to [Issues in rust-based-os-comp2022](https://github.com/LearningOS/rust-based-os-comp2022/issues)