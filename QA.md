# 常见问题解答

## Q0：GitHub classroom是啥？如何使用？

### A:

     - [B站的GitHub Classroom 视频介绍](https://www.bilibili.com/video/BV12L41147r7?spm_id_from=333.337.search-card.all.click&vd_source=8e19ee6e49f598fda8c17e306d8b3726) 
     - [Youtube的GitHub Classroom视频介绍](https://www.youtube.com/playlist?list=PLIRjfNq867bewk3ZGV6Z7a16YDNRCpK3u)
     - [github文档：使用 GitHub Classroom 教学](https://docs.github.com/cn/education/manage-coursework-with-github-classroom/get-started-with-github-classroom)
  
## Q1：已经在classroom中建立了自己的仓库（例如 “LearningOS/lab0-0-setup-env-run-os1-chyyuu2022"），但是源仓库“LearningOS/rust-based-os-comp2022“更新了，如何处理？

### A：

**方法一：**

      重新点击加入课程的链接，在页面下方会有一行字“We've configured the repository associated with this assignment (update)”，“update”是一个链接，点击update就可以把自己的仓库更新到与最新状态的repository template一致。

​      
**方法二：**

​     在自己构建的仓库根目录下执行以下命令:

```makefile
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

##  Q2：在classroom中建立了自己的仓库中，进行提交 `git push` 后，触发 CI后，出现 Annotations 错误“The job was not stared because recent account payments have failed or your spending limit needs to be increased. Please check the 'Billing & plans' section in your settings”，无法完成自动CI功能，比如 `Autograding` 等。

### A:

**方法一：**
    
    这是由于对用户的私有仓库进行CI 相关的github action是需要付费的。用户可通过给自己的github账户充值来解决。https://docs.github.com/cn/billing/managing-billing-for-github-actions/about-billing-for-github-actions 给出了具体信息。

**方法二：**

    对用户的公开仓库进行CI github action是不需要付费的。在项目的 `Settings` -> `Change visibility` 将项目改成Public, 重新触发Action。 
    目前设置了让用户具有修改自己的项目从private --> public的能力。
    如果用户还是发现自己的权限不够，或看不到  `Settings`  这个选项，可以通过联系助教帮助来解决。

## Q3：我刚开始准备学习Rust，是Rust新手，我应该如何入门？

### A:

  - [Rust 大佬给初学者的学习建议](https://github.com/rustlang-cn/Rustt/blob/main/Articles/%5B2022-04-02%5D%20Rust%20%E5%A4%A7%E4%BD%AC%E7%BB%99%E5%88%9D%E5%AD%A6%E8%80%85%E7%9A%84%E5%AD%A6%E4%B9%A0%E5%BB%BA%E8%AE%AE.md)
  - [张汉东：学习 Rust 你需要一个认知框架](https://zhuanlan.zhihu.com/p/494001676)
  - [Rust语言圣经(Rust Course)](https://course.rs/)
  - [Rust速查表（cheatsheet）](https://cheats.rs/) 该项目不仅提供了基础的语法速查，还有执行顺序详解和编写时需要关注的注意事项。项目还包含了示例代码（EX）、书籍（BK）、标准（STD）等相关资料的扩展。

## Q4：我不熟悉GitHub和Git，有啥快速入门的资源吗？

### A:

   - [包括：从 0 开始学习 GitHub 系列1-7](https://jtxiao.com/main/categories/%E5%B7%A5%E5%85%B7/)
   - [超级简单的Git入门](https://backlog.com/git-tutorial/cn/)
   - [git - 简明指南](https://rogerdudler.github.io/git-guide/index.zh.html)
   - [中文git-tips](https://github.com/521xueweihan/git-tips)
   - [GitHub 官方制作的 Git 速查表](https://education.github.com/git-cheat-sheet-education.pdf)

## Q5：我不熟悉Linux的各种命令，有啥快速入门的资源吗？

### A:

   - [中文Linux命令（linux-command）搜索引擎](https://wangchujiang.com/linux-command/)：随用随搜Linux命令，而且还支持中文搜索
   - [新版 Linux 命令百科全书》（英文）](https://github.com/tldr-pages/tldr)

## Q6：我碰到一些命令/应用(比如vim, curl)、操作（比如vscode）或语言用法（比如Makefile）等不知到哪里能快速查找，怎么办？
  
### A:

   - [Rico's cheatsheets](https://devhints.io/) 开源、全面的速查表网站，涵盖了前端、后端、运维、IDE 多个方面，而且界面友好简洁支持在线查看
   - [所有与命令行相关的cheatsheet](http://cheat.sh/)：号称「你唯一需要的命令行相关速查表」