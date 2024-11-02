/*
 * @Author: 7erry
 * @Date: 2024-11-02 17:08:20
 * @LastEditTime: 2024-11-02 21:39:07
 * @Description: 
 */
//! Types related to task management

use super::{
    TaskContext,
    super::config::MAX_SYSCALL_NUM
};

/// The info of a task
#[derive(Copy, Clone, Debug)]
pub struct TaskInfo {
    /// The counter of syscall
    pub syscall_count: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub time: usize,
}

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The task information
    pub task_info: TaskInfo,
    /// First dispatched time, to calculate the time in the TaskInfo.
    pub first_dispatched_time: usize,
}

/// The status of a task
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
