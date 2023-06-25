#![cfg(target_os = "linux")]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(use_asm_exp, feature(asm_experimental_arch))]

#[cfg(feature = "std")]
pub(crate) use std::os::unix::io::RawFd;
#[cfg(not(feature = "std"))]
pub type RawFd = core::ffi::c_int;

pub mod raw;

use core::fmt;

use linux_syscalls::bitflags;

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
