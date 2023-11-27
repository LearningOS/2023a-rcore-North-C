# Lab1: 实现 sys_task_info

## 内容总结

实现了 sys_task_info 系统调用，能够使用户获取到对应任务的任务信息。通过在 `TaskControlBlock` 数据结构中增加 start_time 和 syscall_count 字段，记录任务的初次运行时间和系统调用次数，并在 TaskManager 增添对应处理方法。


## 简答题
解答：

1. 版本为：`[rustsbi] RustSBI version 0.3.0-alpha.2, adapting to RISC-V SBI v1.0.0`。
错误行为：
* bad_address.rs 触发 PageFault,内存访问异常。错误描述为：`[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003c4, kernel killed it.`
* bad_instruction.rs 触发 IllegalInstruction，错误描述为：`IllegalInstruction in application, kernel killed it.`
* bad_register.rs 触发 IllegalInstruction，错误描述为：`IllegalInstruction in application, kernel killed it.`

2. 
    1. 刚进入 `__restore`时，a0 代表 TrapContext结构的地址。两种使用场景如下：第一种，
    2. 处理了 `sstatus`，`sepc`, `sscratch` 三个寄存器。`sstatus` 负责保存特权级切换前的原有特权级，`sepc`负责保存Trap发生时执行的指令地址，`sscratch`负责保存用户态堆栈的地址。
    3. 根据函数调用规范，x2 对应的是 sp，此时sp 对应与 内核栈，应该保存的是保存在sscratch 当中的用户栈地址，但是需要后续 cssr 指令来实现，所以暂时保留位置。x4 对应的是 tp 寄存器，用于线程相关的变量保存，程序不需要使用，则跳过。
    4. sp 存储用户栈栈顶地址，sscratch 存储内核栈栈顶地址。
    5. sret 指令进行特权级的切换，从 S 特权级下降到 U 特权级，即进入用户态。
    6. 该指令之后，sp 保存内核栈栈顶地址， sscratch 保存用户栈栈顶地址。
    7. 执行 `ecall` 指令

## 荣誉准则

在完成本次实验的过程（含此前学习的过程）中，我曾分别与**以下各位**就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

> 无

此外，我也参考了**以下资料**，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

> 无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。