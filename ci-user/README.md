## rCore_tutorial_v3 TESTS

本项目用于为 rCore 实验进行 CI 测试，在 user 目录下 `make all CHAPTER=x` 可获得第 x 章的测例。

- 可选项 2, 3_0, 3_2, 4, 5, 6, 7。

**重要**-加载地址更新：

- chapter2 所有程序加载位置位于 0x80400000，与示例代码一致。
- chapter3 测试程序分为 3 批，每一批的地址都为 0x80400000 + id\*0x20000，id 为程序在这一批中的序号。每一批都与参考代码一致，请分别测试。
- chapter4-7 所有程序加载位置位于 0x0，与示例代码一致。

可以在 `user/build/asm` 目录下查看汇编来确认加载地址。

**测例更新**

- 一部分无用测例已删除，包括 ch2_helloworld, ch3_1_yield 等。
- sleep 测例被转移到第四章
- ch4 之后不再测试 write1

rust 的把user测例分散到了各个branch里，当时想的是尽量把测试的过程屏蔽掉，现在看确实不便于管理，这学期就先这样算了.

### 各章的测例

#### ch3

test1：write0 write1

test2：setprio

test3：stride的六个测例

#### ch4

test1：sleep0 sleep1 测试 sys_time

test2：map0123 unmap12 测试 map unmap 实现

#### ch5 6 7 8

```rust
    "test_sleep\0",
    "test_sleep1\0",
    "test_mmap0\0",
    "test_mmap1\0",
    "test_mmap2\0",
    "test_mmap3\0",
    "test_unmap\0",
    "test_unmap2\0",
    "test_spawn0\0",
    "test_spawn1\0",
    // ch6
    "test_mail0\0",
    "test_mail1\0",
    "test_mail2\0",
    "test_mail3\0",
	// ch7
    "test_file0\0",
    "test_file1\0",
    "test_file2\0",
    // ch8
    ...
```

share mem 的测例放着就行。

ch8 先不管。