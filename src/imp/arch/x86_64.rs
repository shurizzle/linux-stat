#![allow(non_camel_case_types, non_upper_case_globals)]

use core::fmt;

use crate::{Mode, RawFd, Timestamp};

pub const SYS_fstatat: usize = 262;
pub const SYS_statx: usize = 332;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stat {
    st_dev: u64,
    st_ino: u64,
    st_nlink: u64,
    st_mode: u32,
    st_uid: u32,
    st_gid: u32,
    __pad0: u32,
    st_rdev: u64,
    st_size: i64,
    st_blksize: i32,
    st_blocks: i64,
    st_atime: i64,
    st_atime_nsec: i64,
    st_mtime: i64,
    st_mtime_nsec: i64,
    st_ctime: i64,
    st_ctime_nsec: i64,
    __unused: [u32; 3],
}

impl stat {
    #[inline]
    pub const fn dev(&self) -> u64 {
        self.st_dev
    }

    #[inline]
    pub const fn ino(&self) -> u64 {
        self.st_ino
    }

    #[inline]
    pub const fn nlink(&self) -> u32 {
        self.st_nlink as u32
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
        self.st_rdev
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
            secs: self.st_atime,
            nsecs: self.st_atime_nsec as u32,
        }
    }

    #[inline]
    pub const fn mtime(&self) -> Timestamp {
        Timestamp {
            secs: self.st_mtime,
            nsecs: self.st_mtime_nsec as u32,
        }
    }

    #[inline]
    pub const fn ctime(&self) -> Timestamp {
        Timestamp {
            secs: self.st_ctime,
            nsecs: self.st_ctime_nsec as u32,
        }
    }

    pub fn uninit() -> core::mem::MaybeUninit<Self> {
        let mut res = core::mem::MaybeUninit::uninit();
        let buf: &mut Self = unsafe { &mut *res.as_mut_ptr() };
        buf.__pad0 = 0;
        buf.__unused.as_mut_slice().fill(0);
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

#[cfg(all(not(outline_asm), target_arch = "x86_64"))]
#[inline(always)]
pub(crate) unsafe fn fstatat(dirfd: RawFd, path: *const i8, buf: *mut stat, flags: u32) -> usize {
    use core::arch::asm;

    let mut ret: usize;
    asm!(
        "syscall",
        inlateout("rax") SYS_fstatat => ret,
        in("rdi") dirfd as usize,
        in("rsi") path as usize,
        in("rdx") buf as usize,
        in("r10") flags as usize,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );
    ret
}

#[cfg(all(not(outline_asm), target_arch = "x86_64"))]
#[inline(always)]
pub(crate) unsafe fn statx(
    dirfd: RawFd,
    path: *const i8,
    flags: u32,
    mask: u32,
    buf: *mut u8,
) -> usize {
    use core::arch::asm;

    let mut ret: usize;
    asm!(
        "syscall",
        inlateout("rax") SYS_statx => ret,
        in("rdi") dirfd as usize,
        in("rsi") path as usize,
        in("rdx") flags as usize,
        in("r10") mask as usize,
        in("r8") buf as usize,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );
    ret
}

#[cfg(all(outline_asm, target_arch = "x86_64"))]
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

#[cfg(all(outline_asm, target_arch = "x86_64"))]
#[inline(always)]
pub(crate) unsafe fn statx(
    dirfd: RawFd,
    path: *const i8,
    flags: u32,
    mask: u32,
    buf: *mut u8,
) -> usize {
    super::__syscall5(
        SYS_fstatat,
        dirfd as usize,
        path as usize,
        flags as usize,
        mask as usize,
        buf as usize,
    )
}
