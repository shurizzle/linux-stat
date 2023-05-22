#![allow(non_camel_case_types, non_upper_case_globals)]

use core::fmt;

use crate::{Mode, RawFd, Timestamp};

pub const SYS_fstatat: usize = 5252;
pub const SYS_statx: usize = 5326;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stat {
    st_dev: u32,
    __pad0: [u32; 3],
    st_ino: u64,
    st_mode: u32,
    st_nlink: u32,
    st_uid: u32,
    st_gid: u32,
    st_rdev: u32,
    __pad1: [u32; 3],
    st_size: i64,
    st_atime: i32,
    st_atime_nsec: u32,
    st_mtime: i32,
    st_mtime_nsec: u32,
    st_ctime: i32,
    st_ctime_nsec: u32,
    st_blksize: i32,
    __pad2: u32,
    st_blocks: i64,
}

impl stat {
    #[inline]
    pub const fn dev(&self) -> u64 {
        self.st_dev as u64
    }

    #[inline]
    pub const fn ino(&self) -> u64 {
        self.st_ino
    }

    #[inline]
    pub const fn nlink(&self) -> u32 {
        self.st_nlink
    }

    #[inline]
    pub const fn mode(&self) -> Mode {
        Mode(self.st_mode as u16)
    }

    #[inline]
    pub const fn uid(&self) -> u32 {
        self.st_uid
    }

    #[inline]
    pub const fn gid(&self) -> u32 {
        self.st_gid
    }

    #[inline]
    pub const fn rdev(&self) -> u64 {
        self.st_rdev as u64
    }

    #[inline]
    pub const fn size(&self) -> i64 {
        self.st_size
    }

    #[inline]
    pub const fn block_size(&self) -> i32 {
        self.st_blksize
    }

    #[inline]
    pub const fn blocks(&self) -> i64 {
        self.st_blocks
    }

    #[inline]
    pub const fn atime(&self) -> Timestamp {
        Timestamp {
            secs: self.st_atime as i64,
            nsecs: self.st_atime_nsec,
        }
    }

    #[inline]
    pub const fn mtime(&self) -> Timestamp {
        Timestamp {
            secs: self.st_mtime as i64,
            nsecs: self.st_mtime_nsec,
        }
    }

    #[inline]
    pub const fn ctime(&self) -> Timestamp {
        Timestamp {
            secs: self.st_ctime as i64,
            nsecs: self.st_ctime_nsec,
        }
    }

    pub fn uninit() -> core::mem::MaybeUninit<Self> {
        let mut res = core::mem::MaybeUninit::uninit();
        let buf: &mut Self = unsafe { &mut *res.as_mut_ptr() };
        buf.__pad0.as_mut_slice().fill(0);
        buf.__pad1.as_mut_slice().fill(0);
        buf.__pad2 = 0;
        res
    }
}

impl fmt::Debug for stat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("stat")
            .field("dev", &self.dev())
            .field("ino", &self.ino())
            .field("nlink", &self.nlink())
            .field("mode", &self.mode())
            .field("uid", &self.uid())
            .field("gid", &self.gid())
            .field("rdev", &self.rdev())
            .field("size", &self.size())
            .field("block_size", &self.block_size())
            .field("blocks", &self.blocks())
            .field("atime", &self.atime())
            .field("mtime", &self.mtime())
            .field("ctime", &self.ctime())
            .finish()
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
