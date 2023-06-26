# linux-stat

![GitHub Workflow Status (with branch)](https://img.shields.io/github/actions/workflow/status/shurizzle/linux-stat/unit-tests.yml?branch=master&style=for-the-badge)
[![Crates.io](https://img.shields.io/crates/v/linux-stat?style=for-the-badge)](https://crates.io/crates/linux-stat)
[![docs.rs](https://img.shields.io/docsrs/linux-stat?style=for-the-badge)](https://docs.rs/linux-stat)
![Crates.io](https://img.shields.io/crates/l/linux-stat?style=for-the-badge)

A pure Rust library that implements `stat` functions on linux with only syscalls.

#### tl;dr

It tries to use `statx` syscall and fallback to `fstatat`.

### Feature flags

- `std`: enable std support.
- `linux_4_11`: assume that kernel is at least 4.11.0 so `statx` is used.

### `#![no_std]`

Enable `#![no_std]` support by disabling the default `std` feature:

```toml
[dependencies]
linux-stat = { version = "*", default-features = false }
```

### Platforms

- aarch64
- arm
- mips
- mips64
- mips64el
- mipsel
- powerpc
- powerpc64
- powerpc64el
- riscv64
- s390x
- x86
- x86_64

### MSRV

1.46.0
