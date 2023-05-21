use core::ffi::CStr;

use crate::{Errno, RawFd};

pub use crate::arch::stat;

pub const AT_FDCWD: RawFd = -100;

#[inline]
pub fn fstatat(dirfd: RawFd, path: &CStr, flags: core::ffi::c_int) -> Result<stat, Errno> {
    let mut buf = stat::uninit();
    unsafe {
        Errno::from_ret(crate::arch::fstatat(
            dirfd,
            path.as_ptr(),
            buf.as_mut_ptr(),
            flags,
        ))?;
        Ok(buf.assume_init())
    }
}
