#![allow(non_camel_case_types, non_upper_case_globals)]

use crate::RawFd;

pub const SYS_fstatat: usize = 300;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct stat {
    pub st_dev: u64,
    pub __st_dev_padding: u32,
    pub __st_ino_truncated: i32,
    pub st_mode: u32,
    pub st_nlink: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_rdev: u64,
    pub __st_rdev_padding: i32,
    pub st_size: i64,
    pub st_blksize: i32,
    pub st_blocks: i64,
    pub st_atime: i32,
    pub st_atime_nsec: i32,
    pub st_mtime: i32,
    pub st_mtime_nsec: i32,
    pub st_ctime: i32,
    pub st_ctime_nsec: i32,
    pub st_ino: u64,
}

impl stat {
    pub fn uninit() -> core::mem::MaybeUninit<Self> {
        let mut res = core::mem::MaybeUninit::uninit();
        let buf: &mut Self = unsafe { &mut *res.as_mut_ptr() };
        buf.__st_dev_padding = 0;
        buf.__st_ino_truncated = 0;
        buf.__st_rdev_padding = 0;
        res
    }
}

#[cfg(all(not(outline_asm), target_arch = "x86"))]
#[inline(always)]
pub(crate) unsafe fn fstatat(dirfd: RawFd, path: *const i8, buf: *mut stat, flags: i32) -> usize {
    use core::arch::asm;

    let mut ret: usize;

    asm!(
        "xchg esi, {arg4}",
        "int 0x80",
        "xchg esi, {arg4}",
        arg4 = in(reg) flags as usize,
        inlateout("eax") SYS_fstatat => ret,
        in("ebx") dirfd as usize,
        in("ecx") path as usize,
        in("edx") buf as usize,
        options(nostack, preserves_flags)
    );
    ret
}

#[cfg(all(outline_asm, target_arch = "x86"))]
#[inline(always)]
pub(crate) unsafe fn fstatat(dirfd: RawFd, path: *const i8, buf: *mut stat, flags: i32) -> usize {
    super::__syscall4(
        SYS_fstatat,
        dirfd as usize,
        path as usize,
        buf as usize,
        flags as usize,
    )
}
