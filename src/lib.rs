#![cfg(target_os = "linux")]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
#[doc(no_inline)]
pub use std::os::unix::io::RawFd;
#[cfg(not(feature = "std"))]
/// Raw file descriptor.
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

/// Special file descriptor that represent the current directory.
pub const CURRENT_DIRECTORY: RawFd = -100;

bitflags! {
    /// Flags for `fstatat()`.
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum StatAtFlags: u32 {
        /// If pathname is an empty string, operate on the file referred to
        /// by dirfd (which may have been obtained using the open(2) O_PATH
        /// flag).  In this case, dirfd can refer to any type of file, not
        /// just a directory, and the behavior of fstatat() is similar to
        /// that of fstat().  If dirfd is AT_FDCWD, the call operates on the
        /// current working directory.
        EMPTY_PATH = 0x1000,
        /// Don't automount the terminal ("basename") component of pathname.
        /// Since Linux 3.1 this flag is ignored.  Since Linux 4.11 this
        /// flag is implied.
        NO_AUTOMOUNT = 0x800,
        /// If pathname is a symbolic link, do not dereference it: instead
        /// return information about the link itself, like lstat().  (By
        /// default, fstatat() dereferences symbolic links, like stat().)
        SYMLINK_NOFOLLOW = 0x100,
    }
}

bitflags! {
    /// Entity (owner, group or other) permissions representation.
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum ModePermission : u8 {
        /// Read permission.
        READ = 0o4,
        /// Write permission.
        WRITE = 0o2,
        /// Exec permission.
        EXEC = 0o1,
    }
}

/// File permissions representation.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mode(pub(crate) u16);

impl Mode {
    /// Returns owner's file permissions.
    #[inline]
    pub const fn owner(&self) -> ModePermission {
        ModePermission::from_bits(((self.0 >> 6) & 0o7) as u8)
    }

    /// Returns group's file permissions.
    #[inline]
    pub const fn group(&self) -> ModePermission {
        ModePermission::from_bits(((self.0 >> 3) & 0o7) as u8)
    }

    /// Returns other's file permissions.
    #[inline]
    pub const fn other(&self) -> ModePermission {
        ModePermission::from_bits((self.0 & 0o7) as u8)
    }

    /// Returns true if suid is set on file.
    #[inline]
    pub const fn suid(&self) -> bool {
        self.0 & 0o4000 == 0o4000
    }

    /// Returns true if sgid is set on file.
    #[inline]
    pub const fn sgid(&self) -> bool {
        self.0 & 0o2000 == 0o2000
    }

    /// Returns true if svtx is set on file.
    #[inline]
    pub const fn svtx(&self) -> bool {
        self.0 & 0o1000 == 0o1000
    }

    /// Returns [Mode] from a u16.
    #[inline]
    pub const fn from_u16(value: u16) -> Self {
        Self(value)
    }

    /// Returns the underlining u16.
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

/// Timestamp representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp {
    secs: i64,
    nsecs: u32,
}

impl Timestamp {
    /// Returns the seconds part.
    #[inline]
    pub const fn seconds(&self) -> i64 {
        self.secs
    }

    /// Alias for `Self::seconds()`
    #[inline]
    pub const fn secs(&self) -> i64 {
        self.secs
    }

    /// Returns the nanoseconds part.
    #[inline]
    pub const fn nanoseconds(&self) -> u32 {
        self.nsecs
    }

    /// Alias for `Self::nanoseconds()`
    #[inline]
    pub const fn nanosecs(&self) -> u32 {
        self.nsecs
    }

    /// Alias for `Self::nanoseconds()`
    #[inline]
    pub const fn nsecs(&self) -> u32 {
        self.nsecs
    }
}

/// File type representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FileType {
    /// Block file type.
    Block,
    /// Character file type.
    Character,
    /// Directory file type.
    Directory,
    /// FIFO file type.
    Fifo,
    /// Link file type.
    Link,
    /// Regular file type.
    Regular,
    /// Socket file type.
    Socket,
    /// Unknown file type.
    Unknown,
}

impl FileType {
    pub fn as_u16(&self) -> u16 {
        match *self {
            FileType::Socket => 0o140000,
            FileType::Link => 0o120000,
            FileType::Regular => 0o100000,
            FileType::Block => 0o060000,
            FileType::Directory => 0o040000,
            FileType::Character => 0o020000,
            FileType::Fifo => 0o010000,
            FileType::Unknown => 0o000000,
        }
    }
}

#[cfg(all(not(feature = "linux_4_11"), not(target_arch = "loongarch64")))]
static mut HAS_STATX: core::sync::atomic::AtomicU8 = core::sync::atomic::AtomicU8::new(2);

/// Unified Stat structure.
#[cfg(any(feature = "linux_4_11", target_arch = "loongarch64"))]
pub type Stat = crate::raw::Statx;

/// Unified Stat structure.
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
    /// Returns the "preferred" block size for efficient filesystem I/O.
    /// (Writing to a file in smaller chunks may cause an inefficient
    /// read-modify-rewrite.)
    #[inline]
    pub const fn block_size(&self) -> i32 {
        with_stat!(self, |s| s.block_size())
    }

    /// Returns the number of hard links on a file.
    #[inline]
    pub const fn nlink(&self) -> u32 {
        with_stat!(self, |s| s.nlink())
    }

    /// Returns the user ID of the owner of the file.
    #[inline]
    pub const fn uid(&self) -> u32 {
        with_stat!(self, |s| s.uid())
    }

    /// Returns the ID of the group owner of the file.
    #[inline]
    pub const fn gid(&self) -> u32 {
        with_stat!(self, |s| s.gid())
    }

    /// Returns the file mode.
    #[inline]
    pub const fn mode(&self) -> Mode {
        with_stat!(self, |s| s.mode())
    }

    /// Returns the file type.
    pub const fn file_type(&self) -> FileType {
        with_stat!(self, |s| s.file_type())
    }

    /// Returns true if file type is socket.
    #[inline]
    pub const fn is_socket(&self) -> bool {
        with_stat!(self, |s| s.is_socket())
    }

    /// Returns true if file type is link.
    #[inline]
    pub const fn is_link(&self) -> bool {
        with_stat!(self, |s| s.is_link())
    }

    /// Returns true if file type is regular.
    #[inline]
    pub const fn is_regular(&self) -> bool {
        with_stat!(self, |s| s.is_regular())
    }

    /// Returns true if file type is block.
    #[inline]
    pub const fn is_block(&self) -> bool {
        with_stat!(self, |s| s.is_block())
    }

    /// Returns true if file type is directory.
    #[inline]
    pub const fn is_directory(&self) -> bool {
        with_stat!(self, |s| s.is_directory())
    }

    /// Alias for `Self::is_directory()`.
    #[inline]
    pub const fn is_dir(&self) -> bool {
        with_stat!(self, |s| s.is_dir())
    }

    /// Returns true if file type is character.
    #[inline]
    pub const fn is_character(&self) -> bool {
        with_stat!(self, |s| s.is_character())
    }

    /// Alias for `Self::is_character()`.
    #[inline]
    pub const fn is_char(&self) -> bool {
        with_stat!(self, |s| s.is_char())
    }

    /// Returns true if file type is FIFO.
    #[inline]
    pub const fn is_fifo(&self) -> bool {
        with_stat!(self, |s| s.is_fifo())
    }

    /// Returns the inode number of the file.
    #[inline]
    pub const fn inode(&self) -> u64 {
        with_stat!(self, |s| s.inode())
    }

    /// Returns the size of the file (if it is a regular file or a symbolic
    /// link) in bytes. The size of a symbolic link is the length of
    /// the pathname it contains, without a terminating null byte.
    #[inline]
    pub const fn size(&self) -> i64 {
        with_stat!(self, |s| s.size())
    }

    /// Returns the number of blocks allocated to the file on the medium, in
    /// 512-byte units. (This may be smaller than stx_size/512 when the
    /// file has holes.)
    #[inline]
    pub const fn blocks(&self) -> i64 {
        with_stat!(self, |s| s.blocks())
    }

    /// Returns the file's last access timestamp.
    #[inline]
    pub const fn atime(&self) -> Timestamp {
        with_stat!(self, |s| s.atime())
    }

    /// Returns the file's last status change timestamp.
    #[inline]
    pub const fn ctime(&self) -> Timestamp {
        with_stat!(self, |s| s.ctime())
    }

    /// Returns the file's last modification timestamp.
    #[inline]
    pub const fn mtime(&self) -> Timestamp {
        with_stat!(self, |s| s.mtime())
    }

    /// Returns the major device that this file (inode) represents if the file
    /// is of block or character device type
    #[inline]
    pub const fn rdev_major(&self) -> u32 {
        with_stat!(self, |s| s.rdev_major())
    }

    /// Returns the minor device that this file (inode) represents if the file
    /// is of block or character device type
    #[inline]
    pub const fn rdev_minor(&self) -> u32 {
        with_stat!(self, |s| s.rdev_minor())
    }

    /// Returns the device that this file (inode) represents if the file is of
    /// block or character device type
    #[inline]
    pub const fn rdev(&self) -> u64 {
        with_stat!(self, |s| s.rdev())
    }

    /// Returns the major device on which this file (inode) resides.
    #[inline]
    pub const fn dev_major(&self) -> u32 {
        with_stat!(self, |s| s.dev_major())
    }

    /// Returns the minor device on which this file (inode) resides.
    #[inline]
    pub const fn dev_minor(&self) -> u32 {
        with_stat!(self, |s| s.dev_minor())
    }

    /// Returns the device on which this file (inode) resides.
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

/// Returns an empty path representation.
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

/// If not feature `linux_4_11` try to call [crate::raw::statx] and fallback
/// to [crate::raw::fstatat] if not available.
///
/// # Safety
///
/// This function is marked as unsafe because directory file descriptor
/// (`dirfd`) cannot be checked.
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
        1 => crate::raw::statx(dirfd, path, flags, crate::raw::StatXMask::BASIC_STATS)
            .map(Stat::Statx),
        _ => match crate::raw::statx(dirfd, path, flags, crate::raw::StatXMask::BASIC_STATS) {
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

/// If not feature `linux_4_11` try to call [crate::raw::statx] and fallback
/// to [crate::raw::fstatat] if not available.
///
/// # Safety
///
/// This function is marked as unsafe because directory file descriptor
/// (`dirfd`) cannot be checked.
#[cfg(any(feature = "linux_4_11", target_arch = "loongarch64"))]
#[inline]
pub unsafe fn fstatat<P: AsRef<Path>>(
    dirfd: RawFd,
    path: P,
    flags: StatAtFlags,
) -> Result<Stat, Errno> {
    raw::statx(dirfd, path, flags, crate::raw::StatXMask::empty())
}

/// Call [crate::fstatat] for `path` in the current directory
/// following symlinks.
#[inline]
pub fn stat<P: AsRef<Path>>(path: P) -> Result<Stat, Errno> {
    unsafe { fstatat(CURRENT_DIRECTORY, path, StatAtFlags::empty()) }
}

/// Call [crate::fstatat] for `path` in the current directory
/// not following symlinks.
#[inline]
pub fn lstat<P: AsRef<Path>>(path: P) -> Result<Stat, Errno> {
    unsafe { fstatat(CURRENT_DIRECTORY, path, StatAtFlags::SYMLINK_NOFOLLOW) }
}

/// Call [crate::fstatat] on the `dirfd` directory file descriptor
///
/// # Safety
///
/// This function is marked as unsafe because directory file descriptor
/// (`dirfd`) cannot be checked.
#[inline]
pub unsafe fn fstat(dirfd: RawFd) -> Result<Stat, Errno> {
    if dirfd < 0 {
        return Err(Errno::EBADF);
    }

    fstatat(dirfd, empty_path(), StatAtFlags::EMPTY_PATH)
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

        let stat = retry(|| stat(dev_null()));
        assert!(stat.is_ok());
        let stat = stat.unwrap();

        assert_eq!(stat.dev(), c_stat.st_dev as u64);
        assert_eq!(stat.inode(), c_stat.st_ino as u64);
        assert_eq!(stat.nlink(), c_stat.st_nlink as u32);
        assert_eq!(
            stat.mode().as_u16() | stat.file_type().as_u16(),
            c_stat.st_mode as u16
        );
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
