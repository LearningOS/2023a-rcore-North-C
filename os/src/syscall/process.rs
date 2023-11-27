//! Process management syscalls
// use core::{mem::size_of, slice::from_raw_parts};

use crate::{
    config::MAX_SYSCALL_NUM,
    mm::{add_map_area, remove_map_area, PageTable, PhysAddr, VirtAddr, VirtPageNum},
    task::{
        change_program_brk, current_user_token, exit_current_and_run_next,
        get_current_syscall_count, get_current_task_status, get_task_start_time,
        suspend_current_and_run_next, TaskStatus,
    },
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
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

/// Convert a user virtual address to a kernel address
fn user_to_kernel<T>(user_va: *const T) -> *mut T {
    let page_table = PageTable::from_token(current_user_token());
    let user_vpn: VirtPageNum = VirtAddr(user_va as usize).floor();
    let page_off: usize = VirtAddr(user_va as usize).page_offset();
    let ppn: PhysAddr = page_table.translate(user_vpn).unwrap().ppn().into();
    let ptr: *mut T = (ppn.0 + page_off) as *mut T;
    ptr
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    let sec = get_time_us() / 1000000;
    let usec = get_time_us() % 1000000;

    let ptr = user_to_kernel(_ts);
    unsafe { *ptr = TimeVal { sec, usec } }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    let ptr = user_to_kernel(_ti);
    unsafe {
        *ptr = TaskInfo {
            status: get_current_task_status(),
            syscall_times: get_current_syscall_count(),
            time: get_time_us() / 1000 - get_task_start_time(),
        }
    }
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    let virt_start = VirtAddr::from(_start);
    let virt_end = VirtAddr::from(_start + _len);

    if !virt_start.aligned() {
        trace!("kernel: sys_mmap: virt_start is not aligned!");
        return -1;
    }

    if _port & !0x7 != 0 {
        trace!("kernel: other bits in port except 0x7 should be 0!");
        return -1;
    }

    if _port & 0x7 == 0 {
        trace!("kernel: memory is not readable or writeable!");
        return -1;
    }

    if add_map_area(virt_start, virt_end, _port) {
        return 0;
    }
    -1
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    let virt_start = VirtAddr::from(_start);
    if !virt_start.aligned() {
        trace!("kernel: sys_munmap: virt_start is not aligned!");
        return -1;
    }

    if remove_map_area(VirtAddr::from(_start), VirtAddr::from(_start + _len)) {
        return 0;
    }
    -1
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
