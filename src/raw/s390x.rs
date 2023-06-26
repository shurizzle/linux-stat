#![allow(non_camel_case_types, non_upper_case_globals)]

use linux_syscalls::Sysno;

use crate::Timestamp;

pub const SYS_FSTATAT: Sysno = Sysno::newfstatat;

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
    st_atime: i64,
    st_atime_nsec: u64,
    st_mtime: i64,
    st_mtime_nsec: u64,
    st_ctime: i64,
    st_ctime_nsec: u64,
    st_blksize: u64,
    st_blocks: i64,
    __pad1: [u64; 3],
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
    pub(crate) const fn raw_mode(&self) -> u16 {
        self.st_mode as u16
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
        self.st_blksize as i32
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
        core::ptr::write_bytes(
            &mut buf.__pad1[0] as *mut u32 as *mut u8,
            0,
            core::mem::size_of_val(&buf.__pad1),
        );
        res
    }
}
