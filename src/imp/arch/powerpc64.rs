#![allow(non_camel_case_types, non_upper_case_globals)]

use crate::RawFd;

pub const SYS_fstatat: usize = 291;
pub const SYS_statx: usize = 383;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: u64,
    pub st_blocks: u64,
    pub st_atime: u64,
    pub st_atime_nsec: u64,
    pub st_mtime: u64,
    pub st_mtime_nsec: u64,
    pub st_ctime: u64,
    pub st_ctime_nsec: u64,
    pub __pad0: [u64; 3],
}

impl stat {
    pub fn uninit() -> core::mem::MaybeUninit<Self> {
        let mut res = core::mem::MaybeUninit::uninit();
        let buf: &mut Self = unsafe { &mut *res.as_mut_ptr() };
        buf.__pad0.as_mut_slice().fill(0);
        res
    }
}

#[cfg(all(not(outline_asm), target_arch = "powerpc64"))]
#[inline(always)]
pub(crate) unsafe fn fstatat(dirfd: RawFd, path: *const u8, buf: *mut stat, flags: i32) -> usize {
    use core::arch::asm;

    let mut ret: usize;
    asm!(
        "sc",
        "bns 1f",
        "neg 3, 3",
        "1:",
        inlateout("r0") SYS_fstatat => _,
        inlateout("r3") dirfd as usize => ret,
        inlateout("r4") path as usize => _,
        inlateout("r5") buf as usize => _,
        inlateout("r6") flags as usize => _,
        lateout("r7") _,
        lateout("r8") _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
        lateout("r12") _,
        lateout("cr0") _,
        options(nostack, preserves_flags)
    );
    ret
}

#[cfg(all(outline_asm, target_arch = "powerpc64"))]
#[inline(always)]
pub(crate) unsafe fn fstatat(dirfd: RawFd, path: *const u8, buf: *mut stat, flags: i32) -> usize {
    super::__syscall4(
        SYS_fstatat,
        dirfd as usize,
        path as usize,
        buf as usize,
        flags as usize,
    )
}
