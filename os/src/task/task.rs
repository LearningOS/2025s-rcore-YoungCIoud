//! Types related to task management

use crate::syscall::{SYSCALL_EXIT, SYSCALL_GET_TIME, SYSCALL_TRACE, SYSCALL_WRITE, SYSCALL_YIELD};

use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The syscall time
    pub task_syscall_time: TaskSyscallTime,
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
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

/// specific syscall of a task 
#[derive(Copy, Clone)]
pub struct TaskSyscallTime {
    /// write time
    pub write: usize,
    /// exit time
    pub exit: usize,
    /// yield time
    pub yield_: usize,
    /// gettime time
    pub gettime: usize,
    /// trace time
    pub trace: usize,
}

impl TaskSyscallTime {
    /// construct a new instance of TaskSyscallTime
    /// initially all zero
    pub fn new() -> Self {
        Self {
            write: 0,
            exit: 0,
            yield_: 0,
            gettime: 0,
            trace: 0,
        }
    }

    // get a mutable reference of the item specified by id
    fn get_mut_item(&mut self, id: usize) -> Option<&mut usize> {
        match id {
            SYSCALL_WRITE => Some(&mut self.write),
            SYSCALL_EXIT => Some(&mut self.exit),
            SYSCALL_YIELD => Some(&mut self.yield_),
            SYSCALL_GET_TIME => Some(&mut self.gettime),
            SYSCALL_TRACE => Some(&mut self.trace),
            _ => None,
        }
    }

    fn get_item(&self, id: usize) -> Option<&usize> {
        match id {
            SYSCALL_WRITE => Some(&self.write),
            SYSCALL_EXIT => Some(&self.exit),
            SYSCALL_YIELD => Some(&self.yield_),
            SYSCALL_GET_TIME => Some(&self.gettime),
            SYSCALL_TRACE => Some(&self.trace),
            _ => None,
        }
    }

    /// add one to the item specified by id
    /// return -1 when the id is invalid, otherwise 0
    pub fn add_one(&mut self, id: usize) -> isize {
        if let Some(item) = self.get_mut_item(id) {
            *item += 1;
            0
        } else {
            -1
        }
    }

    /// get val of the item specified by id
    /// return -1 when the id is invalid, otherwise the val
    pub fn ask(&self, id: usize) -> isize {
        if let Some(item) = self.get_item(id) {
            *item as isize
        } else {
            -1
        }
    }
}