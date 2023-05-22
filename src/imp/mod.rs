#[cfg(feature = "std")]
pub(crate) use std::os::unix::io::RawFd;
#[cfg(not(feature = "std"))]
pub type RawFd = core::ffi::c_int;

pub mod arch;
pub mod real;
mod sys;

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ModePermission : u8 {
        const READ = 0o4;
        const WRITE = 0o2;
        const EXEC = 0o1;
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mode(u16);

impl Mode {
    #[inline]
    pub fn owner(&self) -> ModePermission {
        ModePermission::from_bits_retain(((self.0 >> 6) & 0o7) as u8)
    }

    #[inline]
    pub fn group(&self) -> ModePermission {
        ModePermission::from_bits_retain(((self.0 >> 3) & 0o7) as u8)
    }

    #[inline]
    pub fn other(&self) -> ModePermission {
        ModePermission::from_bits_retain((self.0 & 0o7) as u8)
    }

    #[inline]
    pub fn suid(&self) -> bool {
        self.0 & 0o4000 == 0o4000
    }

    #[inline]
    pub fn sgid(&self) -> bool {
        self.0 & 0o2000 == 0o2000
    }

    #[inline]
    pub fn svtx(&self) -> bool {
        self.0 & 0o1000 == 0o1000
    }
}

impl From<u16> for Mode {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}
