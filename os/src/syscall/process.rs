//! Process management syscalls
use crate::{
    task::{
        exit_current_and_run_next,
        suspend_current_and_run_next,
        TaskInfo,
        get_current_task,
        get_task_info
    },
    timer::get_time_us,
};

/// Doc
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    /// doc
    pub sec: usize,
    /// doc
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// Documentation
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    let id = get_current_task();
    let info = get_task_info(id);
    trace!("Current Task Info: {info:?}");
    if _ti.is_null() {
        -1
    } else {
        unsafe {
            (*_ti).time = info.time;
            (*_ti).syscall_count = info.syscall_count;
        }
        0
    }
}
