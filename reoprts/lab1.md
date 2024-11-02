# Lab1 Report

## 实现

给 Task Object 加上了 syscall_count 属性字段
打桩使得每次程序

## 简答

### Q1

```plaintext
[rustsbi] RustSBI version 0.4.0-alpha.1, adapting to RISC-V SBI v2.0.0
...
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003ac, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
```

- StoreFault
- IllegalInstruction
- IllegalInstruction

### Q2

1. `a0` 代表上一个 `TaskContext` 的指针
   `__restore`
   - 恢复 Trap 时的上下文
   - 恢复任务切换时的上下文
2. - `sstatus` 给出 Trap 发生前的特权级状态
   - `sepc` 记录 Trap 发生之前执行的最后一条指令的地址
   - `sscratch` 描述 Trap 的原因
3. `x2` 会在 `trap_handler` 调用后恢复， `x4` 指向用不到的线程特定变量。
4. `sp`存储了用户栈的栈顶指针, `sscratch`存储了内核栈的栈顶指针。
5. `sret`，指令的作用就是从核态返回
6. `sp` 存储了内核栈的栈顶指针, `sscratch`存储了用户栈的栈顶指针
7. `ecall`

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
