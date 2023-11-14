//! Semaphore

use crate::sync::UPSafeCell;
use crate::task::{
    block_current_and_run_next, current_task, current_task_tid, wakeup_task, TaskControlBlock,
};
use alloc::collections::BTreeSet;
use alloc::vec::Vec;
use alloc::{collections::VecDeque, sync::Arc};

use super::ResourceManager;

/// semaphore structure
pub struct Semaphore {
    /// semaphore inner
    pub inner: UPSafeCell<SemaphoreInner>,
}

pub struct SemaphoreInner {
    pub count: isize,
    pub allocated: BTreeSet<usize>,
    pub wait_queue: VecDeque<Arc<TaskControlBlock>>,
}

impl Semaphore {
    /// Create a new semaphore
    pub fn new(res_count: usize) -> Self {
        trace!("kernel: Semaphore::new");
        Self {
            inner: unsafe {
                UPSafeCell::new(SemaphoreInner {
                    count: res_count as isize,
                    allocated: BTreeSet::new(),
                    wait_queue: VecDeque::new(),
                })
            },
        }
    }

    /// up operation of semaphore
    /// 释放资源
    pub fn up(&self) {
        trace!("kernel: Semaphore::up");
        let mut inner = self.inner.exclusive_access();
        inner.count += 1;
        if inner.count <= 0 {
            if let Some(task) = inner.wait_queue.pop_front() {
                wakeup_task(task);
            }
        }
        let tid = current_task_tid();
        inner.allocated.remove(&tid);
    }

    /// down operation of semaphore
    /// 分配资源
    pub fn down(&self) {
        trace!("kernel: Semaphore::down");
        let mut inner = self.inner.exclusive_access();
        inner.count -= 1;
        if inner.count < 0 {
            inner.wait_queue.push_back(current_task().unwrap());
            drop(inner);
            block_current_and_run_next();
        } else {
            // 成功分配
            let tid = current_task_tid();
            inner.allocated.insert(tid);
        }
    }
}

impl ResourceManager for Semaphore {
    fn available(&self) -> usize {
        let cnt = self.inner.exclusive_access().count;
        if cnt < 0 {
            0
        } else {
            cnt as usize
        }
    }

    fn allocation(&self) -> Vec<usize> {
        let inner = self.inner.exclusive_access();
        let mut vec = Vec::new();
        for tid in inner.allocated.iter() {
            vec.push(*tid);
        }
        vec
    }

    fn need(&self) -> Vec<usize> {
        let inner = self.inner.exclusive_access();
        let mut vec = Vec::new();
        for task in inner.wait_queue.iter() {
            vec.push(task.inner_exclusive_access().res.as_ref().unwrap().tid);
        }
        vec
    }
}
