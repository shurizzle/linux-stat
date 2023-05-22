#![allow(non_camel_case_types, non_upper_case_globals)]

use crate::RawFd;

pub const SYS_fstatat: usize = 293;
pub const SYS_statx: usize = 379;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct stat {
    pub st_dev: u64,
    pub __st_dev_padding: u16,
    pub __st_ino_truncated: core::ffi::c_long,
    pub st_mode: u32,
    pub st_nlink: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_rdev: u64,
    pub __st_rdev_padding: u16,
    pub st_size: i64,
    pub st_blksize: i32,
    pub st_blocks: i64,
    pub st_atime: core::ffi::c_long,
    pub st_atime_nsec: core::ffi::c_long,
    pub st_mtime: core::ffi::c_long,
    pub st_mtime_nsec: core::ffi::c_long,
    pub st_ctime: core::ffi::c_long,
    pub st_ctime_nsec: core::ffi::c_long,
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

#[cfg(all(not(outline_asm), target_arch = "m68k"))]
#[inline(always)]
pub(crate) unsafe fn fstatat(dirfd: RawFd, path: *const u8, buf: *mut stat, flags: u32) -> usize {
    use core::arch::asm;

    let mut ret: usize;
    asm!(
        "trap #0",
        inlaeout("d0") SYS_fstatat => ret,
        in("d1") dirfd as usize,
        in("d2") path as usize,
        in("d3") buf as usize,
        in("d4") flags as usize,
        options(nostack, preserves_flags)
    );
    ret
}

#[cfg(all(not(outline_asm), target_arch = "m68k"))]
#[inline(always)]
pub(crate) unsafe fn statx(
    dirfd: RawFd,
    path: *const u8,
    flags: u32,
    mask: u32,
    buf: *mut u8,
) -> usize {
    use core::arch::asm;

    let mut ret: usize;
    asm!(
        "trap #0",
        inlaeout("d0") SYS_statx => ret,
        in("d1") dirfd as usize,
        in("d2") path as usize,
        in("d3") flags as usize,
        in("d4") mask as usize,
        in("d5") buf as usize,
        options(nostack, preserves_flags)
    );
    ret
}

#[cfg(all(outline_asm, target_arch = "m68k"))]
#[inline(always)]
pub(crate) unsafe fn fstatat(dirfd: RawFd, path: *const u8, buf: *mut stat, flags: u32) -> usize {
    super::__syscall4(
        SYS_fstatat,
        dirfd as usize,
        path as usize,
        buf as usize,
        flags as usize,
    )
}

#[cfg(all(outline_asm, target_arch = "m68k"))]
#[inline(always)]
pub(crate) unsafe fn statx(
    dirfd: RawFd,
    path: *const u8,
    flags: u32,
    mask: u32,
    buf: *mut u8,
) -> usize {
    super::__syscall5(
        SYS_statx,
        dirfd as usize,
        path as usize,
        flags as usize,
        mask as usize,
        buf as usize,
    )
}
