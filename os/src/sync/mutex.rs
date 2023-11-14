//! Mutex (spin-like and blocking(sleep))

use super::resource::ResourceManager;
use super::UPSafeCell;
use crate::task::{block_current_and_run_next, suspend_current_and_run_next};
use crate::task::{current_task, wakeup_task};
use crate::task::{current_task_tid, TaskControlBlock};
use alloc::vec::Vec;
use alloc::{collections::VecDeque, sync::Arc};

/// Mutex trait
pub trait Mutex: Sync + Send + ResourceManager {
    /// Lock the mutex
    fn lock(&self);
    /// Unlock the mutex
    fn unlock(&self);
}

/// Spinlock Mutex struct
pub struct MutexSpin {
    inner: UPSafeCell<MutexSpinInner>,
}

pub struct MutexSpinInner {
    locked: bool,
    allocated: Option<usize>,
}

impl MutexSpin {
    /// Create a new spinlock mutex
    pub fn new() -> Self {
        Self {
            inner: unsafe {
                UPSafeCell::new(MutexSpinInner {
                    locked: false,
                    allocated: None,
                })
            },
        }
    }
}

impl Mutex for MutexSpin {
    /// Lock the spinlock mutex
    fn lock(&self) {
        trace!("kernel: MutexSpin::lock");
        loop {
            let mut inner = self.inner.exclusive_access();
            if inner.locked {
                drop(inner);
                suspend_current_and_run_next();
                continue;
            } else {
                inner.locked = true;
                inner.allocated = Some(current_task_tid());
                return;
            }
        }
    }

    fn unlock(&self) {
        trace!("kernel: MutexSpin::unlock");
        let mut inner = self.inner.exclusive_access();
        inner.locked = false;
        inner.allocated = None;
    }
}

impl ResourceManager for MutexSpin {
    fn available(&self) -> usize {
        if self.inner.exclusive_access().locked {
            0
        } else {
            1
        }
    }

    fn allocation(&self) -> Vec<usize> {
        let mut vec = Vec::new();
        let inner = self.inner.exclusive_access();
        if inner.locked {
            vec.push(inner.allocated.unwrap());
        }
        vec
    }

    fn need(&self) -> Vec<usize> {
        Vec::new()
    }
}

/// Blocking Mutex struct
pub struct MutexBlocking {
    inner: UPSafeCell<MutexBlockingInner>,
}

pub struct MutexBlockingInner {
    locked: bool,
    allocated: Option<usize>,
    wait_queue: VecDeque<Arc<TaskControlBlock>>,
}

impl MutexBlocking {
    /// Create a new blocking mutex
    pub fn new() -> Self {
        trace!("kernel: MutexBlocking::new");
        Self {
            inner: unsafe {
                UPSafeCell::new(MutexBlockingInner {
                    locked: false,
                    allocated: None,
                    wait_queue: VecDeque::new(),
                })
            },
        }
    }
}

impl Mutex for MutexBlocking {
    /// lock the blocking mutex
    fn lock(&self) {
        trace!("kernel: MutexBlocking::lock");
        let mut mutex_inner = self.inner.exclusive_access();
        if mutex_inner.locked {
            mutex_inner.wait_queue.push_back(current_task().unwrap());
            drop(mutex_inner);
            block_current_and_run_next();
        } else {
            mutex_inner.locked = true;
            mutex_inner.allocated = Some(current_task_tid());
        }
    }

    /// unlock the blocking mutex
    fn unlock(&self) {
        trace!("kernel: MutexBlocking::unlock");
        let mut mutex_inner = self.inner.exclusive_access();
        assert!(mutex_inner.locked);
        if let Some(waking_task) = mutex_inner.wait_queue.pop_front() {
            wakeup_task(waking_task);
        } else {
            mutex_inner.locked = false;
            mutex_inner.allocated = None;
        }
    }
}

impl ResourceManager for MutexBlocking {
    fn available(&self) -> usize {
        if self.inner.exclusive_access().locked {
            0
        } else {
            1
        }
    }

    fn allocation(&self) -> Vec<usize> {
        let mut vec = Vec::new();
        let inner = self.inner.exclusive_access();
        if inner.locked {
            vec.push(inner.allocated.unwrap());
        }
        vec
    }

    fn need(&self) -> Vec<usize> {
        let mut vec = Vec::new();
        let inner = self.inner.exclusive_access();
        for t in inner.wait_queue.iter() {
            vec.push(t.inner_exclusive_access().res.as_ref().unwrap().tid);
        }
        vec
    }
}
