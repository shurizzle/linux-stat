use core::fmt;

mod generated;

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Errno(pub(crate) i32);

impl Errno {
    #[inline]
    pub fn new(num: i32) -> Self {
        Self(num)
    }

    #[inline]
    pub fn into_raw(self) -> i32 {
        self.0
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.0 < 4096
    }

    #[inline(always)]
    pub fn from_ret(value: usize) -> Result<usize, Errno> {
        if value > -4096isize as usize {
            Err(Self(-(value as i32)))
        } else {
            Ok(value)
        }
    }

    pub fn name(&self) -> Option<&'static str> {
        self.name_and_description().map(|x| x.0)
    }

    pub fn description(&self) -> Option<&'static str> {
        self.name_and_description().map(|x| x.1)
    }

    #[cfg(feature = "std")]
    #[inline]
    pub fn from_io_error(err: std::io::Error) -> Option<Self> {
        err.raw_os_error().map(Self)
    }
}

impl fmt::Display for Errno {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.name_and_description() {
            Some((name, desc)) => {
                write!(f, "{} {name} ({desc})", -self.0)
            }
            None => {
                if self.is_valid() {
                    write!(f, "{}", -self.0)
                } else {
                    write!(f, "Unknown errno {:#x}", self.0)
                }
            }
        }
    }
}

impl fmt::Debug for Errno {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.name() {
            Some(name) => f.write_str(name),
            None => write!(f, "Errno({})", self.0),
        }
    }
}

#[cfg(feature = "std")]
impl From<Errno> for std::io::Error {
    #[inline]
    fn from(value: Errno) -> Self {
        std::io::Error::from_raw_os_error(value.into_raw())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Errno {}
