# 常见问题解答
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

    对用户的公开仓库进行CI action是不需要付费的。在项目 `Settings` -> `Change visibility` 将项目改成Public, 重新触发Action。 如果权限不够，可联系助教帮助。
