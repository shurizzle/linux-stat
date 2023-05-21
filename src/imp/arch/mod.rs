#[cfg(any(feature = "aarch64", target_arch = "aarch64"))]
mod aarch64;
#[cfg(any(feature = "arm", target_arch = "arm"))]
mod arm;
#[cfg(any(feature = "mips", target_arch = "mips"))]
mod mips;
#[cfg(any(feature = "mips64", target_arch = "mips64"))]
mod mips64;
#[cfg(any(feature = "powerpc", target_arch = "powerpc"))]
mod powerpc;
#[cfg(any(feature = "powerpc64", target_arch = "powerpc64"))]
mod powerpc64;
#[cfg(any(feature = "riscv32", target_arch = "riscv32"))]
mod riscv32;
#[cfg(any(feature = "riscv64", target_arch = "riscv64"))]
mod riscv64;
#[cfg(any(feature = "s390x", target_arch = "s390x"))]
mod s390x;
#[cfg(any(feature = "x86", target_arch = "x86"))]
mod x86;
#[cfg(any(feature = "x86_64", target_arch = "x86_64"))]
mod x86_64;

#[cfg(target_arch = "aarch64")]
pub use aarch64::*;
#[cfg(target_arch = "arm")]
pub use arm::*;
#[cfg(target_arch = "mips")]
pub use mips::*;
#[cfg(target_arch = "mips64")]
pub use mips64::*;
#[cfg(target_arch = "powerpc")]
pub use powerpc::*;
#[cfg(target_arch = "powerpc64")]
pub use powerpc64::*;
#[cfg(target_arch = "riscv32")]
pub use riscv32::*;
#[cfg(target_arch = "riscv64")]
pub use riscv64::*;
#[cfg(target_arch = "s390x")]
pub use s390x::*;
#[cfg(target_arch = "x86")]
pub use x86::*;
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

#[cfg(outline_asm)]
extern "C" {
    pub(crate) fn __syscall0(n: usize) -> usize;
    pub(crate) fn __syscall1(n: usize, arg0: usize) -> usize;
    pub(crate) fn __syscall2(n: usize, arg0: usize, arg1: usize) -> usize;
    pub(crate) fn __syscall3(n: usize, arg0: usize, arg1: usize, arg2: usize) -> usize;
    pub(crate) fn __syscall4(n: usize, arg0: usize, arg1: usize, arg2: usize, arg3: usize)
        -> usize;
    pub(crate) fn __syscall5(
        n: usize,
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
    ) -> usize;
    pub(crate) fn __syscall6(
        n: usize,
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
        arg5: usize,
    ) -> usize;

    #[cfg(target_arch = "mips")]
    pub(crate) fn __syscall7(
        n: usize,
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
        arg6: usize,
    ) -> usize;
}
