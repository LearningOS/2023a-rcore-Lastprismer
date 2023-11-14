use alloc::vec::Vec;

/// 锁的抽象
pub trait ResourceManager {
    /// 计算当前可分配的资源数
    fn available(&self) -> usize;
    /// 计算当前分配给的线程
    ///
    /// mutex和semaphore给每个线程至多分配一个资源
    fn allocation(&self) -> Vec<usize>;
    /// 计算当前有需求的线程
    ///
    /// mutex和semaphore使每个线程至多需求一个资源
    fn need(&self) -> Vec<usize>;
}
