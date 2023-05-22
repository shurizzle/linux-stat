#![allow(non_camel_case_types, non_upper_case_globals)]

use crate::RawFd;

pub const SYS_fstatat: usize = 327;
pub const SYS_statx: usize = 397;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct stat {
    pub st_dev: u32,
    pub __pad0: u32,
    pub __pad1: u32,
    pub st_ino: u32,
    pub st_mode: u32,
    pub st_nlink: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_rdev: u64,
    pub __pad2: u32,
    pub st_size: i64,
    pub st_blksize: i32,
    pub st_blocks: i64,
    pub st_atime: i32,
    pub st_atime_nsec: u32,
    pub st_mtime: i32,
    pub st_mtime_nsec: u32,
    pub st_ctime: i32,
    pub st_ctime_nsec: u32,
}

impl stat {
    pub fn uninit() -> core::mem::MaybeUninit<Self> {
        let mut res = core::mem::MaybeUninit::uninit();
        let buf: &mut Self = unsafe { &mut *res.as_mut_ptr() };
        buf.__pad0 = 0;
        buf.__pad1 = 0;
        buf.__pad2 = 0;
        res
    }
}

#[cfg(all(
    not(outline_asm),
    target_arch = "arm",
    not(target_feature = "thumb-mode")
))]
#[inline(always)]
pub(crate) unsafe fn fstatat(dirfd: RawFd, path: *const u8, buf: *mut stat, flags: u32) -> usize {
    use core::arch::asm;

    let mut ret: usize;
    asm!(
        "svc 0",
        in("r7") SYS_fstatat,
        inlateout("r0") dirfd as usize => ret,
        in("r1") path as usize,
        in("r2") buf as usize,
        in("r3") flags as usize,
        options(nostack, preserves_flags)
    );
    ret
}

#[cfg(all(
    not(outline_asm),
    target_arch = "arm",
    not(target_feature = "thumb-mode")
))]
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
        "svc 0",
        in("r7") SYS_statx,
        inlateout("r0") dirfd as usize => ret,
        in("r1") path as usize,
        in("r2") flags as usize,
        in("r3") mask as usize,
        in("r4") buf as usize,
        options(nostack, preserves_flags)
    );
    ret
}

#[cfg(all(not(outline_asm), target_arch = "arm", target_feature = "thumb-mode"))]
#[inline(always)]
pub(crate) unsafe fn fstatat(dirfd: RawFd, path: *const u8, buf: *mut stat, flags: u32) -> usize {
    use core::arch::asm;

    let ret: usize;
    asm!(
        "mov {tmp}, r7",
        "mov r7, {nr}",
        "svc 0",
        "mov r7, {tmp}",
        nr = in(reg) SYS_fstatat,
        tmp = out(reg) _,
        inlateout("r0") dirfd as usize => ret,
        in("r1") path as usize,
        in("r2") buf as usize,
        in("r3") flags as usize,
        options(nostack, preserves_flags)
    );
    ret
}

#[cfg(all(not(outline_asm), target_arch = "arm", target_feature = "thumb-mode"))]
#[inline(always)]
pub(crate) unsafe fn statx(
    dirfd: RawFd,
    path: *const u8,
    flags: u32,
    mask: u32,
    buf: *mut u8,
) -> usize {
    use core::arch::asm;

    let ret: usize;
    asm!(
        "mov {tmp}, r7",
        "mov r7, {nr}",
        "svc 0",
        "mov r7, {tmp}",
        nr = in(reg) SYS_statx,
        tmp = out(reg) _,
        inlateout("r0") dirfd as usize => ret,
        in("r1") path as usize,
        in("r2") flags as usize,
        in("r3") mask as usize,
        in("r4") buf as usize,
        options(nostack, preserves_flags)
    );
    ret
}

#[cfg(all(outline_asm, target_arch = "arm"))]
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

#[cfg(all(outline_asm, target_arch = "arm"))]
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
