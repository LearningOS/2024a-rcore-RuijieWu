//! Types related to task management

use crate::task::TaskContext;
/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task context
    pub task_cx: TaskContext,
    /// The task information
    pub task_info: TaskInfo,
    /// First dispatched time, to calculate the time in the TaskInfo.
    pub first_dispatched_time: usize,
}

#[derive(Copy, Clone, Debug)]
/// Task's information
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_count: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub time: usize,
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
