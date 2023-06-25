use linux_syscalls::Errno;

use crate::RawFd;

use super::sys::AT_FDCWD;

pub use super::sys::{stat as Stat, statx, StatAtFlags, StatXAttr, StatXMask, Statx};

pub fn fstatat(dirfd: RawFd, path: &[u8], flags: StatAtFlags) -> Result<Stat, Errno> {
    super::sys::fstatat(dirfd, path, flags)
}

pub fn stat(path: &[u8]) -> Result<Stat, Errno> {
    fstatat(AT_FDCWD, path, StatAtFlags::empty())
}

pub fn lstat(path: &[u8]) -> Result<Stat, Errno> {
    fstatat(AT_FDCWD, path, StatAtFlags::SYMLINK_NOFOLLOW)
}

pub fn fstat(fd: RawFd) -> Result<Stat, Errno> {
    if fd < 0 {
        return Err(Errno::EBADF);
    }
    fstatat(fd, b"\0", StatAtFlags::EMPTY_PATH)
}
