#![cfg(target_os = "linux")]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
pub(crate) use std::os::unix::io::RawFd;
#[cfg(not(feature = "std"))]
pub type RawFd = cty::c_int;

#[cfg(not(extern_cstr))]
pub use core::ffi::CStr;
#[cfg(extern_cstr)]
pub use cstr_core::CStr;

#[cfg(feature = "std")]
pub use std::path::Path;
#[cfg(not(feature = "std"))]
pub use CStr as Path;

pub use linux_syscalls::Errno;

pub mod raw;

use core::fmt;

use linux_syscalls::bitflags;

pub const CURRENT_DIRECTORY: RawFd = -100;

bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum StatAtFlags: u32 {
        EMPTY_PATH = 0x1000,
        NO_AUTOMOUNT = 0x800,
        SYMLINK_NOFOLLOW = 0x100,
    }
}

bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum ModePermission : u8 {
        READ = 0o4,
        WRITE = 0o2,
        EXEC = 0o1,
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mode(pub(crate) u16);

impl Mode {
    #[inline]
    pub const fn owner(&self) -> ModePermission {
        ModePermission::from_bits(((self.0 >> 6) & 0o7) as u8)
    }

    #[inline]
    pub const fn group(&self) -> ModePermission {
        ModePermission::from_bits(((self.0 >> 3) & 0o7) as u8)
    }

    #[inline]
    pub const fn other(&self) -> ModePermission {
        ModePermission::from_bits((self.0 & 0o7) as u8)
    }

    #[inline]
    pub const fn suid(&self) -> bool {
        self.0 & 0o4000 == 0o4000
    }

    #[inline]
    pub const fn sgid(&self) -> bool {
        self.0 & 0o2000 == 0o2000
    }

    #[inline]
    pub const fn svtx(&self) -> bool {
        self.0 & 0o1000 == 0o1000
    }

    #[inline]
    pub const fn from_u16(value: u16) -> Self {
        Self(value)
    }

    #[inline]
    pub const fn as_u16(&self) -> u16 {
        self.0
    }
}

impl From<u16> for Mode {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl fmt::Debug for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn format_c(f: &mut fmt::Formatter<'_>, b: bool, c: char) -> fmt::Result {
            if b {
                fmt::Display::fmt(&c, f)
            } else {
                fmt::Display::fmt(&'-', f)
            }
        }

        fn format_perm(
            f: &mut fmt::Formatter<'_>,
            p: ModePermission,
            x: Option<char>,
        ) -> fmt::Result {
            format_c(f, p.contains(ModePermission::READ), 'r')?;
            format_c(f, p.contains(ModePermission::WRITE), 'w')?;
            if let Some(x) = x {
                fmt::Display::fmt(&x, f)
            } else {
                format_c(f, p.contains(ModePermission::EXEC), 'x')
            }
        }

        write!(f, "Mode(")?;
        format_perm(f, self.owner(), if self.suid() { Some('s') } else { None })?;
        format_perm(f, self.group(), if self.sgid() { Some('s') } else { None })?;
        format_perm(f, self.other(), None)?;
        write!(f, ")")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp {
    secs: i64,
    nsecs: u32,
}

impl Timestamp {
    #[inline]
    pub const fn secs(&self) -> i64 {
        self.secs
    }

    #[inline]
    pub const fn seconds(&self) -> i64 {
        self.secs
    }

    #[inline]
    pub const fn nsecs(&self) -> u32 {
        self.nsecs
    }

    #[inline]
    pub const fn nanosecs(&self) -> u32 {
        self.nsecs
    }

    #[inline]
    pub const fn nanoseconds(&self) -> u32 {
        self.nsecs
    }
}

#[cfg(all(not(feature = "linux_4_11"), not(target_arch = "loongarch64")))]
static mut HAS_STATX: core::sync::atomic::AtomicU8 = core::sync::atomic::AtomicU8::new(2);

#[cfg(any(feature = "linux_4_11", target_arch = "loongarch64"))]
pub type Stat = crate::raw::Statx;

#[cfg(all(not(feature = "linux_4_11"), not(target_arch = "loongarch64")))]
#[derive(Clone, Copy)]
pub enum Stat {
    Stat64(crate::raw::stat),
    Statx(crate::raw::Statx),
}

#[cfg(all(not(feature = "linux_4_11"), not(target_arch = "loongarch64")))]
macro_rules! with_stat {
    ($outer:expr, |$name:ident| $($tt:tt)+) => {
        match $outer {
            $crate::Stat::Stat64($name) => $($tt)+,
            $crate::Stat::Statx($name) => $($tt)+,
        }
    };
}

#[cfg(all(not(feature = "linux_4_11"), not(target_arch = "loongarch64")))]
impl Stat {
    #[inline]
    pub const fn block_size(&self) -> i32 {
        with_stat!(self, |s| s.block_size())
    }

    #[inline]
    pub const fn nlink(&self) -> u32 {
        with_stat!(self, |s| s.nlink())
    }

    #[inline]
    pub const fn uid(&self) -> u32 {
        with_stat!(self, |s| s.uid())
    }

    #[inline]
    pub const fn gid(&self) -> u32 {
        with_stat!(self, |s| s.gid())
    }

    #[inline]
    pub const fn mode(&self) -> Mode {
        with_stat!(self, |s| s.mode())
    }

    #[inline]
    pub const fn ino(&self) -> u64 {
        with_stat!(self, |s| s.ino())
    }

    #[inline]
    pub const fn size(&self) -> i64 {
        with_stat!(self, |s| s.size())
    }

    #[inline]
    pub const fn blocks(&self) -> i64 {
        with_stat!(self, |s| s.blocks())
    }

    #[inline]
    pub const fn atime(&self) -> Timestamp {
        with_stat!(self, |s| s.atime())
    }

    #[inline]
    pub const fn ctime(&self) -> Timestamp {
        with_stat!(self, |s| s.ctime())
    }

    #[inline]
    pub const fn mtime(&self) -> Timestamp {
        with_stat!(self, |s| s.mtime())
    }

    #[inline]
    pub const fn rdev_major(&self) -> u32 {
        with_stat!(self, |s| s.rdev_major())
    }

    #[inline]
    pub const fn rdev_minor(&self) -> u32 {
        with_stat!(self, |s| s.rdev_minor())
    }

    #[inline]
    pub const fn rdev(&self) -> u64 {
        with_stat!(self, |s| s.rdev())
    }

    #[inline]
    pub const fn dev_major(&self) -> u32 {
        with_stat!(self, |s| s.dev_major())
    }

    #[inline]
    pub const fn dev_minor(&self) -> u32 {
        with_stat!(self, |s| s.dev_minor())
    }

    #[inline]
    pub const fn dev(&self) -> u64 {
        with_stat!(self, |s| s.dev())
    }
}

#[cfg(all(not(feature = "linux_4_11"), not(target_arch = "loongarch64")))]
impl fmt::Debug for Stat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        with_stat!(self, |s| s.debug(f, "Stat"))
    }
}

#[inline]
pub fn empty_path() -> &'static Path {
    #[cfg(feature = "std")]
    let empty: &Path = Path::new("");
    #[cfg(not(feature = "std"))]
    let empty: &Path = unsafe { CStr::from_bytes_with_nul_unchecked(b"\0") };
    empty
}

#[cfg(feature = "std")]
#[inline(always)]
pub(crate) fn run_with_cstr<P, T, F>(path: P, f: F) -> Result<T, Errno>
where
    P: AsRef<Path>,
    F: FnOnce(&CStr) -> Result<T, Errno>,
{
    use core::mem::MaybeUninit;
    #[cfg(extern_cstr)]
    use cstr_core::CString;
    #[cfg(not(extern_cstr))]
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;

    #[cfg(not(target_os = "espidf"))]
    const MAX_STACK_ALLOCATION: usize = 384;
    #[cfg(target_os = "espidf")]
    const MAX_STACK_ALLOCATION: usize = 32;

    let path = path.as_ref().as_os_str().as_bytes();

    if path.is_empty() {
        return f(unsafe { CStr::from_bytes_with_nul_unchecked(b"\0") });
    }

    if path.last().map(|&c| c == 0).unwrap_or(false) {
        return f(CStr::from_bytes_with_nul(path).map_err(|_| Errno::ENOENT)?);
    }

    if path.len() >= MAX_STACK_ALLOCATION {
        return CString::new(path).map_or(Err(Errno::ENOENT), |path| f(&path));
    }

    let mut buf = MaybeUninit::<[u8; MAX_STACK_ALLOCATION]>::uninit();
    let buf_ptr = buf.as_mut_ptr() as *mut u8;

    unsafe {
        core::ptr::copy_nonoverlapping(path.as_ptr(), buf_ptr, path.len());
        buf_ptr.add(path.len()).write(0);
    }

    CStr::from_bytes_with_nul(unsafe { core::slice::from_raw_parts(buf_ptr, path.len() + 1) })
        .map_or(Err(Errno::ENOENT), f)
}

#[cfg(not(feature = "std"))]
#[inline(always)]
pub(crate) fn run_with_cstr<P, T, F>(path: P, f: F) -> Result<T, Errno>
where
    P: AsRef<Path>,
    F: FnOnce(&CStr) -> Result<T, Errno>,
{
    f(path.as_ref())
}

#[cfg(all(not(feature = "linux_4_11"), not(target_arch = "loongarch64")))]
pub unsafe fn fstatat<P: AsRef<Path>>(
    dirfd: RawFd,
    path: P,
    flags: StatAtFlags,
) -> Result<Stat, Errno> {
    let path = path.as_ref();

    use core::sync::atomic::Ordering;

    match HAS_STATX.load(Ordering::Relaxed) {
        0 => crate::raw::fstatat(dirfd, path, flags).map(Stat::Stat64),
        1 => crate::raw::statx(dirfd, path, flags, crate::raw::StatXMask::empty()).map(Stat::Statx),
        _ => match crate::raw::statx(dirfd, path, flags, crate::raw::StatXMask::empty()) {
            Err(Errno::ENOSYS) => {
                HAS_STATX.store(0, Ordering::Relaxed);
                crate::raw::fstatat(dirfd, path, flags).map(Stat::Stat64)
            }
            other => {
                HAS_STATX.store(1, Ordering::Relaxed);
                other.map(Stat::Statx)
            }
        },
    }
}

#[cfg(any(feature = "linux_4_11", target_arch = "loongarch64"))]
#[inline]
pub fn fstatat<P: AsRef<Path>>(dirfd: RawFd, path: P, flags: StatAtFlags) -> Result<Stat, Errno> {
    raw::statx(dirfd, path, flags, crate::raw::StatXMask::empty())
}

#[inline]
pub fn stat<P: AsRef<Path>>(path: P) -> Result<Stat, Errno> {
    unsafe { fstatat(CURRENT_DIRECTORY, path, StatAtFlags::empty()) }
}

#[inline]
pub fn lstat<P: AsRef<Path>>(path: P) -> Result<Stat, Errno> {
    unsafe { fstatat(CURRENT_DIRECTORY, path, StatAtFlags::SYMLINK_NOFOLLOW) }
}

#[inline]
pub unsafe fn fstat(fd: RawFd) -> Result<Stat, Errno> {
    if fd < 0 {
        return Err(Errno::EBADF);
    }

    fstatat(fd, empty_path(), StatAtFlags::EMPTY_PATH)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[cfg(feature = "std")]
    pub fn dev_null() -> &'static Path {
        Path::new("/dev/null\0")
    }

    #[cfg(not(feature = "std"))]
    pub fn dev_null() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(b"/dev/null\0") }
    }

    pub fn retry<T, F: Fn() -> Result<T, Errno>>(f: F) -> Result<T, Errno> {
        loop {
            match f() {
                Err(Errno::EINTR) => (),
                other => return other,
            }
        }
    }

    pub fn c_stat() -> Result<libc::stat64, Errno> {
        unsafe {
            let mut buf = core::mem::MaybeUninit::<libc::stat64>::uninit();
            if libc::fstatat64(
                libc::AT_FDCWD,
                b"/dev/null\0".as_ptr() as *const _,
                buf.as_mut_ptr(),
                0,
            ) == -1
            {
                return Err(Errno::new(*libc::__errno_location()));
            }
            Ok(buf.assume_init())
        }
    }

    #[test]
    #[allow(clippy::unnecessary_cast)]
    fn stat_dev_null() {
        linux_syscalls::init();

        let c_stat = retry(c_stat);
        assert!(c_stat.is_ok());
        let c_stat = c_stat.unwrap();

        let stat = retry(|| unsafe { stat(dev_null()) });
        assert!(stat.is_ok());
        let stat = stat.unwrap();

        assert_eq!(stat.dev(), c_stat.st_dev as u64);
        assert_eq!(stat.ino(), c_stat.st_ino as u64);
        assert_eq!(stat.nlink(), c_stat.st_nlink as u32);
        assert_eq!(stat.mode().as_u16(), c_stat.st_mode as u16);
        assert_eq!(stat.uid(), c_stat.st_uid as u32);
        assert_eq!(stat.gid(), c_stat.st_gid as u32);
        assert_eq!(stat.rdev(), c_stat.st_rdev as u64);
        assert_eq!(stat.size(), c_stat.st_size as i64);
        assert_eq!(stat.block_size(), c_stat.st_blksize as i32);
        assert_eq!(stat.blocks(), c_stat.st_blocks as i64);
        assert_eq!(stat.atime().secs, c_stat.st_atime as i64);
        assert_eq!(stat.atime().nsecs, c_stat.st_atime_nsec as u32);
        assert_eq!(stat.mtime().secs, c_stat.st_mtime as i64);
        assert_eq!(stat.mtime().nsecs, c_stat.st_mtime_nsec as u32);
        assert_eq!(stat.ctime().secs, c_stat.st_ctime as i64);
        assert_eq!(stat.ctime().nsecs, c_stat.st_ctime_nsec as u32);
    }
}
