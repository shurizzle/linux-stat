#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(use_asm_exp, feature(asm_experimental_arch))]

#[cfg(all(not(feature = "std"), test))]
extern crate alloc;

#[cfg(any(target_os = "linux"))]
mod imp;
#[cfg(any(target_os = "linux"))]
pub use imp::*;

// v x86_64
// v x86
// v aarch64
// v arm
// ? m68k (nightly) dnc
// v mips (nightly)
// v mips64 (nightly)
// v powerpc (nightly)
// v powerpc64 (nightly)
// ? riscv32 (nightly) dnc
// v riscv64
// v s390x (nightly)
