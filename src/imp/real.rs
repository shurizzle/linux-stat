use core::ffi::CStr;

use crate::{Errno, RawFd};

use super::sys::AT_FDCWD;

pub use super::sys::{stat as Stat, statx, StatAtFlags, StatXAttr, StatXMask, Statx};

pub fn fstatat(dirfd: RawFd, path: &CStr, flags: StatAtFlags) -> Result<Stat, Errno> {
    super::sys::fstatat(dirfd, path, flags)
}

pub fn stat(path: &CStr) -> Result<Stat, Errno> {
    fstatat(AT_FDCWD, path, StatAtFlags::empty())
}

pub fn lstat(path: &CStr) -> Result<Stat, Errno> {
    fstatat(AT_FDCWD, path, StatAtFlags::SYMLINK_NOFOLLOW)
}

pub fn fstat(fd: RawFd) -> Result<Stat, Errno> {
    if fd < 0 {
        return Err(Errno::EBADF);
    }
    fstatat(
        fd,
        unsafe { CStr::from_ptr(b"\0".as_ptr().cast()) },
        StatAtFlags::EMPTY_PATH,
    )
}

// #define major(x) \
// 	((unsigned)( (((x)>>31>>1) & 0xfffff000) | (((x)>>8) & 0x00000fff) ))
// #define minor(x) \
// 	((unsigned)( (((x)>>12) & 0xffffff00) | ((x) & 0x000000ff) ))
//
// #define makedev(x,y) ( \
//         (((x)&0xfffff000ULL) << 32) | \
// 	(((x)&0x00000fffULL) << 8) | \
//         (((y)&0xffffff00ULL) << 12) | \
// 	(((y)&0x000000ffULL)) )
