#![allow(non_camel_case_types, non_upper_case_globals)]

use crate::RawFd;

pub const SYS_fstatat: usize = 79;
pub const SYS_statx: usize = 291;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_mode: u32,
    pub st_nlink: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_rdev: u64,
    pub __pad1: u64,
    pub st_size: i64,
    pub st_blksize: i32,
    pub __pad2: i32,
    pub st_blocks: i64,
    pub st_atime_sec: i32,
    pub st_atime_nsec: u32,
    pub st_mtime_sec: i32,
    pub st_mtime_nsec: u32,
    pub st_ctime_sec: i32,
    pub st_ctime_nsec: u32,
    pub __unused4: u32,
    pub __unused5: u32,
}

impl stat {
    pub fn uninit() -> core::mem::MaybeUninit<Self> {
        let mut res = core::mem::MaybeUninit::uninit();
        let buf: &mut Self = unsafe { &mut *res.as_mut_ptr() };
        buf.__pad1 = 0;
        buf.__pad2 = 0;
        buf.__unused4 = 0;
        buf.__unused5 = 0;
        res
    }
}

#[cfg(all(not(outline_asm), target_arch = "riscv32"))]
#[inline(always)]
pub(crate) unsafe fn fstatat(dirfd: RawFd, path: *const u8, buf: *mut stat, flags: i32) -> usize {
    use core::arch::asm;

    let mut ret: usize;
    asm!(
        "ecall",
        in("a7") SYS_fstatat,
        inlateout("a0") dirfd as usize => ret,
        in("a1") path as usize,
        in("a2") buf as usize,
        in("a3") flags as usize,
        options(nostack, preserves_flags)
    );
    ret
}
