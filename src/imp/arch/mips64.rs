#![allow(non_camel_case_types, non_upper_case_globals)]

use crate::RawFd;

pub const SYS_fstatat: usize = 5252;
pub const SYS_statx: usize = 5326;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct stat {
    pub st_dev: u32,
    pub __pad0: [u32; 3],
    pub st_ino: u64,
    pub st_mode: u32,
    pub st_nlink: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_rdev: u32,
    pub __pad1: [u32; 3],
    pub st_size: i64,
    pub st_atime_sec: u32,
    pub st_atime_nsec: u32,
    pub st_mtime_sec: u32,
    pub st_mtime_nsec: u32,
    pub st_ctime_sec: u32,
    pub st_ctime_nsec: u32,
    pub st_blksize: u32,
    pub __pad2: u32,
    pub st_blocks: i64,
}

impl stat {
    pub fn uninit() -> core::mem::MaybeUninit<Self> {
        let mut res = core::mem::MaybeUninit::uninit();
        let buf: &mut Self = unsafe { &mut *res.as_mut_ptr() };
        buf.__pad0.as_mut_slice().fill(0);
        buf.__pad1.as_mut_slice().fill(0);
        buf.__pad2 = 0;
        res
    }
}

#[cfg(all(not(outline_asm), target_arch = "mips64"))]
#[inline(always)]
pub(crate) unsafe fn fstatat(dirfd: RawFd, path: *const i8, buf: *mut stat, flags: u32) -> usize {
    use core::arch::asm;

    let mut err: usize;
    let mut ret: usize;
    asm!(
        "syscall",
        inlateout("$2") SYS_fstatat => ret,
        in("$4") dirfd as usize,
        in("$5") path as usize,
        in("$6") buf as usize,
        inlateout("$7") flags as usize => err,
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(nostack, preserves_flags)
    );
    if err == 0 {
        ret
    } else {
        ret.wrapping_neg()
    }
}

#[cfg(all(not(outline_asm), target_arch = "mips64"))]
#[inline(always)]
pub(crate) unsafe fn statx(
    dirfd: RawFd,
    path: *const i8,
    flags: u32,
    mask: u32,
    buf: *mut u8,
) -> usize {
    use core::arch::asm;

    let mut err: usize;
    let mut ret: usize;
    asm!(
        "syscall",
        inlateout("$2") SYS_statx => ret,
        in("$4") dirfd as usize,
        in("$5") path as usize,
        in("$6") flags as usize,
        // $7 is now used for both input and output.
        inlateout("$7") mask as usize => err,
        inlateout("$8") buf as usize => _,
        // All temporary registers are always clobbered
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(nostack, preserves_flags)
    );
    if err == 0 {
        ret
    } else {
        ret.wrapping_neg()
    }
}

#[cfg(all(outline_asm, target_arch = "mips64"))]
#[inline(always)]
pub(crate) unsafe fn fstatat(dirfd: RawFd, path: *const i8, buf: *mut stat, flags: u32) -> usize {
    super::__syscall4(
        SYS_fstatat,
        dirfd as usize,
        path as usize,
        buf as usize,
        flags as usize,
    )
}

#[cfg(all(outline_asm, target_arch = "mips64"))]
#[inline(always)]
pub(crate) unsafe fn statx(
    dirfd: RawFd,
    path: *const i8,
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
