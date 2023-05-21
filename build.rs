use std::{fmt, io::Write};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_OS");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_ARCH");

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() != "linux" {
        return;
    }

    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    if use_arm_thumb() {
        use_feature("thumb");
    }

    if use_arm_thumb2() {
        use_feature("thumb2");
    }

    let test_asm = "extern crate core;\npub unsafe fn f() { ::core::arch::asm!(\"nop\"); }";

    if can_compile(test_asm) {
        return;
    } else if can_compile(format!("#![feature(asm_experimental_arch)]\n{}", test_asm)) {
        use_feature("use_asm_exp");
        return;
    } else {
        use_feature("outline_asm");
    }

    let trampoline = format!("trampoline/{}.c", arch);

    println!("cargo:rerun-if-changed={}", trampoline);

    let mut comp = cc::Build::new();
    comp.cargo_metadata(true)
        .emit_rerun_if_env_changed(true)
        .pic(false)
        .static_crt(true)
        .file(&trampoline);
    if has_broken_ebx_asm() {
        comp.define("BROKEN_EBX_ASM", None);
    }
    comp.compile("libtrampoline.a");
}

fn use_feature<T: AsRef<str>>(feat: T) {
    println!("cargo:rustc-cfg={}", feat.as_ref());
}

fn has_broken_ebx_asm() -> bool {
    try_compile(
        "int foo(int x) { __asm__ ( \"\" : \"+b\"(x) ); return x; }",
        |c| {
            c.pic(true);
        },
    )
}

fn try_compile<T, F>(content: T, setter: F) -> bool
where
    T: fmt::Display,
    F: FnOnce(&mut cc::Build),
{
    let mut comp = cc::Build::new();
    comp.cargo_metadata(false)
        .emit_rerun_if_env_changed(false)
        .pic(false)
        .static_crt(true);
    setter(&mut comp);
    let comp = comp.get_compiler();

    if comp.is_like_msvc() {
        // we don't need the check this if we are on Windows.
        return false;
    }

    let mut comp = comp.to_command();

    let fin = {
        let mut tempin = tempfile::Builder::new().suffix(".c").tempfile().unwrap();
        write!(tempin, "{}", content).unwrap();
        tempin.into_temp_path()
    };

    comp.arg("-o")
        .arg("/dev/null")
        .arg(&fin)
        .stderr(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success()
}

fn cc_test_defined<T: AsRef<str>>(def: T) -> bool {
    struct H<'a>(&'a str);
    impl<'a> fmt::Display for H<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "#ifdef ")?;
            write!(f, "{}", self.0)?;
            write!(
                f,
                "
int main(void) {{ return 0; }}
#else
#error UNDEFINED
#endif"
            )
        }
    }

    try_compile(H(def.as_ref()), |_| ())
}

fn use_arm_thumb() -> bool {
    if std::env::var("CARGO_CFG_TARGET_ARCH").unwrap() != "arm" {
        return false;
    }
    cc_test_defined("__thumb__")
}

fn use_arm_thumb2() -> bool {
    if std::env::var("CARGO_CFG_TARGET_ARCH").unwrap() != "arm" {
        return false;
    }
    cc_test_defined("__thumb2__")
}

fn can_compile<T: AsRef<str>>(test: T) -> bool {
    use std::process::Stdio;

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let rustc = std::env::var("RUSTC").unwrap();
    let target = std::env::var("TARGET").unwrap();

    // Use `RUSTC_WRAPPER` if it's set, unless it's set to an empty string,
    // as documented [here].
    // [here]: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-reads
    let wrapper =
        std::env::var("RUSTC_WRAPPER")
            .ok()
            .and_then(|w| if w.is_empty() { None } else { Some(w) });

    let mut cmd = if let Some(wrapper) = wrapper {
        let mut cmd = std::process::Command::new(wrapper);
        // The wrapper's first argument is supposed to be the path to rustc.
        cmd.arg(rustc);
        cmd
    } else {
        std::process::Command::new(rustc)
    };

    cmd.arg("--crate-type=rlib") // Don't require `main`.
        .arg("--emit=metadata") // Do as little as possible but still parse.
        .arg("--target")
        .arg(target)
        .arg("--out-dir")
        .arg(out_dir); // Put the output somewhere inconsequential.

    // If Cargo wants to set RUSTFLAGS, use that.
    if let Ok(rustflags) = std::env::var("CARGO_ENCODED_RUSTFLAGS") {
        if !rustflags.is_empty() {
            for arg in rustflags.split('\x1f') {
                cmd.arg(arg);
            }
        }
    }

    let mut child = cmd
        .arg("-") // Read from stdin.
        .stdin(Stdio::piped()) // Stdin is a pipe.
        .stderr(Stdio::null()) // Errors from feature detection aren't interesting and can be confusing.
        .spawn()
        .unwrap();

    writeln!(child.stdin.take().unwrap(), "{}", test.as_ref()).unwrap();

    child.wait().unwrap().success()
}
