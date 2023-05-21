use core::ffi::CStr;

use crate::{Errno, RawFd};

use super::sys::AT_FDCWD;

pub use super::sys::stat;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StatAtFlags: core::ffi::c_int {
        const EMPTY_PATH = 0x1000;
        const NO_AUTOMOUNT = 0x800;
        const SYMLINK_NOFOLLOW = 0x100;
    }
}

pub fn fstatat(dirfd: RawFd, path: &CStr, flags: StatAtFlags) -> Result<stat, Errno> {
    super::sys::fstatat(dirfd, path, flags.bits())
}

pub fn stat(path: &CStr) -> Result<stat, Errno> {
    fstatat(AT_FDCWD, path, StatAtFlags::empty())
}

pub fn lstat(path: &CStr) -> Result<stat, Errno> {
    fstatat(AT_FDCWD, path, StatAtFlags::SYMLINK_NOFOLLOW)
}

pub fn fstat(fd: RawFd) -> Result<stat, Errno> {
    if fd < 0 {
        return Err(Errno::EBADF);
    }
    fstatat(
        fd,
        unsafe { CStr::from_ptr(b"\0".as_ptr().cast()) },
        StatAtFlags::EMPTY_PATH,
    )
}

#[test]
#[ignore]
fn print_types() {
    #[cfg(not(feature = "std"))]
    macro_rules! dt {
        ($t:ty) => {
            unsafe {
                libc::printf(
                    concat!("sizeof(", stringify!($t), ") = %lu\n\0")
                        .as_ptr()
                        .cast(),
                    $t::BITS,
                );
            }
        };
    }

    #[cfg(feature = "std")]
    macro_rules! dt {
        ($t:ty) => {
            println!(
                concat!("sizeof(", stringify!($t), ") = {}"),
                core::mem::size_of::<$t>() * 8
            );
        };
    }

    dt!(core::ffi::c_short);
    dt!(core::ffi::c_ushort);
    dt!(core::ffi::c_int);
    dt!(core::ffi::c_uint);
    dt!(core::ffi::c_long);
    dt!(core::ffi::c_ulong);
    dt!(core::ffi::c_longlong);
    dt!(core::ffi::c_ulonglong);
}

#[test]
#[ignore]
#[allow(clippy::unnecessary_cast)]
fn stat_dev_null() {
    #[cfg(not(feature = "std"))]
    macro_rules! dbg {
        ($e:expr) => {
            unsafe {
                struct DelegateDebug<'a, T: core::fmt::Debug>(&'a T);
                impl<'a, T: core::fmt::Debug> core::fmt::Display for DelegateDebug<'a, T> {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        core::fmt::Debug::fmt(self.0, f)
                    }
                }

                let s = stringify!($e);
                libc::printf(b"%.*s = \0".as_ptr().cast(), s.len(), s.as_ptr());
                let v = $e;
                let mut data = alloc::string::ToString::to_string(&DelegateDebug(&v)).into_bytes();
                data.push(b'0');
                libc::puts(data.as_ptr().cast());
                v
            }
        };
    }

    let s = unsafe {
        let mut buf = core::mem::MaybeUninit::<libc::stat64>::uninit();
        assert_ne!(
            libc::fstatat64(
                libc::AT_FDCWD,
                b"/dev/null\0".as_ptr().cast(),
                buf.as_mut_ptr(),
                0,
            ),
            -1
        );
        buf.assume_init()
    };
    let res = stat(unsafe { CStr::from_ptr(b"/dev/null\0".as_ptr().cast()) });
    let res = res.unwrap();

    dbg!(&res);
    dbg!(s.st_dev);
    dbg!(s.st_ino);
    dbg!(s.st_mode);
    dbg!(s.st_nlink);
    dbg!(s.st_uid);
    dbg!(s.st_gid);
    dbg!(s.st_rdev);
    dbg!(s.st_size);
    dbg!(s.st_atime);
    dbg!(s.st_atime_nsec);
    dbg!(s.st_mtime);
    dbg!(s.st_mtime_nsec);
    dbg!(s.st_ctime);
    dbg!(s.st_ctime_nsec);
    dbg!(s.st_blksize);
    dbg!(s.st_blocks);

    // assert_eq!(s.st_dev as dev_t, res.st_dev);
    // assert_eq!(s.st_ino as ino_t, res.st_ino);
    // assert_eq!(s.st_nlink as nlink_t, res.st_nlink);
    // assert_eq!(s.st_mode as mode_t, res.st_mode);
    // assert_eq!(s.st_uid as uid_t, res.st_uid);
    // assert_eq!(s.st_gid as gid_t, res.st_gid);
    // assert_eq!(s.st_rdev as dev_t, res.st_rdev);
    // assert_eq!(s.st_size as off_t, res.st_size);
    // assert_eq!(s.st_blksize as blksize_t, res.st_blksize);
    // assert_eq!(s.st_blocks as blkcnt_t, res.st_blocks);

    // #[cfg(not(target_arch = "mips64"))]
    // {
    //     assert_eq!(s.st_atime, res.st_atime_sec);
    //     assert_eq!(s.st_atime_nsec, res.st_atime_nsec);
    //     assert_eq!(s.st_mtime, res.st_mtime_sec);
    //     assert_eq!(s.st_mtime_nsec, res.st_mtime_nsec);
    //     assert_eq!(s.st_ctime, res.st_ctime_sec);
    //     assert_eq!(s.st_ctime_nsec, res.st_ctime_nsec);
    // }
    // #[cfg(target_arch = "mips64")]
    // {
    //     assert_eq!(s.st_atime, res.st_atime_sec as i64);
    //     assert_eq!(s.st_atime_nsec, res.st_atime_nsec as i64);
    //     assert_eq!(s.st_mtime, res.st_mtime_sec as i64);
    //     assert_eq!(s.st_mtime_nsec, res.st_mtime_nsec as i64);
    //     assert_eq!(s.st_ctime, res.st_ctime_sec as i64);
    //     assert_eq!(s.st_ctime_nsec, res.st_ctime_nsec as i64);
    // }
}
