#[cfg(feature = "std")]
pub(crate) use std::os::unix::io::RawFd;
#[cfg(not(feature = "std"))]
pub type RawFd = core::ffi::c_int;

pub mod arch;
pub mod real;
mod sys;
