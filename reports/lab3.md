## 编程作业

* 将`ch4`相关代码搬到`ch5`
* 实现`fn sys_spawn(path: *const u8) -> isize`，不复制父进程地址空间
* 实现 stride 调度算法和`fn sys_set_priority(prio: isize) -> isize`，`BIG_STRIDE = 1 << 31`



## 问答作业

stride 算法深入

* 实际情况是轮到 p1 执行吗？为什么？

  不是，因为整型溢出，$(250+10)\%256=4$，p2先执行
  
* 证明 $\text{STRIDE\_MAX} – \text{STRIDE\_MIN} \leqslant \text{BigStride} / 2$：
  
  使用反证法，假设存在某一时刻：
  $$
  \text{StrideMax}^t\ –\ \text{StrideMin}^t > \text{BigStride}\ /\ 2
  $$
  也就是：
  $$
  \text{stride}_{p_1}^t - \text{stride}_{p_2}^t > \text{BigStride}\ /\ 2
  $$
  假设在 $t$ 时刻：
  $$
  \text{stride}_{p_1}^t=\text{StrideMax}^t,\ \text{stride}_{p_2}^t=\text{StrideMin}^t \\
  \text{stride}_{p_2}^t < \text{stride}_{p_i}^t < \text{stride}_{p_1}^t,\ \forall i \in [3, n] \\
  \text{stride}_{p_1}^t - \text{stride}_{p_2}^t > \frac{\text{BigStride}}2
  $$
  
  
  此时有以下几种可能：
  
  
  
  1. $t-1$ 时刻执行进程 $p_1$，则有：
     $$
     \text{stride}_{p_1}^{t-1} < \text{stride}_{p_2}^{t-1}=\text{stride}_{p_2}^t, \\
     \text{pass}_{p_1}^{t-1} = \text{stride}_{p_1}^t - \text{stride}_{p_1}^{t-1} > \text{stride}_{p_1}^t - \text{stride}_{p_2}^t > \frac{\text{BigStride}}2
     $$
     因为
     $$
     \text{pass}_{p_1}^{t-1} = \frac{\text{BigStride}}{\text{prio}_{p_1}} \leqslant \frac{\text{BigStride}}2
     $$
     矛盾，故不成立
  
     
  
  2. $t-1$ 时刻执行进程 $p_2$，则有：
     $$
     \text{stride}_{p_i}^{t-1} = \text{stride}_{p_i}^t,\ \forall i \in [1,n] \backslash \{2\} \\
     \text{stride}_{p_2}^{t-1} < \text{stride}_{p_2}^t
     $$
     考虑 $t-2$ 时刻，同理有：执行进程 $p_2$，
     $$
     \text{stride}_{p_i}^{t-2} = \text{stride}_{p_i}^{t-1} = \text{stride}_{p_i}^t,\ \forall i \in [1,n] \backslash \{2\} \\
     \text{stride}_{p_2}^{t-2} < \text{stride}_{p_2}^{t-1}
     $$
     迭代，直到
     $$
     t' = t - \frac{\text{stride}_{p_2}^t}{prio_{p_2}}
     $$
     时刻，此时：
     $$
     \text{stride}_{p_i}^{t'} = \text{stride}_{p_i}^t > \text{StrideMin}^t \geqslant 0,\ \forall i \in [1,n] \backslash \{2\} \\
     \text{stride}_{p_2}^{t'} = 0
     $$
     则考虑 $t'-1$ 时刻：必然是执行进程 $p_{j_0}$，且
     $$
     \text{stride}_{p_{j_0}}^{t'-1} = 0,\ j_0 \in [1,n] \backslash \{2\}
     $$
     再次迭代，直到 $t'-k$ 时刻执行 $p_1$（$j_k = 1$），且
     $$
     \text{stride}_{p_1}^{t'-k} = 0,\ \text{pass}_{p_1}^{t'-k} = \frac{\text{BigStride}}{\text{prio}_{p_1}} \leqslant \frac{\text{BigStride}}2
     $$
     按算法过程，$p_1$ 不会在直到 $t$ 时刻期间任何时刻再被调度，故
     $$
     \text{stride}_{p_1}^t = \text{pass}_{p_1}^{t'-k} \leqslant \frac{\text{BigStride}}2
     $$
     而已知
     $$
     \text{stride}_{p_1}^t > \frac{\text{BigStride}}2 + \text{stride}_{p_2}^t
     $$
     矛盾，故不成立
  
  3. $t-1$ 时刻执行 $p_i, i \in [3,n]$，则必然有：
     $$
     \text{stride}_{p_i}^{t-1} < \text{stride}_{p_2}^{t-1} = \text{stride}_{p_2}^t = \text{StrideMin}^t
     $$
     否则 $p_i$ 不会被调度
  
     假设 $p_1$ 上一次被调度是在 $t'_1-1$，$p_2$ 上一次被调度是在 $t'_2 - 1$。
  
     * 若 $t'_2-1>t'_1-1$，即上一次 $p_1$ 先于 $p_2$ 调度，则在 $t_2'-1$ 时刻：
       $$
       \text{stride}_{p_2}^{t_2'-1} = \text{StrideMin}^{t'_2-1}, \text{stride}^{t'_2-1}_{p_1}=\text{StrideMax}^{t'_2-1} \\
       \text{stride}_{p_2}^{t_2'-1} < \text{stride}_{p_i}^{t_2'-1} < \text{stride}_{p_1}^{t_2'-1},\ \forall i \in [3, n] \\
       \text{stride}_{p_1}^{t_2'-1} - \text{stride}_{p_2}^{t_2'-1} = \text{stride}_{p_1}^t  - \text{stride}_{p_2}^{t_2'-1} > \text{stride}_{p_1}^t  - \text{stride}_{p_2}^t > \frac{\text{BigStride}}2
       $$
       根据可能性2，将 $t'_2-1$ 替换为 $t$，分析过程相同，最终得到矛盾
  
     * 若 $t'_2-1<t'_1-1$，即上一次 $p_2$ 先于 $p_1$ 调度，则在 $t_1'-1$ 时刻，设 $t_1'$ 时刻调度进程 $p_k$：
       $$
       \text{stride}_{p_1}^{t'_1}=\text{StrideMax}^{t'_1}=\text{StrideMax}^t,\ \text{stride}_{p_k}^{t'_1}=\text{StrideMin}^{t'_1} \\
       \text{stride}_{p_k}^{t'_1} < \text{stride}_{p_i}^{t'_1} < \text{stride}_{p_1}^{t'_1},\ \forall i \in [3, n] \\
       \text{stride}_{p_1}^{t'_1} - \text{stride}_{p_k}^{t'_1} = \text{stride}_{p_1}^t - \text{stride}_{p_k}^{t'_1} > \text{stride}_{p_1}^t - \text{stride}_{p_2}^{t'_1} = \text{stride}_{p_1}^t - \text{stride}_{p_2}^t > \frac{\text{BigStride}}2
       $$
       
       根据可能性1，将 $t_1'-1$ 替换为 $t$，将 $p_2$ 替换为 $p_k$，分析过程相同，最终得到矛盾
  
  故严格证明 $\text{STRIDE\_MAX} – \text{STRIDE\_MIN} \leqslant \text{BigStride} / 2$
  
  * ```rust
    use core::cmp::Ordering;
    struct Stride(u64);
    
    const BIG_STRIDE:u8 = 255;
    const PASSMAX:u8 = BIG_STRIDE/2;
    /// 一旦
    impl PartialOrd for Stride {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            let s = self.0 as u8;
            let o = other.0 as u8;
            if s > o {
                let diff = s - o;
                if diff > PASSMAX {
                    Some(Ordering::Less)
                }else {
                    Some(Ordering::Greater)
                }
            } else {
                let diff = o - s;
                if diff > PASSMAX {
                    Some(Ordering::Greater)
                }else {
                    Some(Ordering::Less)
                }
            }
    
        }
    }
    
    impl PartialEq for Stride {
        fn eq(&self, other: &Self) -> bool {
            false
        }
    }
    ```

  

## 荣耀准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：无
2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：无
3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。
4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。