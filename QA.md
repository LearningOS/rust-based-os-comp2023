Q1:已经在classroom中建立了自己的仓库（例如 “LearningOS/lab0-0-setup-env-run-os1-chyyuu2022"），但是源仓库“LearningOS/rust-based-os-comp2022“更新了，如何处理？

A：

**方法一：**

​       向管理员“助教许善朴”申请删除已生成仓库，再点击 链接重新创建仓库。

**方法二：**

​       在自己构建的仓库根目录下执行以下命令:

```makefile
git remote add upstream "https://github.com/LearningOS/rust-based-os-comp2022.git"
git fetch upstream
git checkout -b foo
git branch -D main
git checkout -t upstream/main
git reset --hard origin/main
git push -f
```