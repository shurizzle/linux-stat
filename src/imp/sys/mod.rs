#![allow(non_camel_case_types)]

use core::{ffi::CStr, mem::MaybeUninit};

use crate::{Errno, RawFd};

pub use crate::arch::stat;

pub const AT_FDCWD: RawFd = -100;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StatAtFlags: u32 {
        const EMPTY_PATH = 0x1000;
        const NO_AUTOMOUNT = 0x800;
        const SYMLINK_NOFOLLOW = 0x100;
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StatXMask: u32 {
        const TYPE = 0x0001;
        const MODE = 0x0002;
        const NLINK = 0x0004;
        const UID = 0x0008;
        const GID = 0x0010;
        const ATIME = 0x0020;
        const MTIME = 0x0040;
        const CTIME = 0x0080;
        const INO = 0x0100;
        const SIZE = 0x0200;
        const BLOCKS = 0x0400;
        const BASIC_STATS = 0x07ff;
        const ALL = 0x0fff;
        const BTIME = 0x0800;
        const MNT_ID = 0x1000;
        const DIOALIGN = 0x2000;
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StatXAttr: u32 {
        const COMPRESSED = 0x0004;
        const IMMUTABLE = 0x0010;
        const APPEND = 0x0020;
        const NODUMP = 0x0040;
        const ENCRYPTED = 0x0800;
        const AUTOMOUNT = 0x1000;
        const MOUNT_ROOT = 0x2000;
        const VERITY = 0x100000;
        const DAX = 0x200000;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timestamp {
    pub tv_sec: i64,
    pub tv_nsec: u32,
    pub __pad0: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Statx {
    stx_mask: StatXMask,
    stx_blksize: u32,
    stx_attributes: u64,
    stx_nlink: u32,
    stx_uid: u32,
    stx_gid: u32,
    stx_mode: u16,
    __pad0: u16,
    stx_ino: u64,
    stx_size: u64,
    stx_blocks: u64,
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

impl Statx {
    #[inline]
    pub fn blksize(&self) -> u32 {
        self.stx_blksize
    }

    #[inline]
    pub fn attributes(&self) -> u64 {
        self.stx_attributes
    }

    #[inline]
    pub fn nlink(&self) -> u32 {
        self.stx_nlink
    }

    #[inline]
    pub fn uid(&self) -> u32 {
        self.stx_uid
    }

    #[inline]
    pub fn gid(&self) -> u32 {
        self.stx_gid
    }

    #[inline]
    pub fn mode(&self) -> u16 {
        self.stx_mode
    }

    #[inline]
    pub fn ino(&self) -> u64 {
        self.stx_ino
    }

    #[inline]
    pub fn size(&self) -> u64 {
        self.stx_size
    }

    #[inline]
    pub fn blocks(&self) -> u64 {
        self.stx_blocks
    }

    #[inline]
    pub fn attributes_mask(&self) -> StatXAttr {
        self.stx_attributes_mask
    }

    #[inline]
    pub fn atime(&self) -> timestamp {
        self.stx_atime
    }

    #[inline]
    pub fn btime(&self) -> timestamp {
        self.stx_btime
    }

    #[inline]
    pub fn ctime(&self) -> timestamp {
        self.stx_ctime
    }

    #[inline]
    pub fn mtime(&self) -> timestamp {
        self.stx_mtime
    }

    #[inline]
    pub fn rdev_major(&self) -> u32 {
        self.stx_rdev_major
    }

    #[inline]
    pub fn rdev_minor(&self) -> u32 {
        self.stx_rdev_minor
    }

    #[inline]
    pub fn dev_major(&self) -> u32 {
        self.stx_dev_major
    }

    #[inline]
    pub fn dev_minor(&self) -> u32 {
        self.stx_dev_minor
    }

    #[inline]
    pub fn mnt_id(&self) -> u64 {
        self.stx_mnt_id
    }

    #[inline]
    pub fn dio_mem_align(&self) -> u32 {
        self.stx_dio_mem_align
    }

    #[inline]
    pub fn dio_offset_align(&self) -> u32 {
        self.stx_dio_offset_align
    }
}

impl Statx {
    pub fn uninit() -> MaybeUninit<Self> {
        let mut buf = MaybeUninit::<Self>::uninit();
        unsafe {
            let buf = &mut *buf.as_mut_ptr();
            buf.__pad0 = 0;
            buf.spare.as_mut_slice().fill(0);
        }
        buf
    }
}

#[inline]
pub fn fstatat(dirfd: RawFd, path: &CStr, flags: StatAtFlags) -> Result<stat, Errno> {
    let mut buf = stat::uninit();
    unsafe {
        Errno::from_ret(crate::arch::fstatat(
            dirfd,
            path.as_ptr(),
            buf.as_mut_ptr(),
            flags.bits(),
        ))?;
        Ok(buf.assume_init())
    }
}

#[inline]
pub fn statx(
    dirfd: RawFd,
    path: &CStr,
    flags: StatAtFlags,
    mask: StatXMask,
) -> Result<Statx, Errno> {
    let mut buf = Statx::uninit();
    unsafe {
        Errno::from_ret(crate::arch::statx(
            dirfd,
            path.as_ptr(),
            flags.bits(),
            mask.bits(),
            buf.as_mut_ptr().cast(),
        ))?;
        Ok(buf.assume_init())
    }
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
fn statx_dev_null() {
    _ = dbg!(statx(
        AT_FDCWD,
        unsafe { CStr::from_ptr(b"/dev/null\0".as_ptr().cast()) },
        StatAtFlags::empty(),
        StatXMask::BASIC_STATS
    ));
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
    let res = fstatat(
        AT_FDCWD,
        unsafe { CStr::from_ptr(b"/dev/null\0".as_ptr().cast()) },
        StatAtFlags::empty(),
    );
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
