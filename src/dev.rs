#![allow(clippy::partialeq_ne_impl)]

use core::fmt;

/// Device ID representation backed by an u32.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Dev32(u32);

impl Dev32 {
    /// Crate a new [Dev32] from an u32.
    #[inline]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns device major.
    #[inline]
    pub const fn major(&self) -> u32 {
        (self.0 >> 8) & 0xff
    }

    /// Returns device minor.
    #[inline]
    pub const fn minor(&self) -> u32 {
        (self.0 >> 19) | (self.0 & 0xff)
    }

    /// Returns device id as a u32.
    #[inline]
    pub const fn as_u32(&self) -> u32 {
        self.0
    }

    /// Returns device id as a u64.
    #[inline]
    pub const fn as_u64(&self) -> u64 {
        DevSplit::new(self.major(), self.minor()).as_u64()
    }
}

impl core::hash::Hash for Dev32 {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_u64().hash(state);
    }
}

impl fmt::Debug for Dev32 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dev32({}:{})", self.major(), self.minor())
    }
}

impl PartialEq for Dev32 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        self.0 != other.0
    }
}

impl PartialEq<DevSplit> for Dev32 {
    #[inline]
    fn eq(&self, other: &DevSplit) -> bool {
        self.major() == other.major() && self.minor() == other.minor()
    }

    #[inline]
    fn ne(&self, other: &DevSplit) -> bool {
        self.major() != other.major() || self.minor() != other.minor()
    }
}

impl PartialEq<Dev64> for Dev32 {
    #[inline]
    fn eq(&self, other: &Dev64) -> bool {
        self.major() == other.major() && self.minor() == other.minor()
    }

    #[inline]
    fn ne(&self, other: &Dev64) -> bool {
        self.major() != other.major() || self.minor() != other.minor()
    }
}

impl PartialEq<Dev> for Dev32 {
    fn eq(&self, other: &Dev) -> bool {
        match other {
            Dev::B32(d) => self.eq(d),
            Dev::Split(d) => self.eq(d),
            Dev::B64(d) => self.eq(d),
        }
    }

    fn ne(&self, other: &Dev) -> bool {
        match other {
            Dev::B32(d) => self.ne(d),
            Dev::Split(d) => self.ne(d),
            Dev::B64(d) => self.ne(d),
        }
    }
}

impl PartialEq<u32> for Dev32 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }

    #[inline]
    fn ne(&self, other: &u32) -> bool {
        self.0 != *other
    }
}

impl PartialEq<(u32, u32)> for Dev32 {
    #[inline]
    fn eq(&self, other: &(u32, u32)) -> bool {
        self.eq(&DevSplit::new(other.0, other.1))
    }

    #[inline]
    fn ne(&self, other: &(u32, u32)) -> bool {
        self.ne(&DevSplit::new(other.0, other.1))
    }
}

impl PartialEq<u64> for Dev32 {
    #[inline]
    fn eq(&self, other: &u64) -> bool {
        self.eq(&Dev64::new(*other))
    }

    #[inline]
    fn ne(&self, other: &u64) -> bool {
        self.ne(&Dev64::new(*other))
    }
}

impl Eq for Dev32 {}

impl PartialOrd for Dev32 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        self.0.lt(&other.0)
    }

    #[inline]
    fn le(&self, other: &Self) -> bool {
        self.0.le(&other.0)
    }

    #[inline]
    fn gt(&self, other: &Self) -> bool {
        self.0.gt(&other.0)
    }

    #[inline]
    fn ge(&self, other: &Self) -> bool {
        self.0.ge(&other.0)
    }
}

impl PartialOrd<u32> for Dev32 {
    #[inline]
    fn partial_cmp(&self, other: &u32) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(other)
    }

    #[inline]
    fn lt(&self, other: &u32) -> bool {
        self.0.lt(other)
    }

    #[inline]
    fn le(&self, other: &u32) -> bool {
        self.0.le(other)
    }

    #[inline]
    fn gt(&self, other: &u32) -> bool {
        self.0.gt(other)
    }

    #[inline]
    fn ge(&self, other: &u32) -> bool {
        self.0.ge(other)
    }
}

impl PartialOrd<DevSplit> for Dev32 {
    fn partial_cmp(&self, other: &DevSplit) -> Option<core::cmp::Ordering> {
        match self.major().partial_cmp(&other.major()) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.minor().partial_cmp(&other.minor())
    }

    fn lt(&self, other: &DevSplit) -> bool {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Less => return true,
            _ => return false,
        }
        self.minor().lt(&other.minor())
    }

    fn le(&self, other: &DevSplit) -> bool {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Less => return true,
            _ => return false,
        }
        self.minor().le(&other.minor())
    }

    fn gt(&self, other: &DevSplit) -> bool {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Greater => return true,
            _ => return false,
        }
        self.minor().gt(&other.minor())
    }

    fn ge(&self, other: &DevSplit) -> bool {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Greater => return true,
            _ => return false,
        }
        self.minor().ge(&other.minor())
    }
}

impl PartialOrd<(u32, u32)> for Dev32 {
    fn partial_cmp(&self, other: &(u32, u32)) -> Option<core::cmp::Ordering> {
        match self.major().partial_cmp(&other.0) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.minor().partial_cmp(&other.1)
    }

    fn lt(&self, other: &(u32, u32)) -> bool {
        match self.major().cmp(&other.0) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Less => return true,
            _ => return false,
        }
        self.minor().lt(&other.1)
    }

    fn le(&self, other: &(u32, u32)) -> bool {
        match self.major().cmp(&other.0) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Less => return true,
            _ => return false,
        }
        self.minor().le(&other.1)
    }

    fn gt(&self, other: &(u32, u32)) -> bool {
        match self.major().cmp(&other.0) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Greater => return true,
            _ => return false,
        }
        self.minor().gt(&other.1)
    }

    fn ge(&self, other: &(u32, u32)) -> bool {
        match self.major().cmp(&other.0) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Greater => return true,
            _ => return false,
        }
        self.minor().ge(&other.1)
    }
}

impl PartialOrd<Dev64> for Dev32 {
    fn partial_cmp(&self, other: &Dev64) -> Option<core::cmp::Ordering> {
        match self.major().partial_cmp(&other.major()) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.minor().partial_cmp(&other.minor())
    }

    fn lt(&self, other: &Dev64) -> bool {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Less => return true,
            _ => return false,
        }
        self.minor().lt(&other.minor())
    }

    fn le(&self, other: &Dev64) -> bool {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Less => return true,
            _ => return false,
        }
        self.minor().le(&other.minor())
    }

    fn gt(&self, other: &Dev64) -> bool {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Greater => return true,
            _ => return false,
        }
        self.minor().gt(&other.minor())
    }

    fn ge(&self, other: &Dev64) -> bool {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Greater => return true,
            _ => return false,
        }
        self.minor().ge(&other.minor())
    }
}

impl PartialOrd<u64> for Dev32 {
    #[inline]
    fn partial_cmp(&self, other: &u64) -> Option<core::cmp::Ordering> {
        self.partial_cmp(&Dev64::new(*other))
    }

    #[inline]
    fn lt(&self, other: &u64) -> bool {
        self.lt(&Dev64::new(*other))
    }

    #[inline]
    fn le(&self, other: &u64) -> bool {
        self.le(&Dev64::new(*other))
    }

    #[inline]
    fn gt(&self, other: &u64) -> bool {
        self.gt(&Dev64::new(*other))
    }

    #[inline]
    fn ge(&self, other: &u64) -> bool {
        self.ge(&Dev64::new(*other))
    }
}

impl PartialOrd<Dev> for Dev32 {
    fn partial_cmp(&self, other: &Dev) -> Option<core::cmp::Ordering> {
        match other {
            Dev::B32(d) => self.partial_cmp(d),
            Dev::Split(d) => self.partial_cmp(d),
            Dev::B64(d) => self.partial_cmp(d),
        }
    }

    fn lt(&self, other: &Dev) -> bool {
        match other {
            Dev::B32(d) => self.lt(d),
            Dev::Split(d) => self.lt(d),
            Dev::B64(d) => self.lt(d),
        }
    }

    fn le(&self, other: &Dev) -> bool {
        match other {
            Dev::B32(d) => self.le(d),
            Dev::Split(d) => self.le(d),
            Dev::B64(d) => self.le(d),
        }
    }

    fn gt(&self, other: &Dev) -> bool {
        match other {
            Dev::B32(d) => self.gt(d),
            Dev::Split(d) => self.gt(d),
            Dev::B64(d) => self.gt(d),
        }
    }

    fn ge(&self, other: &Dev) -> bool {
        match other {
            Dev::B32(d) => self.ge(d),
            Dev::Split(d) => self.ge(d),
            Dev::B64(d) => self.ge(d),
        }
    }
}

impl Ord for Dev32 {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

/// Device ID representation backed by two u32 (major, minor).
#[derive(Copy, Clone)]
pub struct DevSplit(u32, u32);

impl DevSplit {
    /// Crate a new [DevSplit] from two u32 (major, minor).
    #[inline]
    pub const fn new(major: u32, minor: u32) -> Self {
        Self(major, minor)
    }

    /// Returns device major.
    #[inline]
    pub const fn major(&self) -> u32 {
        self.0
    }

    /// Returns device minor.
    #[inline]
    pub const fn minor(&self) -> u32 {
        self.1
    }

    /// Returns device id as a u64.
    #[inline]
    pub const fn as_u64(&self) -> u64 {
        (((self.1 as u64) & 0xfffff000) << 32)
            | ((self.1 as u64) & 0x00000fff) << 8
            | (((self.0 as u64) & 0xffffff00) << 12)
            | (self.0 as u64) & 0xff
    }
}

impl fmt::Debug for DevSplit {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DevSplit({}:{})", self.major(), self.minor())
    }
}

impl core::hash::Hash for DevSplit {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_u64().hash(state);
    }
}

impl PartialEq for DevSplit {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.major() == other.major() && self.minor() == other.minor()
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        self.major() != other.major() || self.minor() != other.minor()
    }
}

impl PartialEq<Dev32> for DevSplit {
    #[inline]
    fn eq(&self, other: &Dev32) -> bool {
        self.major() == other.major() && self.minor() == other.minor()
    }

    #[inline]
    fn ne(&self, other: &Dev32) -> bool {
        self.major() != other.major() || self.minor() != other.minor()
    }
}

impl PartialEq<Dev64> for DevSplit {
    #[inline]
    fn eq(&self, other: &Dev64) -> bool {
        self.major() == other.major() && self.minor() == other.minor()
    }

    #[inline]
    fn ne(&self, other: &Dev64) -> bool {
        self.major() != other.major() || self.minor() != other.minor()
    }
}

impl PartialEq<Dev> for DevSplit {
    fn eq(&self, other: &Dev) -> bool {
        match other {
            Dev::B32(d) => self.eq(d),
            Dev::Split(d) => self.eq(d),
            Dev::B64(d) => self.eq(d),
        }
    }

    fn ne(&self, other: &Dev) -> bool {
        match other {
            Dev::B32(d) => self.ne(d),
            Dev::Split(d) => self.ne(d),
            Dev::B64(d) => self.ne(d),
        }
    }
}

impl PartialEq<u32> for DevSplit {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        self.eq(&Dev32::new(*other))
    }

    #[inline]
    fn ne(&self, other: &u32) -> bool {
        self.ne(&Dev32::new(*other))
    }
}

impl PartialEq<(u32, u32)> for DevSplit {
    #[inline]
    fn eq(&self, other: &(u32, u32)) -> bool {
        self.0 == other.0 && self.1 == other.1
    }

    #[inline]
    fn ne(&self, other: &(u32, u32)) -> bool {
        self.0 != other.0 || self.1 != other.1
    }
}

impl PartialEq<u64> for DevSplit {
    #[inline]
    fn eq(&self, other: &u64) -> bool {
        self.eq(&Dev64::new(*other))
    }

    #[inline]
    fn ne(&self, other: &u64) -> bool {
        self.ne(&Dev64::new(*other))
    }
}

impl Eq for DevSplit {}

impl PartialOrd for DevSplit {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        match self.major().partial_cmp(&other.major()) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.minor().partial_cmp(&other.minor())
    }

    fn lt(&self, other: &Self) -> bool {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Less => return true,
            _ => return false,
        }
        self.minor().lt(&other.minor())
    }

    fn le(&self, other: &Self) -> bool {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Less => return true,
            _ => return false,
        }
        self.minor().le(&other.minor())
    }

    fn gt(&self, other: &Self) -> bool {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Greater => return true,
            _ => return false,
        }
        self.minor().gt(&other.minor())
    }

    fn ge(&self, other: &Self) -> bool {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Greater => return true,
            _ => return false,
        }
        self.minor().ge(&other.minor())
    }
}

impl PartialOrd<(u32, u32)> for DevSplit {
    fn partial_cmp(&self, other: &(u32, u32)) -> Option<core::cmp::Ordering> {
        match self.major().partial_cmp(&other.0) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.minor().partial_cmp(&other.1)
    }

    fn lt(&self, other: &(u32, u32)) -> bool {
        match self.major().cmp(&other.0) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Less => return true,
            _ => return false,
        }
        self.minor().lt(&other.1)
    }

    fn le(&self, other: &(u32, u32)) -> bool {
        match self.major().cmp(&other.0) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Less => return true,
            _ => return false,
        }
        self.minor().le(&other.1)
    }

    fn gt(&self, other: &(u32, u32)) -> bool {
        match self.major().cmp(&other.0) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Greater => return true,
            _ => return false,
        }
        self.minor().gt(&other.1)
    }

    fn ge(&self, other: &(u32, u32)) -> bool {
        match self.major().cmp(&other.0) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Greater => return true,
            _ => return false,
        }
        self.minor().ge(&other.1)
    }
}

impl PartialOrd<Dev32> for DevSplit {
    fn partial_cmp(&self, other: &Dev32) -> Option<core::cmp::Ordering> {
        match self.major().partial_cmp(&other.major()) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.minor().partial_cmp(&other.minor())
    }

    fn lt(&self, other: &Dev32) -> bool {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Less => return true,
            _ => return false,
        }
        self.minor().lt(&other.minor())
    }

    fn le(&self, other: &Dev32) -> bool {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Less => return true,
            _ => return false,
        }
        self.minor().le(&other.minor())
    }

    fn gt(&self, other: &Dev32) -> bool {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Greater => return true,
            _ => return false,
        }
        self.minor().gt(&other.minor())
    }

    fn ge(&self, other: &Dev32) -> bool {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Greater => return true,
            _ => return false,
        }
        self.minor().ge(&other.minor())
    }
}

impl PartialOrd<u32> for DevSplit {
    #[inline]
    fn partial_cmp(&self, other: &u32) -> Option<core::cmp::Ordering> {
        self.partial_cmp(&Dev32::new(*other))
    }

    #[inline]
    fn lt(&self, other: &u32) -> bool {
        self.lt(&Dev32::new(*other))
    }

    #[inline]
    fn le(&self, other: &u32) -> bool {
        self.le(&Dev32::new(*other))
    }

    #[inline]
    fn gt(&self, other: &u32) -> bool {
        self.gt(&Dev32::new(*other))
    }

    #[inline]
    fn ge(&self, other: &u32) -> bool {
        self.ge(&Dev32::new(*other))
    }
}

impl PartialOrd<Dev64> for DevSplit {
    fn partial_cmp(&self, other: &Dev64) -> Option<core::cmp::Ordering> {
        match self.major().partial_cmp(&other.major()) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.minor().partial_cmp(&other.minor())
    }

    fn lt(&self, other: &Dev64) -> bool {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Less => return true,
            _ => return false,
        }
        self.minor().lt(&other.minor())
    }

    fn le(&self, other: &Dev64) -> bool {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Less => return true,
            _ => return false,
        }
        self.minor().le(&other.minor())
    }

    fn gt(&self, other: &Dev64) -> bool {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Greater => return true,
            _ => return false,
        }
        self.minor().gt(&other.minor())
    }

    fn ge(&self, other: &Dev64) -> bool {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Greater => return true,
            _ => return false,
        }
        self.minor().ge(&other.minor())
    }
}

impl PartialOrd<u64> for DevSplit {
    #[inline]
    fn partial_cmp(&self, other: &u64) -> Option<core::cmp::Ordering> {
        self.partial_cmp(&Dev64::new(*other))
    }

    #[inline]
    fn lt(&self, other: &u64) -> bool {
        self.lt(&Dev64::new(*other))
    }

    #[inline]
    fn le(&self, other: &u64) -> bool {
        self.le(&Dev64::new(*other))
    }

    #[inline]
    fn gt(&self, other: &u64) -> bool {
        self.gt(&Dev64::new(*other))
    }

    #[inline]
    fn ge(&self, other: &u64) -> bool {
        self.ge(&Dev64::new(*other))
    }
}

impl PartialOrd<Dev> for DevSplit {
    fn partial_cmp(&self, other: &Dev) -> Option<core::cmp::Ordering> {
        match other {
            Dev::B32(d) => self.partial_cmp(d),
            Dev::Split(d) => self.partial_cmp(d),
            Dev::B64(d) => self.partial_cmp(d),
        }
    }

    fn lt(&self, other: &Dev) -> bool {
        match other {
            Dev::B32(d) => self.lt(d),
            Dev::Split(d) => self.lt(d),
            Dev::B64(d) => self.lt(d),
        }
    }

    fn le(&self, other: &Dev) -> bool {
        match other {
            Dev::B32(d) => self.le(d),
            Dev::Split(d) => self.le(d),
            Dev::B64(d) => self.le(d),
        }
    }

    fn gt(&self, other: &Dev) -> bool {
        match other {
            Dev::B32(d) => self.gt(d),
            Dev::Split(d) => self.gt(d),
            Dev::B64(d) => self.gt(d),
        }
    }

    fn ge(&self, other: &Dev) -> bool {
        match other {
            Dev::B32(d) => self.ge(d),
            Dev::Split(d) => self.ge(d),
            Dev::B64(d) => self.ge(d),
        }
    }
}

impl Ord for DevSplit {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.minor().cmp(&other.minor())
    }
}

/// Device ID representation backed by an u64.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Dev64(u64);

impl Dev64 {
    /// Crate a new [Dev64] from an u64.
    #[inline]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns device major.
    #[inline]
    pub const fn major(&self) -> u32 {
        (((self.0 >> 32) & 0xfffff000) | ((self.0 >> 8) & 0xfff)) as u32
    }

    /// Returns device minor.
    #[inline]
    pub const fn minor(&self) -> u32 {
        (((self.0 >> 12) & 0xffffff00) | (self.0 & 0xff)) as u32
    }

    /// Returns device id as a u64.
    #[inline]
    pub const fn as_u64(&self) -> u64 {
        self.0
    }
}

impl fmt::Debug for Dev64 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dev64({}:{})", self.major(), self.minor())
    }
}

impl core::hash::Hash for Dev64 {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_u64().hash(state);
    }
}

impl PartialEq for Dev64 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    #[inline]
    fn ne(&self, other: &Dev64) -> bool {
        self.0 != other.0
    }
}

impl PartialEq<Dev32> for Dev64 {
    #[inline]
    fn eq(&self, other: &Dev32) -> bool {
        self.major() == other.major() && self.minor() == other.minor()
    }

    #[inline]
    fn ne(&self, other: &Dev32) -> bool {
        self.major() != other.major() || self.minor() != other.minor()
    }
}

impl PartialEq<DevSplit> for Dev64 {
    #[inline]
    fn eq(&self, other: &DevSplit) -> bool {
        self.major() == other.major() && self.minor() == other.minor()
    }

    #[inline]
    fn ne(&self, other: &DevSplit) -> bool {
        self.major() != other.major() || self.minor() != other.minor()
    }
}

impl PartialEq<Dev> for Dev64 {
    fn eq(&self, other: &Dev) -> bool {
        match other {
            Dev::B32(d) => self.eq(d),
            Dev::Split(d) => self.eq(d),
            Dev::B64(d) => self.eq(d),
        }
    }

    fn ne(&self, other: &Dev) -> bool {
        match other {
            Dev::B32(d) => self.ne(d),
            Dev::Split(d) => self.ne(d),
            Dev::B64(d) => self.ne(d),
        }
    }
}

impl PartialEq<u32> for Dev64 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        self.eq(&Dev32::new(*other))
    }

    #[inline]
    fn ne(&self, other: &u32) -> bool {
        self.ne(&Dev32::new(*other))
    }
}

impl PartialEq<(u32, u32)> for Dev64 {
    #[inline]
    fn eq(&self, other: &(u32, u32)) -> bool {
        self.eq(&DevSplit::new(other.0, other.1))
    }

    #[inline]
    fn ne(&self, other: &(u32, u32)) -> bool {
        self.ne(&DevSplit::new(other.0, other.1))
    }
}

impl PartialEq<u64> for Dev64 {
    #[inline]
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }

    #[inline]
    fn ne(&self, other: &u64) -> bool {
        self.0 != *other
    }
}

impl Eq for Dev64 {}

impl PartialOrd for Dev64 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        self.0.lt(&other.0)
    }

    #[inline]
    fn le(&self, other: &Self) -> bool {
        self.0.le(&other.0)
    }

    #[inline]
    fn gt(&self, other: &Self) -> bool {
        self.0.gt(&other.0)
    }

    #[inline]
    fn ge(&self, other: &Self) -> bool {
        self.0.ge(&other.0)
    }
}

impl PartialOrd<u64> for Dev64 {
    #[inline]
    fn partial_cmp(&self, other: &u64) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(other)
    }

    #[inline]
    fn lt(&self, other: &u64) -> bool {
        self.0.lt(other)
    }

    #[inline]
    fn le(&self, other: &u64) -> bool {
        self.0.le(other)
    }

    #[inline]
    fn gt(&self, other: &u64) -> bool {
        self.0.gt(other)
    }

    #[inline]
    fn ge(&self, other: &u64) -> bool {
        self.0.ge(other)
    }
}

impl PartialOrd<Dev32> for Dev64 {
    fn partial_cmp(&self, other: &Dev32) -> Option<core::cmp::Ordering> {
        match self.major().partial_cmp(&other.major()) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.minor().partial_cmp(&other.minor())
    }

    fn lt(&self, other: &Dev32) -> bool {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Less => return true,
            _ => return false,
        }
        self.minor().lt(&other.minor())
    }

    fn le(&self, other: &Dev32) -> bool {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Less => return true,
            _ => return false,
        }
        self.minor().le(&other.minor())
    }

    fn gt(&self, other: &Dev32) -> bool {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Greater => return true,
            _ => return false,
        }
        self.minor().gt(&other.minor())
    }

    fn ge(&self, other: &Dev32) -> bool {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Greater => return true,
            _ => return false,
        }
        self.minor().ge(&other.minor())
    }
}

impl PartialOrd<u32> for Dev64 {
    #[inline]
    fn partial_cmp(&self, other: &u32) -> Option<core::cmp::Ordering> {
        self.partial_cmp(&Dev32::new(*other))
    }

    #[inline]
    fn lt(&self, other: &u32) -> bool {
        self.lt(&Dev32::new(*other))
    }

    #[inline]
    fn le(&self, other: &u32) -> bool {
        self.le(&Dev32::new(*other))
    }

    #[inline]
    fn gt(&self, other: &u32) -> bool {
        self.gt(&Dev32::new(*other))
    }

    #[inline]
    fn ge(&self, other: &u32) -> bool {
        self.ge(&Dev32::new(*other))
    }
}

impl PartialOrd<DevSplit> for Dev64 {
    fn partial_cmp(&self, other: &DevSplit) -> Option<core::cmp::Ordering> {
        match self.major().partial_cmp(&other.major()) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.minor().partial_cmp(&other.minor())
    }

    fn lt(&self, other: &DevSplit) -> bool {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Less => return true,
            _ => return false,
        }
        self.minor().lt(&other.minor())
    }

    fn le(&self, other: &DevSplit) -> bool {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Less => return true,
            _ => return false,
        }
        self.minor().le(&other.minor())
    }

    fn gt(&self, other: &DevSplit) -> bool {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Greater => return true,
            _ => return false,
        }
        self.minor().gt(&other.minor())
    }

    fn ge(&self, other: &DevSplit) -> bool {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Greater => return true,
            _ => return false,
        }
        self.minor().ge(&other.minor())
    }
}

impl PartialOrd<(u32, u32)> for Dev64 {
    fn partial_cmp(&self, other: &(u32, u32)) -> Option<core::cmp::Ordering> {
        match self.major().partial_cmp(&other.0) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.minor().partial_cmp(&other.1)
    }

    fn lt(&self, other: &(u32, u32)) -> bool {
        match self.major().cmp(&other.0) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Less => return true,
            _ => return false,
        }
        self.minor().lt(&other.1)
    }

    fn le(&self, other: &(u32, u32)) -> bool {
        match self.major().cmp(&other.0) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Less => return true,
            _ => return false,
        }
        self.minor().le(&other.1)
    }

    fn gt(&self, other: &(u32, u32)) -> bool {
        match self.major().cmp(&other.0) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Greater => return true,
            _ => return false,
        }
        self.minor().gt(&other.1)
    }

    fn ge(&self, other: &(u32, u32)) -> bool {
        match self.major().cmp(&other.0) {
            core::cmp::Ordering::Equal => (),
            core::cmp::Ordering::Greater => return true,
            _ => return false,
        }
        self.minor().ge(&other.1)
    }
}

impl PartialOrd<Dev> for Dev64 {
    fn partial_cmp(&self, other: &Dev) -> Option<core::cmp::Ordering> {
        match other {
            Dev::B32(d) => self.partial_cmp(d),
            Dev::Split(d) => self.partial_cmp(d),
            Dev::B64(d) => self.partial_cmp(d),
        }
    }

    fn lt(&self, other: &Dev) -> bool {
        match other {
            Dev::B32(d) => self.lt(d),
            Dev::Split(d) => self.lt(d),
            Dev::B64(d) => self.lt(d),
        }
    }

    fn le(&self, other: &Dev) -> bool {
        match other {
            Dev::B32(d) => self.le(d),
            Dev::Split(d) => self.le(d),
            Dev::B64(d) => self.le(d),
        }
    }

    fn gt(&self, other: &Dev) -> bool {
        match other {
            Dev::B32(d) => self.gt(d),
            Dev::Split(d) => self.gt(d),
            Dev::B64(d) => self.gt(d),
        }
    }

    fn ge(&self, other: &Dev) -> bool {
        match other {
            Dev::B32(d) => self.ge(d),
            Dev::Split(d) => self.ge(d),
            Dev::B64(d) => self.ge(d),
        }
    }
}

impl Ord for Dev64 {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

/// Unified Device ID representation.
#[derive(Copy, Clone)]
pub enum Dev {
    B32(Dev32),
    Split(DevSplit),
    B64(Dev64),
}

impl Dev {
    /// Returns device major.
    #[inline]
    pub const fn major(&self) -> u32 {
        match self {
            Self::B32(d) => d.major(),
            Self::Split(d) => d.major(),
            Self::B64(d) => d.major(),
        }
    }

    /// Returns device minor.
    #[inline]
    pub const fn minor(&self) -> u32 {
        match self {
            Self::B32(d) => d.minor(),
            Self::Split(d) => d.minor(),
            Self::B64(d) => d.minor(),
        }
    }

    /// Returns device id as a u64.
    #[inline]
    pub const fn as_u64(&self) -> u64 {
        match self {
            Self::B32(d) => d.as_u64(),
            Self::Split(d) => d.as_u64(),
            Self::B64(d) => d.as_u64(),
        }
    }

    /// Create a [Dev] from a u32.
    #[inline]
    pub const fn from_u32(value: u32) -> Self {
        Self::B32(Dev32::new(value))
    }

    /// Create a [Dev] from a u64.
    #[inline]
    pub const fn from_u64(value: u64) -> Self {
        Self::B64(Dev64::new(value))
    }

    /// Create a [Dev] from two u32 (major, minor).
    #[inline]
    pub const fn from_split(major: u32, minor: u32) -> Self {
        Self::Split(DevSplit::new(major, minor))
    }
}

impl fmt::Debug for Dev {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dev({}:{})", self.major(), self.minor())
    }
}

impl core::hash::Hash for Dev {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_u64().hash(state);
    }
}

impl PartialEq for Dev {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Dev::B32(d) => d.eq(other),
            Dev::Split(d) => d.eq(other),
            Dev::B64(d) => d.eq(other),
        }
    }

    fn ne(&self, other: &Self) -> bool {
        match self {
            Dev::B32(d) => d.ne(other),
            Dev::Split(d) => d.ne(other),
            Dev::B64(d) => d.ne(other),
        }
    }
}

impl PartialEq<Dev32> for Dev {
    fn eq(&self, other: &Dev32) -> bool {
        match self {
            Dev::B32(d) => d.eq(other),
            Dev::Split(d) => d.eq(other),
            Dev::B64(d) => d.eq(other),
        }
    }

    fn ne(&self, other: &Dev32) -> bool {
        match self {
            Dev::B32(d) => d.ne(other),
            Dev::Split(d) => d.ne(other),
            Dev::B64(d) => d.ne(other),
        }
    }
}

impl PartialEq<Dev64> for Dev {
    fn eq(&self, other: &Dev64) -> bool {
        match self {
            Dev::B32(d) => d.eq(other),
            Dev::Split(d) => d.eq(other),
            Dev::B64(d) => d.eq(other),
        }
    }

    fn ne(&self, other: &Dev64) -> bool {
        match self {
            Dev::B32(d) => d.ne(other),
            Dev::Split(d) => d.ne(other),
            Dev::B64(d) => d.ne(other),
        }
    }
}

impl PartialEq<DevSplit> for Dev {
    fn eq(&self, other: &DevSplit) -> bool {
        match self {
            Dev::B32(d) => d.eq(other),
            Dev::Split(d) => d.eq(other),
            Dev::B64(d) => d.eq(other),
        }
    }

    fn ne(&self, other: &DevSplit) -> bool {
        match self {
            Dev::B32(d) => d.ne(other),
            Dev::Split(d) => d.ne(other),
            Dev::B64(d) => d.ne(other),
        }
    }
}

impl PartialEq<u32> for Dev {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        self.eq(&Dev32::new(*other))
    }

    #[inline]
    fn ne(&self, other: &u32) -> bool {
        self.ne(&Dev32::new(*other))
    }
}

impl PartialEq<(u32, u32)> for Dev {
    #[inline]
    fn eq(&self, other: &(u32, u32)) -> bool {
        self.eq(&DevSplit::new(other.0, other.1))
    }

    #[inline]
    fn ne(&self, other: &(u32, u32)) -> bool {
        self.ne(&DevSplit::new(other.0, other.1))
    }
}

impl PartialEq<u64> for Dev {
    #[inline]
    fn eq(&self, other: &u64) -> bool {
        self.eq(&Dev64::new(*other))
    }

    #[inline]
    fn ne(&self, other: &u64) -> bool {
        self.ne(&Dev64::new(*other))
    }
}

impl Eq for Dev {}

impl PartialOrd for Dev {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        match self {
            Self::B32(d) => d.partial_cmp(other),
            Self::Split(d) => d.partial_cmp(other),
            Self::B64(d) => d.partial_cmp(other),
        }
    }

    fn lt(&self, other: &Self) -> bool {
        match self {
            Self::B32(d) => d.lt(other),
            Self::Split(d) => d.lt(other),
            Self::B64(d) => d.lt(other),
        }
    }

    fn le(&self, other: &Self) -> bool {
        match self {
            Self::B32(d) => d.le(other),
            Self::Split(d) => d.le(other),
            Self::B64(d) => d.le(other),
        }
    }

    fn gt(&self, other: &Self) -> bool {
        match self {
            Self::B32(d) => d.gt(other),
            Self::Split(d) => d.gt(other),
            Self::B64(d) => d.gt(other),
        }
    }

    fn ge(&self, other: &Self) -> bool {
        match self {
            Self::B32(d) => d.ge(other),
            Self::Split(d) => d.ge(other),
            Self::B64(d) => d.ge(other),
        }
    }
}

impl PartialOrd<Dev32> for Dev {
    fn partial_cmp(&self, other: &Dev32) -> Option<core::cmp::Ordering> {
        match self {
            Self::B32(d) => d.partial_cmp(other),
            Self::Split(d) => d.partial_cmp(other),
            Self::B64(d) => d.partial_cmp(other),
        }
    }

    fn lt(&self, other: &Dev32) -> bool {
        match self {
            Self::B32(d) => d.lt(other),
            Self::Split(d) => d.lt(other),
            Self::B64(d) => d.lt(other),
        }
    }

    fn le(&self, other: &Dev32) -> bool {
        match self {
            Self::B32(d) => d.le(other),
            Self::Split(d) => d.le(other),
            Self::B64(d) => d.le(other),
        }
    }

    fn gt(&self, other: &Dev32) -> bool {
        match self {
            Self::B32(d) => d.gt(other),
            Self::Split(d) => d.gt(other),
            Self::B64(d) => d.gt(other),
        }
    }

    fn ge(&self, other: &Dev32) -> bool {
        match self {
            Self::B32(d) => d.ge(other),
            Self::Split(d) => d.ge(other),
            Self::B64(d) => d.ge(other),
        }
    }
}

impl PartialOrd<u32> for Dev {
    fn partial_cmp(&self, other: &u32) -> Option<core::cmp::Ordering> {
        match self {
            Self::B32(d) => d.partial_cmp(other),
            Self::Split(d) => d.partial_cmp(other),
            Self::B64(d) => d.partial_cmp(other),
        }
    }

    fn lt(&self, other: &u32) -> bool {
        match self {
            Self::B32(d) => d.lt(other),
            Self::Split(d) => d.lt(other),
            Self::B64(d) => d.lt(other),
        }
    }

    fn le(&self, other: &u32) -> bool {
        match self {
            Self::B32(d) => d.le(other),
            Self::Split(d) => d.le(other),
            Self::B64(d) => d.le(other),
        }
    }

    fn gt(&self, other: &u32) -> bool {
        match self {
            Self::B32(d) => d.gt(other),
            Self::Split(d) => d.gt(other),
            Self::B64(d) => d.gt(other),
        }
    }

    fn ge(&self, other: &u32) -> bool {
        match self {
            Self::B32(d) => d.ge(other),
            Self::Split(d) => d.ge(other),
            Self::B64(d) => d.ge(other),
        }
    }
}

impl PartialOrd<DevSplit> for Dev {
    fn partial_cmp(&self, other: &DevSplit) -> Option<core::cmp::Ordering> {
        match self {
            Self::B32(d) => d.partial_cmp(other),
            Self::Split(d) => d.partial_cmp(other),
            Self::B64(d) => d.partial_cmp(other),
        }
    }

    fn lt(&self, other: &DevSplit) -> bool {
        match self {
            Self::B32(d) => d.lt(other),
            Self::Split(d) => d.lt(other),
            Self::B64(d) => d.lt(other),
        }
    }

    fn le(&self, other: &DevSplit) -> bool {
        match self {
            Self::B32(d) => d.le(other),
            Self::Split(d) => d.le(other),
            Self::B64(d) => d.le(other),
        }
    }

    fn gt(&self, other: &DevSplit) -> bool {
        match self {
            Self::B32(d) => d.gt(other),
            Self::Split(d) => d.gt(other),
            Self::B64(d) => d.gt(other),
        }
    }

    fn ge(&self, other: &DevSplit) -> bool {
        match self {
            Self::B32(d) => d.ge(other),
            Self::Split(d) => d.ge(other),
            Self::B64(d) => d.ge(other),
        }
    }
}

impl PartialOrd<(u32, u32)> for Dev {
    fn partial_cmp(&self, other: &(u32, u32)) -> Option<core::cmp::Ordering> {
        match self {
            Self::B32(d) => d.partial_cmp(other),
            Self::Split(d) => d.partial_cmp(other),
            Self::B64(d) => d.partial_cmp(other),
        }
    }

    fn lt(&self, other: &(u32, u32)) -> bool {
        match self {
            Self::B32(d) => d.lt(other),
            Self::Split(d) => d.lt(other),
            Self::B64(d) => d.lt(other),
        }
    }

    fn le(&self, other: &(u32, u32)) -> bool {
        match self {
            Self::B32(d) => d.le(other),
            Self::Split(d) => d.le(other),
            Self::B64(d) => d.le(other),
        }
    }

    fn gt(&self, other: &(u32, u32)) -> bool {
        match self {
            Self::B32(d) => d.gt(other),
            Self::Split(d) => d.gt(other),
            Self::B64(d) => d.gt(other),
        }
    }

    fn ge(&self, other: &(u32, u32)) -> bool {
        match self {
            Self::B32(d) => d.ge(other),
            Self::Split(d) => d.ge(other),
            Self::B64(d) => d.ge(other),
        }
    }
}

impl PartialOrd<Dev64> for Dev {
    fn partial_cmp(&self, other: &Dev64) -> Option<core::cmp::Ordering> {
        match self {
            Self::B32(d) => d.partial_cmp(other),
            Self::Split(d) => d.partial_cmp(other),
            Self::B64(d) => d.partial_cmp(other),
        }
    }

    fn lt(&self, other: &Dev64) -> bool {
        match self {
            Self::B32(d) => d.lt(other),
            Self::Split(d) => d.lt(other),
            Self::B64(d) => d.lt(other),
        }
    }

    fn le(&self, other: &Dev64) -> bool {
        match self {
            Self::B32(d) => d.le(other),
            Self::Split(d) => d.le(other),
            Self::B64(d) => d.le(other),
        }
    }

    fn gt(&self, other: &Dev64) -> bool {
        match self {
            Self::B32(d) => d.gt(other),
            Self::Split(d) => d.gt(other),
            Self::B64(d) => d.gt(other),
        }
    }

    fn ge(&self, other: &Dev64) -> bool {
        match self {
            Self::B32(d) => d.ge(other),
            Self::Split(d) => d.ge(other),
            Self::B64(d) => d.ge(other),
        }
    }
}

impl PartialOrd<u64> for Dev {
    fn partial_cmp(&self, other: &u64) -> Option<core::cmp::Ordering> {
        match self {
            Self::B32(d) => d.partial_cmp(other),
            Self::Split(d) => d.partial_cmp(other),
            Self::B64(d) => d.partial_cmp(other),
        }
    }

    fn lt(&self, other: &u64) -> bool {
        match self {
            Self::B32(d) => d.lt(other),
            Self::Split(d) => d.lt(other),
            Self::B64(d) => d.lt(other),
        }
    }

    fn le(&self, other: &u64) -> bool {
        match self {
            Self::B32(d) => d.le(other),
            Self::Split(d) => d.le(other),
            Self::B64(d) => d.le(other),
        }
    }

    fn gt(&self, other: &u64) -> bool {
        match self {
            Self::B32(d) => d.gt(other),
            Self::Split(d) => d.gt(other),
            Self::B64(d) => d.gt(other),
        }
    }

    fn ge(&self, other: &u64) -> bool {
        match self {
            Self::B32(d) => d.ge(other),
            Self::Split(d) => d.ge(other),
            Self::B64(d) => d.ge(other),
        }
    }
}

impl Ord for Dev {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        match self.major().cmp(&other.major()) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.minor().cmp(&other.minor())
    }
}

impl From<Dev32> for Dev {
    #[inline]
    fn from(value: Dev32) -> Self {
        Self::B32(value)
    }
}

impl From<DevSplit> for Dev {
    #[inline]
    fn from(value: DevSplit) -> Self {
        Self::Split(value)
    }
}

impl From<Dev64> for Dev {
    #[inline]
    fn from(value: Dev64) -> Self {
        Self::B64(value)
    }
}

impl From<u32> for Dev {
    #[inline]
    fn from(value: u32) -> Self {
        Self::from_u32(value)
    }
}

impl From<u64> for Dev {
    #[inline]
    fn from(value: u64) -> Self {
        Self::from_u64(value)
    }
}

impl From<(u32, u32)> for Dev {
    #[inline]
    fn from(value: (u32, u32)) -> Self {
        Self::from_split(value.0, value.1)
    }
}
