#![allow(non_camel_case_types, non_upper_case_globals)]

use linux_syscalls::Sysno;

use crate::Timestamp;

pub const SYS_FSTATAT: Sysno = Sysno::fstatat64;

/// `fstatat()` file informations representation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stat {
    st_dev: u32,
    __pad0: [u32; 2],
    st_ino: u32,
    st_mode: u32,
    st_nlink: u32,
    st_uid: u32,
    st_gid: u32,
    st_rdev: u64,
    __pad1: u32,
    st_size: i64,
    st_blksize: i32,
    st_blocks: i64,
    st_atime: i32,
    st_atime_nsec: u32,
    st_mtime: i32,
    st_mtime_nsec: u32,
    st_ctime: i32,
    st_ctime_nsec: u32,
}

impl stat {
    /// Returns the device on which this file (inode) resides.
    #[inline]
    pub const fn dev(&self) -> u64 {
        self.st_dev as u64
    }

    /// Returns the inode number of the file.
    #[inline]
    pub const fn inode(&self) -> u64 {
        self.st_ino as u64
    }

    /// Returns the number of hard links on a file.
    #[inline]
    pub const fn nlink(&self) -> u32 {
        self.st_nlink
    }

    #[inline]
    pub(crate) const fn raw_mode(&self) -> u16 {
        self.st_mode
    }

    /// Returns the user ID of the owner of the file.
    #[inline]
    pub const fn uid(&self) -> u32 {
        self.st_uid
    }

    /// Returns the ID of the group owner of the file.
    #[inline]
    pub const fn gid(&self) -> u32 {
        self.st_gid
    }

    /// Returns the device that this file (inode) represents if the file is of
    /// block or character device type
    #[inline]
    pub const fn rdev(&self) -> u64 {
        self.st_rdev
    }

    /// Returns the size of the file (if it is a regular file or a symbolic
    /// link) in bytes. The size of a symbolic link is the length of
    /// the pathname it contains, without a terminating null byte.
    #[inline]
    pub const fn size(&self) -> i64 {
        self.st_size
    }

    /// Returns the "preferred" block size for efficient filesystem I/O.
    /// (Writing to a file in smaller chunks may cause an inefficient
    /// read-modify-rewrite.)
    #[inline]
    pub const fn block_size(&self) -> i32 {
        self.st_blksize
    }

    /// Returns the number of blocks allocated to the file on the medium, in
    /// 512-byte units. (This may be smaller than stx_size/512 when the
    /// file has holes.)
    #[inline]
    pub const fn blocks(&self) -> i64 {
        self.st_blocks
    }

    /// Returns the file's last access timestamp.
    #[inline]
    pub const fn atime(&self) -> Timestamp {
        Timestamp {
            secs: self.st_atime as i64,
            nsecs: self.st_atime_nsec,
        }
    }

    /// Returns the file's last modification timestamp.
    #[inline]
    pub const fn mtime(&self) -> Timestamp {
        Timestamp {
            secs: self.st_mtime as i64,
            nsecs: self.st_mtime_nsec,
        }
    }

    /// Returns the file's last status change timestamp.
    #[inline]
    pub const fn ctime(&self) -> Timestamp {
        Timestamp {
            secs: self.st_ctime as i64,
            nsecs: self.st_ctime_nsec,
        }
    }

    #[doc(hidden)]
    pub fn uninit() -> core::mem::MaybeUninit<Self> {
        let mut res = core::mem::MaybeUninit::uninit();
        let buf: &mut Self = unsafe { &mut *res.as_mut_ptr() };
        core::ptr::write_bytes(
            &mut buf.__pad0[0] as *mut u32 as *mut u8,
            0,
            core::mem::size_of_val(&buf.__pad0),
        );
        buf.__pad1 = 0;
        res
    }
}
