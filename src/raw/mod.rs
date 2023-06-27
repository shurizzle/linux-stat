//! Raw structures and functions implementations.

use core::{fmt, mem::MaybeUninit};

use crate::{CStr, FileType, Mode, RawFd, StatAtFlags, Timestamp};

#[cfg_attr(
    all(not(feature = "linux_4_11"), target_arch = "aarch64"),
    path = "aarch64.rs"
)]
#[cfg_attr(all(not(feature = "linux_4_11"), target_arch = "arm"), path = "arm.rs")]
#[cfg_attr(
    all(not(feature = "linux_4_11"), target_arch = "mips"),
    path = "mips.rs"
)]
#[cfg_attr(
    all(not(feature = "linux_4_11"), target_arch = "mips64"),
    path = "mips64.rs"
)]
#[cfg_attr(
    all(not(feature = "linux_4_11"), target_arch = "powerpc"),
    path = "powerpc.rs"
)]
#[cfg_attr(
    all(not(feature = "linux_4_11"), target_arch = "powerpc64"),
    path = "powerpc64.rs"
)]
#[cfg_attr(
    all(not(feature = "linux_4_11"), target_arch = "riscv64"),
    path = "riscv64.rs"
)]
#[cfg_attr(
    all(not(feature = "linux_4_11"), target_arch = "s390x"),
    path = "s390x.rs"
)]
#[cfg_attr(all(not(feature = "linux_4_11"), target_arch = "x86"), path = "x86.rs")]
#[cfg_attr(
    all(not(feature = "linux_4_11"), target_arch = "x86_64"),
    path = "x86_64.rs"
)]
mod stat_imp;

#[cfg(all(not(feature = "linux_4_11"), not(target_arch = "loongarch64")))]
pub use stat_imp::stat;

use linux_syscalls::{bitflags, syscall, Errno, Sysno};

bitflags! {
    /// A mask to tell the kernel which fields the caller is interested in.
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum StatXMask: u32 {
        /// Want stx_mode & S_IFMT
        TYPE = 0x0001,
        /// Want stx_mode & ~S_IFMT
        MODE = 0x0002,
        /// Want stx_nlink
        NLINK = 0x0004,
        /// Want stx_uid
        UID = 0x0008,
        /// Want stx_gid
        GID = 0x0010,
        /// Want stx_atime
        ATIME = 0x0020,
        /// Want stx_mtime
        MTIME = 0x0040,
        /// Want stx_ctime
        CTIME = 0x0080,
        /// Want stx_ino
        INO = 0x0100,
        /// Want stx_size
        SIZE = 0x0200,
        /// Want stx_blocks
        BLOCKS = 0x0400,
        /// [All of the above]
        BASIC_STATS = 0x07ff,
        /// Want stx_btime
        BTIME = 0x0800,
        /// The same as STATX_BASIC_STATS | STATX_BTIME.
        ALL = 0x0fff,
        /// Want stx_mnt_id (since Linux 5.8)
        MNT_ID = 0x1000,
        /// Want stx_dio_mem_align and stx_dio_offset_align
        /// (since Linux 6.1; support varies by filesystem)
        DIOALIGN = 0x2000,
    }
}

bitflags! {
    /// A set of ORed flags that indicate additional attributes of the file.
    /// Note that any attribute that is not indicated as supported by
    /// stx_attributes_mask has no usable value here.
    /// The bits in stx_attributes_mask correspond bit-by-bit to stx_attributes.
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum StatXAttr: u64 {
        /// The file is compressed by the filesystem and may take extra
        /// resources to access
        COMPRESSED = 0x0004,
        /// The file cannot be modified: it cannot be deleted or renamed, no
        /// hard links can be created to this file and no data can be
        /// written to it.  See chattr(1).
        IMMUTABLE = 0x0010,
        /// The file can only be opened in append mode for writing.  Random
        /// access writing is not permitted.  See chattr(1).
        APPEND = 0x0020,
        /// File is not a candidate for backup when a backup program such as
        /// dump(8) is run.  See chattr(1).
        NODUMP = 0x0040,
        /// A key is required for the file to be encrypted by the
        /// filesystem.
        ENCRYPTED = 0x0800,
        AUTOMOUNT = 0x1000,
        MOUNT_ROOT = 0x2000,
        /// The file has fs-verity enabled.  It cannot be written to, and
        /// all reads from it will be verified against a cryptographic hash
        /// that covers the entire file (e.g., via a Merkle tree).
        VERITY = 0x100000,
        /// The file is in the DAX (cpu direct access) state.  DAX state
        /// attempts to minimize software cache effects for both I/O and
        /// memory mappings of this file.  It requires a file system which
        /// has been configured to support DAX.
        ///
        /// DAX generally assumes all accesses are via CPU load / store
        /// instructions which can minimize overhead for small accesses, but
        /// may adversely affect CPU utilization for large transfers.
        ///
        /// File I/O is done directly to/from user-space buffers and memory
        /// mapped I/O may be performed with direct memory mappings that
        /// bypass the kernel page cache.
        ///
        /// While the DAX property tends to result in data being transferred
        /// synchronously, it does not give the same guarantees as the
        /// O_SYNC flag (see open(2)), where data and the necessary metadata
        /// are transferred together.
        ///
        /// A DAX file may support being mapped with the MAP_SYNC flag,
        /// which enables a program to use CPU cache flush instructions to
        /// persist CPU store operations without an explicit fsync(2).  See
        /// mmap(2) for more information
        DAX = 0x200000,
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
struct timestamp {
    tv_sec: i64,
    tv_nsec: u32,
    __pad: u32,
}

impl fmt::Debug for timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("timestamp")
            .field("tv_sec", &self.tv_sec)
            .field("tv_nsec", &self.tv_nsec)
            .finish()
    }
}

/// `statx()` file informations representation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Statx {
    stx_mask: StatXMask,
    stx_blksize: i32,
    stx_attributes: u64,
    stx_nlink: u32,
    stx_uid: u32,
    stx_gid: u32,
    stx_mode: u16,
    stx_ino: u64,
    stx_size: i64,
    stx_blocks: i64,
    stx_attributes_mask: StatXAttr,
    stx_atime: timestamp,
    stx_btime: timestamp,
    stx_ctime: timestamp,
    stx_mtime: timestamp,
    stx_rdev_major: u32,
    stx_rdev_minor: u32,
    stx_dev_major: u32,
    stx_dev_minor: u32,
    stx_mnt_id: u64,
    stx_dio_mem_align: u32,
    stx_dio_offset_align: u32,
    spare: [u64; 14],
}

#[inline(always)]
const fn makedev(minor: u32, major: u32) -> u64 {
    (((minor as u64) & 0xfffff000) << 32)
        | ((minor as u64) & 0x00000fff) << 8
        | (((major as u64) & 0xffffff00) << 12)
        | (major as u64) & 0xff
}

#[inline(always)]
const fn file_type(mode: u16) -> FileType {
    match mode & 0o170000 {
        0o140000 => FileType::Socket,
        0o120000 => FileType::Link,
        0o100000 => FileType::Regular,
        0o060000 => FileType::Block,
        0o040000 => FileType::Directory,
        0o020000 => FileType::Character,
        0o010000 => FileType::Fifo,
        _ => FileType::Unknown,
    }
}

impl Statx {
    /// Returns the "preferred" block size for efficient filesystem I/O.
    /// (Writing to a file in smaller chunks may cause an inefficient
    /// read-modify-rewrite.)
    #[inline]
    pub const fn block_size(&self) -> i32 {
        self.stx_blksize
    }
    /// Returns further status information about the file (see below for more
    /// information).
    #[inline]
    pub const fn attributes(&self) -> u64 {
        self.stx_attributes
    }

    /// Returns the number of hard links on a file.
    #[inline]
    pub const fn nlink(&self) -> u32 {
        self.stx_nlink
    }

    /// Returns the user ID of the owner of the file.
    #[inline]
    pub const fn uid(&self) -> u32 {
        self.stx_uid
    }

    /// Returns the ID of the group owner of the file.
    #[inline]
    pub const fn gid(&self) -> u32 {
        self.stx_gid
    }

    /// Returns the file mode.
    #[inline]
    pub const fn mode(&self) -> Mode {
        Mode(self.stx_mode & !0o170000)
    }

    /// Returns the file type.
    pub const fn file_type(&self) -> FileType {
        file_type(self.stx_mode)
    }

    /// Returns true if file type is socket.
    #[inline]
    pub const fn is_socket(&self) -> bool {
        self.stx_mode & 0o170000 == 0o140000
    }

    /// Returns true if file type is link.
    #[inline]
    pub const fn is_link(&self) -> bool {
        self.stx_mode & 0o170000 == 0o120000
    }

    /// Returns true if file type is regular.
    #[inline]
    pub const fn is_regular(&self) -> bool {
        self.stx_mode & 0o170000 == 0o100000
    }

    /// Returns true if file type is block.
    #[inline]
    pub const fn is_block(&self) -> bool {
        self.stx_mode & 0o170000 == 0o060000
    }

    /// Returns true if file type is directory.
    #[inline]
    pub const fn is_directory(&self) -> bool {
        self.stx_mode & 0o170000 == 0o040000
    }

    /// Alias for `Self::is_directory()`.
    #[inline]
    pub const fn is_dir(&self) -> bool {
        self.is_directory()
    }

    /// Returns true if file type is character.
    #[inline]
    pub const fn is_character(&self) -> bool {
        self.stx_mode & 0o170000 == 0o020000
    }

    /// Alias for `Self::is_character()`.
    #[inline]
    pub const fn is_char(&self) -> bool {
        self.is_character()
    }

    /// Returns true if file type is FIFO.
    #[inline]
    pub const fn is_fifo(&self) -> bool {
        self.stx_mode & 0o170000 == 0o010000
    }

    /// Returns the inode number of the file.
    #[inline]
    pub const fn inode(&self) -> u64 {
        self.stx_ino
    }

    /// Returns the size of the file (if it is a regular file or a symbolic
    /// link) in bytes. The size of a symbolic link is the length of
    /// the pathname it contains, without a terminating null byte.
    #[inline]
    pub const fn size(&self) -> i64 {
        self.stx_size
    }

    /// Returns the number of blocks allocated to the file on the medium, in
    /// 512-byte units. (This may be smaller than stx_size/512 when the
    /// file has holes.)
    #[inline]
    pub const fn blocks(&self) -> i64 {
        self.stx_blocks
    }

    /// A mask indicating which bits in stx_attributes are supported by
    /// the VFS and the filesystem.
    #[inline]
    pub const fn attributes_mask(&self) -> StatXAttr {
        self.stx_attributes_mask
    }

    /// Returns the file's last access timestamp.
    #[inline]
    pub const fn atime(&self) -> Timestamp {
        Timestamp {
            secs: self.stx_atime.tv_sec,
            nsecs: self.stx_atime.tv_nsec,
        }
    }

    /// Returns the file's creation timestamp
    #[inline]
    pub const fn btime(&self) -> Timestamp {
        Timestamp {
            secs: self.stx_btime.tv_sec,
            nsecs: self.stx_btime.tv_nsec,
        }
    }

    /// Returns the file's last status change timestamp.
    #[inline]
    pub const fn ctime(&self) -> Timestamp {
        Timestamp {
            secs: self.stx_ctime.tv_sec,
            nsecs: self.stx_ctime.tv_nsec,
        }
    }

    /// Returns the file's last modification timestamp.
    #[inline]
    pub const fn mtime(&self) -> Timestamp {
        Timestamp {
            secs: self.stx_mtime.tv_sec,
            nsecs: self.stx_mtime.tv_nsec,
        }
    }

    /// Returns the major device that this file (inode) represents if the file
    /// is of block or character device type
    #[inline]
    pub const fn rdev_major(&self) -> u32 {
        self.stx_rdev_major
    }

    /// Returns the minor device that this file (inode) represents if the file
    /// is of block or character device type
    #[inline]
    pub const fn rdev_minor(&self) -> u32 {
        self.stx_rdev_minor
    }

    /// Returns the device that this file (inode) represents if the file is of
    /// block or character device type
    #[inline]
    pub const fn rdev(&self) -> u64 {
        makedev(self.rdev_major(), self.rdev_minor())
    }

    /// Returns the major device on which this file (inode) resides.
    #[inline]
    pub const fn dev_major(&self) -> u32 {
        self.stx_dev_major
    }

    /// Returns the minor device on which this file (inode) resides.
    #[inline]
    pub const fn dev_minor(&self) -> u32 {
        self.stx_dev_minor
    }

    /// Returns the device on which this file (inode) resides.
    #[inline]
    pub const fn dev(&self) -> u64 {
        makedev(self.dev_major(), self.dev_minor())
    }

    /// The mount ID of the mount containing the file.  This is the same
    /// number reported by name_to_handle_at(2) and corresponds to the
    /// number in the first field in one of the records in
    /// /proc/self/mountinfo.
    #[inline]
    pub const fn mount_id(&self) -> u64 {
        self.stx_mnt_id
    }

    /// Returns the alignment (in bytes) required for user memory buffers for
    /// direct I/O (O_DIRECT) on this file, or 0 if direct I/O is not
    /// supported on this file.
    ///
    /// STATX_DIOALIGN (stx_dio_mem_align and stx_dio_offset_align) is
    /// supported on block devices since Linux 6.1.  The support on
    /// regular files varies by filesystem; it is supported by ext4,
    /// f2fs, and xfs since Linux 6.1.
    #[inline]
    pub const fn dio_mem_align(&self) -> u32 {
        self.stx_dio_mem_align
    }

    /// Returns the alignment (in bytes) required for file offsets and I/O
    /// segment lengths for direct I/O (O_DIRECT) on this file, or 0 if
    /// direct I/O is not supported on this file.  This will only be
    /// nonzero if stx_dio_mem_align is nonzero, and vice versa.
    #[inline]
    pub const fn dio_offset_align(&self) -> u32 {
        self.stx_dio_offset_align
    }

    pub(crate) fn debug(&self, f: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result {
        f.debug_struct(name)
            .field("dev", &self.dev())
            .field("ino", &self.inode())
            .field("nlink", &self.nlink())
            .field("mode", &self.mode())
            .field("uid", &self.uid())
            .field("gid", &self.gid())
            .field("rdev", &self.rdev())
            .field("size", &self.size())
            .field("block_size", &self.block_size())
            .field("blocks", &self.blocks())
            .field("atime", &self.atime())
            .field("btime", &self.btime())
            .field("mtime", &self.mtime())
            .field("ctime", &self.ctime())
            .field("attributes", &self.attributes())
            .field("attributes_mask", &self.attributes_mask())
            .field("mount_id", &self.mount_id())
            .field("dio_mem_align", &self.dio_mem_align())
            .field("dio_offset_align", &self.dio_offset_align())
            .finish()
    }
}

impl fmt::Debug for Statx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.debug(f, "Statx")
    }
}

impl Statx {
    #[doc(hidden)]
    pub fn uninit() -> MaybeUninit<Self> {
        let mut buf = MaybeUninit::<Self>::uninit();
        unsafe {
            let buf = &mut *buf.as_mut_ptr();
            core::ptr::write_bytes(
                &mut buf.spare[0] as *mut u64 as *mut u8,
                0,
                core::mem::size_of_val(&buf.spare),
            );
        }
        buf
    }
}

#[cfg(all(not(feature = "linux_4_11"), not(target_arch = "loongarch64")))]
#[inline(always)]
const fn minor(dev: u64) -> u32 {
    (((dev >> 32) & 0xfffff000) | ((dev >> 8) & 0xfff)) as u32
}

#[cfg(all(not(feature = "linux_4_11"), not(target_arch = "loongarch64")))]
#[inline(always)]
const fn major(dev: u64) -> u32 {
    (((dev >> 12) & 0xffffff00) | (dev & 0xff)) as u32
}

#[cfg(all(not(feature = "linux_4_11"), not(target_arch = "loongarch64")))]
impl stat {
    /// Returns the file mode.
    #[inline]
    pub const fn mode(&self) -> Mode {
        Mode(self.raw_mode() & !0o170000)
    }

    /// Returns the file type.
    pub const fn file_type(&self) -> FileType {
        file_type(self.raw_mode())
    }

    /// Returns true if file type is socket.
    #[inline]
    pub const fn is_socket(&self) -> bool {
        self.raw_mode() & 0o170000 == 0o140000
    }

    /// Returns true if file type is link.
    #[inline]
    pub const fn is_link(&self) -> bool {
        self.raw_mode() & 0o170000 == 0o120000
    }

    /// Returns true if file type is regular.
    #[inline]
    pub const fn is_regular(&self) -> bool {
        self.raw_mode() & 0o170000 == 0o100000
    }

    /// Returns true if file type is block.
    #[inline]
    pub const fn is_block(&self) -> bool {
        self.raw_mode() & 0o170000 == 0o060000
    }

    /// Returns true if file type is directory.
    #[inline]
    pub const fn is_directory(&self) -> bool {
        self.raw_mode() & 0o170000 == 0o040000
    }

    /// Alias for `Self::is_directory()`.
    #[inline]
    pub const fn is_dir(&self) -> bool {
        self.is_directory()
    }

    /// Returns true if file type is character.
    #[inline]
    pub const fn is_character(&self) -> bool {
        self.raw_mode() & 0o170000 == 0o020000
    }

    /// Alias for `Self::is_character()`.
    #[inline]
    pub const fn is_char(&self) -> bool {
        self.is_character()
    }

    /// Returns true if file type is FIFO.
    #[inline]
    pub const fn is_fifo(&self) -> bool {
        self.raw_mode() & 0o170000 == 0o010000
    }

    /// Returns the minor device on which this file (inode) resides.
    #[inline]
    pub const fn dev_minor(&self) -> u32 {
        minor(self.dev())
    }

    /// Returns the major device on which this file (inode) resides.
    #[inline]
    pub const fn dev_major(&self) -> u32 {
        major(self.dev())
    }

    /// Returns the minor device that this file (inode) represents if the file
    /// is of block or character device type
    #[inline]
    pub const fn rdev_minor(&self) -> u32 {
        minor(self.rdev())
    }

    /// Returns the major device that this file (inode) represents if the file
    /// is of block or character device type
    #[inline]
    pub const fn rdev_major(&self) -> u32 {
        major(self.rdev())
    }

    pub(crate) fn debug(&self, f: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result {
        f.debug_struct(name)
            .field("dev", &self.dev())
            .field("ino", &self.inode())
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

#[cfg(all(not(feature = "linux_4_11"), not(target_arch = "loongarch64")))]
impl fmt::Debug for stat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.debug(f, "stat")
    }
}

/// Invoke `fstatat` system call.
///
/// # Safety
///
/// This functions is inherently unsafe because it just wrap the system call
/// and directory file descriptor (`dirfd`) cannot be checked.
#[cfg(all(not(feature = "linux_4_11"), not(target_arch = "loongarch64")))]
#[inline]
pub unsafe fn fstatat<P: AsRef<crate::Path>>(
    dirfd: RawFd,
    path: P,
    flags: StatAtFlags,
) -> Result<stat, Errno> {
    crate::run_with_cstr(path, |path| fstatat_cstr(dirfd, path, flags))
}

/// Invoke `fstatat` system call with `path` as a [crate::CStr].
///
/// # Safety
///
/// This functions is inherently unsafe because it just wrap the system call
/// and directory file descriptor (`dirfd`) cannot be checked.
#[cfg(all(not(feature = "linux_4_11"), not(target_arch = "loongarch64")))]
#[inline]
pub unsafe fn fstatat_cstr(dirfd: RawFd, path: &CStr, flags: StatAtFlags) -> Result<stat, Errno> {
    let mut buf = stat::uninit();
    syscall!(
        stat_imp::SYS_FSTATAT,
        dirfd,
        path.as_ptr(),
        buf.as_mut_ptr(),
        flags.bits()
    )?;
    Ok(buf.assume_init())
}

/// Invoke `statx` system call.
///
/// # Safety
///
/// This functions is inherently unsafe because it just wrap the system call
/// and directory file descriptor (`dirfd`) cannot be checked.
#[inline]
pub unsafe fn statx<P: AsRef<crate::Path>>(
    dirfd: RawFd,
    path: P,
    flags: StatAtFlags,
    mask: StatXMask,
) -> Result<Statx, Errno> {
    crate::run_with_cstr(path, |path| statx_cstr(dirfd, path, flags, mask))
}

/// Invoke `statx` system call with `path` as a [crate::CStr].
///
/// # Safety
///
/// This functions is inherently unsafe because it just wrap the system call
/// and directory file descriptor (`dirfd`) cannot be checked.
#[inline]
pub unsafe fn statx_cstr(
    dirfd: RawFd,
    path: &CStr,
    flags: StatAtFlags,
    mask: StatXMask,
) -> Result<Statx, Errno> {
    let mut buf = Statx::uninit();
    syscall!(
        Sysno::statx,
        dirfd,
        path.as_ptr(),
        flags.bits(),
        mask.bits(),
        buf.as_mut_ptr(),
    )?;
    Ok(buf.assume_init())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(all(not(feature = "linux_4_11"), not(target_arch = "loongarch64")))]
    #[test]
    #[allow(clippy::unnecessary_cast)]
    fn stat64_dev_null() {
        linux_syscalls::init();

        let c_stat = crate::tests::retry(crate::tests::c_stat);
        assert!(c_stat.is_ok());
        let c_stat = c_stat.unwrap();

        let stat = crate::tests::retry(|| unsafe {
            fstatat(
                crate::CURRENT_DIRECTORY,
                crate::tests::dev_null(),
                StatAtFlags::empty(),
            )
        });
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

    #[test]
    #[allow(clippy::unnecessary_cast)]
    #[cfg_attr(target_arch = "s390x", ignore)]
    fn statx_dev_null() {
        linux_syscalls::init();

        let c_stat = crate::tests::retry(crate::tests::c_stat);
        assert!(c_stat.is_ok());
        let c_stat = c_stat.unwrap();

        let statx = crate::tests::retry(|| unsafe {
            statx(
                crate::CURRENT_DIRECTORY,
                crate::tests::dev_null(),
                StatAtFlags::empty(),
                StatXMask::empty(),
            )
        });
        assert!(statx.is_ok());
        let statx = statx.unwrap();

        assert_eq!(statx.dev(), c_stat.st_dev as u64);
        assert_eq!(statx.inode(), c_stat.st_ino as u64);
        assert_eq!(statx.nlink(), c_stat.st_nlink as u32);
        assert_eq!(
            statx.mode().as_u16() | statx.file_type().as_u16(),
            c_stat.st_mode as u16
        );
        assert_eq!(statx.uid(), c_stat.st_uid as u32);
        assert_eq!(statx.gid(), c_stat.st_gid as u32);
        assert_eq!(statx.rdev(), c_stat.st_rdev as u64);
        assert_eq!(statx.size(), c_stat.st_size as i64);
        assert_eq!(statx.block_size(), c_stat.st_blksize as i32);
        assert_eq!(statx.blocks(), c_stat.st_blocks as i64);
        assert_eq!(statx.atime().secs, c_stat.st_atime as i64);
        assert_eq!(statx.atime().nsecs, c_stat.st_atime_nsec as u32);
        assert_eq!(statx.mtime().secs, c_stat.st_mtime as i64);
        assert_eq!(statx.mtime().nsecs, c_stat.st_mtime_nsec as u32);
        assert_eq!(statx.ctime().secs, c_stat.st_ctime as i64);
        assert_eq!(statx.ctime().nsecs, c_stat.st_ctime_nsec as u32);
    }
}
