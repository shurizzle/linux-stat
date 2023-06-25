#![allow(non_camel_case_types)]

use core::{fmt, mem::MaybeUninit};

use crate::{Mode, RawFd, Timestamp};

#[cfg_attr(target_arch = "aarch64", path = "aarch64.rs")]
#[cfg_attr(target_arch = "arm", path = "arm.rs")]
#[cfg_attr(target_arch = "mips", path = "mips.rs")]
#[cfg_attr(target_arch = "mips64", path = "mips64.rs")]
#[cfg_attr(target_arch = "powerpc", path = "powerpc.rs")]
#[cfg_attr(target_arch = "powerpc64", path = "powerpc64.rs")]
#[cfg_attr(target_arch = "riscv64", path = "riscv64.rs")]
#[cfg_attr(target_arch = "s390x", path = "s390x.rs")]
#[cfg_attr(target_arch = "x86", path = "x86.rs")]
#[cfg_attr(target_arch = "x86_64", path = "x86_64.rs")]
mod imp;

#[cfg(not(target_arch = "loongarch64"))]
pub use imp::stat;

use linux_syscalls::{bitflags, syscall, Errno, Sysno};

pub const AT_FDCWD: RawFd = -100;

#[cfg(not(target_arch = "loongarch64"))]
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
    pub enum StatXMask: u32 {
        TYPE = 0x0001,
        MODE = 0x0002,
        NLINK = 0x0004,
        UID = 0x0008,
        GID = 0x0010,
        ATIME = 0x0020,
        MTIME = 0x0040,
        CTIME = 0x0080,
        INO = 0x0100,
        SIZE = 0x0200,
        BLOCKS = 0x0400,
        BASIC_STATS = 0x07ff,
        ALL = 0x0fff,
        BTIME = 0x0800,
        MNT_ID = 0x1000,
        DIOALIGN = 0x2000,
    }
}

bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum StatXAttr: u64 {
        COMPRESSED = 0x0004,
        IMMUTABLE = 0x0010,
        APPEND = 0x0020,
        NODUMP = 0x0040,
        ENCRYPTED = 0x0800,
        AUTOMOUNT = 0x1000,
        MOUNT_ROOT = 0x2000,
        VERITY = 0x100000,
        DAX = 0x200000,
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Statx {
    stx_mask: StatXMask,
    stx_blksize: i32,
    stx_attributes: u64,
    stx_nlink: u32,
    stx_uid: u32,
    stx_gid: u32,
    stx_mode: Mode,
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

impl Statx {
    #[inline]
    pub const fn block_size(&self) -> i32 {
        self.stx_blksize
    }

    #[inline]
    pub const fn attributes(&self) -> u64 {
        self.stx_attributes
    }

    #[inline]
    pub const fn nlink(&self) -> u32 {
        self.stx_nlink
    }

    #[inline]
    pub const fn uid(&self) -> u32 {
        self.stx_uid
    }

    #[inline]
    pub const fn gid(&self) -> u32 {
        self.stx_gid
    }

    #[inline]
    pub const fn mode(&self) -> Mode {
        self.stx_mode
    }

    #[inline]
    pub const fn ino(&self) -> u64 {
        self.stx_ino
    }

    #[inline]
    pub const fn size(&self) -> i64 {
        self.stx_size
    }

    #[inline]
    pub const fn blocks(&self) -> i64 {
        self.stx_blocks
    }

    #[inline]
    pub const fn attributes_mask(&self) -> StatXAttr {
        self.stx_attributes_mask
    }

    #[inline]
    pub const fn atime(&self) -> Timestamp {
        Timestamp {
            secs: self.stx_atime.tv_sec,
            nsecs: self.stx_atime.tv_nsec,
        }
    }

    #[inline]
    pub const fn btime(&self) -> Timestamp {
        Timestamp {
            secs: self.stx_btime.tv_sec,
            nsecs: self.stx_btime.tv_nsec,
        }
    }

    #[inline]
    pub const fn ctime(&self) -> Timestamp {
        Timestamp {
            secs: self.stx_ctime.tv_sec,
            nsecs: self.stx_ctime.tv_nsec,
        }
    }

    #[inline]
    pub const fn mtime(&self) -> Timestamp {
        Timestamp {
            secs: self.stx_mtime.tv_sec,
            nsecs: self.stx_mtime.tv_nsec,
        }
    }

    #[inline]
    pub const fn rdev_major(&self) -> u32 {
        self.stx_rdev_major
    }

    #[inline]
    pub const fn rdev_minor(&self) -> u32 {
        self.stx_rdev_minor
    }

    #[inline]
    pub const fn rdev(&self) -> u64 {
        makedev(self.rdev_major(), self.rdev_minor())
    }

    #[inline]
    pub const fn dev_major(&self) -> u32 {
        self.stx_dev_major
    }

    #[inline]
    pub const fn dev_minor(&self) -> u32 {
        self.stx_dev_minor
    }

    #[inline]
    pub const fn dev(&self) -> u64 {
        makedev(self.dev_major(), self.dev_minor())
    }

    #[inline]
    pub const fn mount_id(&self) -> u64 {
        self.stx_mnt_id
    }

    #[inline]
    pub const fn dio_mem_align(&self) -> u32 {
        self.stx_dio_mem_align
    }

    #[inline]
    pub const fn dio_offset_align(&self) -> u32 {
        self.stx_dio_offset_align
    }
}

impl fmt::Debug for Statx {
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

impl Statx {
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

#[cfg(not(target_arch = "loongarch64"))]
#[inline(always)]
const fn minor(dev: u64) -> u32 {
    (((dev >> 32) & 0xfffff000) | ((dev >> 8) & 0xfff)) as u32
}

#[cfg(not(target_arch = "loongarch64"))]
#[inline(always)]
const fn major(dev: u64) -> u32 {
    (((dev >> 12) & 0xffffff00) | (dev & 0xff)) as u32
}

#[cfg(not(target_arch = "loongarch64"))]
impl stat {
    #[inline]
    pub const fn dev_minor(&self) -> u32 {
        minor(self.dev())
    }

    #[inline]
    pub const fn dev_major(&self) -> u32 {
        major(self.dev())
    }

    #[inline]
    pub const fn rdev_minor(&self) -> u32 {
        minor(self.rdev())
    }

    #[inline]
    pub const fn rdev_major(&self) -> u32 {
        major(self.rdev())
    }
}

#[cfg(not(target_arch = "loongarch64"))]
#[inline]
pub fn fstatat(dirfd: RawFd, path: &[u8], flags: StatAtFlags) -> Result<stat, Errno> {
    let mut buf = stat::uninit();
    unsafe {
        syscall!(
            imp::SYS_FSTATAT,
            dirfd,
            path.as_ptr(),
            buf.as_mut_ptr(),
            flags.bits()
        )?;
        Ok(buf.assume_init())
    }
}

#[inline]
pub fn statx(
    dirfd: RawFd,
    path: &[u8],
    flags: StatAtFlags,
    mask: StatXMask,
) -> Result<Statx, Errno> {
    let mut buf = Statx::uninit();
    unsafe {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn retry<T, F: Fn() -> Result<T, Errno>>(f: F) -> Result<T, Errno> {
        loop {
            match f() {
                Err(Errno::EINTR) => (),
                other => return other,
            }
        }
    }

    fn c_stat() -> Result<libc::stat64, Errno> {
        unsafe {
            let mut buf = core::mem::MaybeUninit::<libc::stat64>::uninit();
            if libc::fstatat64(
                libc::AT_FDCWD,
                b"/dev/null\0".as_ptr().cast(),
                buf.as_mut_ptr(),
                0,
            ) == -1
            {
                return Err(Errno::from_io_error(std::io::Error::last_os_error()).unwrap());
            }
            Ok(buf.assume_init())
        }
    }

    #[cfg(not(target_arch = "loongarch64"))]
    #[test]
    #[allow(clippy::unnecessary_cast)]
    fn stat64_dev_null() {
        let c_stat = retry(c_stat);
        assert!(c_stat.is_ok());
        let c_stat = c_stat.unwrap();

        let stat = retry(|| fstatat(AT_FDCWD, b"/dev/null\0", StatAtFlags::empty()));
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

    #[test]
    #[allow(clippy::unnecessary_cast)]
    #[cfg_attr(target_arch = "s390x", ignore)]
    fn statx_dev_null() {
        let c_stat = retry(c_stat);
        assert!(c_stat.is_ok());
        let c_stat = c_stat.unwrap();

        let statx = retry(|| {
            statx(
                AT_FDCWD,
                b"/dev/null\0",
                StatAtFlags::empty(),
                StatXMask::empty(),
            )
        });
        assert!(statx.is_ok());
        let statx = statx.unwrap();

        assert_eq!(statx.dev(), c_stat.st_dev as u64);
        assert_eq!(statx.ino(), c_stat.st_ino as u64);
        assert_eq!(statx.nlink(), c_stat.st_nlink as u32);
        assert_eq!(statx.mode().as_u16(), c_stat.st_mode as u16);
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
