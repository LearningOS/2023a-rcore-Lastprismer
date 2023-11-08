//! Process management syscalls
use core::mem::size_of;

use crate::{
    config::MAX_SYSCALL_NUM,
    mm::{copyout, ptr2bytes, MapPermission, VirtAddr},
    task::{
        change_program_brk, current_user_token, exit_current_and_run_next, get_task_info, mmap,
        munmap, suspend_current_and_run_next, TaskStatus,
    },
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
///
pub struct TimeVal {
    ///
    pub sec: usize,
    ///
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let timeval = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    copyout(
        current_user_token(),
        _ts as *const u8,
        size_of::<TaskInfo>(),
        ptr2bytes(&timeval as *const _ as *const u8, size_of::<TimeVal>()),
    );
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    let taskinfo = get_task_info();
    copyout(
        current_user_token(),
        _ti as *const u8,
        size_of::<TaskInfo>(),
        ptr2bytes(&taskinfo as *const _ as *const u8, size_of::<TaskInfo>()),
    );
    0
}

/// YOUR JOB: Implement mmap.
///
/// start 需要映射的虚存起始地址，要求按页对齐
///
/// len 映射字节长度，可以为 0
///
/// port：第 0 位表示是否可读，第 1 位表示是否可写，第 2 位表示是否可执行。其他位无效且必须为 0
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap");
    if _start % 4096 != 0 || _port & !0x7 != 0 || _port & 0x7 == 0 {
        return -1;
    }
    let mut perm = MapPermission::empty() | MapPermission::U;
    if _port & 1 != 0 {
        perm |= MapPermission::R
    }
    if _port & 2 != 0 {
        perm |= MapPermission::W;
    }
    if _port & 4 != 0 {
        perm |= MapPermission::X;
    }
    let start_va = VirtAddr::from(_start);
    let end_va = VirtAddr::from(VirtAddr::from(_start + _len).ceil());
    mmap(start_va, end_va, perm)
}

/// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap");
    let start_va = VirtAddr::from(_start);
    let end_va = VirtAddr::from(VirtAddr::from(_start + _len).ceil());
    munmap(start_va, end_va)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
