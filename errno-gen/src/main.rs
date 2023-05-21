use std::collections::HashMap;

use atoi::FromRadix10;
use color_eyre::{
    eyre::{bail, Context},
    Report, Result,
};
use serde::Deserialize;

static LINUX_REPO: &str = "https://raw.githubusercontent.com/torvalds/linux";

#[derive(Deserialize)]
struct Release {
    pub version: String,
    pub moniker: String,
}

#[derive(Deserialize)]
struct Releases {
    pub releases: Vec<Release>,
}

fn fetch_path(version: &str, path: &str) -> Result<Box<str>> {
    let url = format!("{LINUX_REPO}/v{version}/{path}");

    eprintln!("Fetching {url}");
    Ok(ureq::get(&url)
        .call()
        .wrap_err_with(|| format!("Failed to fetch {url}"))?
        .into_string()
        .wrap_err_with(|| format!("Failed to parse {url}"))?
        .into_boxed_str())
}

#[derive(Debug)]
pub enum Errno {
    Define(Box<str>, i32, Option<Box<str>>),
    Alias(Box<str>, Box<str>, Option<Box<str>>),
}

fn fetch_errno<'a, I>(version: &'a str, paths: I) -> impl Iterator<Item = Result<Box<str>>> + 'a
where
    I: IntoIterator<Item = &'a str> + 'a,
{
    paths.into_iter().map(|p| fetch_path(version, p))
}

fn parse_line(line: &str) -> Option<Errno> {
    fn space0(b: &str) -> &str {
        b.trim_start()
    }

    fn space1(b: &str) -> Option<&str> {
        if b.chars().next()?.is_whitespace() {
            Some(b.trim_start())
        } else {
            None
        }
    }

    fn int<T: FromRadix10>(b: &str) -> Option<(T, &str)> {
        let (res, size) = T::from_radix_10(b.as_bytes());
        if size == 0 {
            None
        } else {
            Some((res, unsafe { b.get_unchecked(size..) }))
        }
    }

    fn comment(b: &str) -> Option<Box<str>> {
        let b = b.trim();
        let b = b.strip_prefix("/*")?;
        let b = b.strip_suffix("*/")?;
        Some(b.trim().to_string().into_boxed_str())
    }

    let line = line.trim();
    let line = line.strip_prefix('#')?;
    let line = space0(line);
    let line = line.strip_prefix("define")?;
    let line = space1(line)?;
    let (name, line) = line
        .char_indices()
        .find(|&(_, c)| c.is_whitespace())
        .and_then(|(i, _)| {
            let name = unsafe { line.get_unchecked(..i) }
                .to_string()
                .into_boxed_str();
            if name.starts_with('E') {
                Some((name.to_string().into_boxed_str(), unsafe {
                    line.get_unchecked(i..)
                }))
            } else {
                None
            }
        })?;
    let line = space1(line)?;

    if let Some((value, line)) = int::<i32>(line) {
        Some(Errno::Define(name, value, comment(line)))
    } else if line.starts_with('E') {
        let (alias, line) = line
            .char_indices()
            .find(|&(_, c)| c.is_whitespace())
            .and_then(|(i, _)| {
                let name = unsafe { line.get_unchecked(..i) }
                    .to_string()
                    .into_boxed_str();
                if name.starts_with('E') {
                    Some((name.to_string().into_boxed_str(), unsafe {
                        line.get_unchecked(i..)
                    }))
                } else {
                    None
                }
            })?;
        Some(Errno::Alias(
            name,
            alias.to_string().into_boxed_str(),
            comment(line),
        ))
    } else {
        None
    }
}

struct L {
    _keep: Box<str>,
    iter: std::str::Lines<'static>,
}

impl L {
    pub fn new(s: Box<str>) -> Self {
        let iter = unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(s.as_ptr(), s.len())).lines()
        };
        Self { _keep: s, iter }
    }
}

impl Iterator for L {
    type Item = Errno;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let line = self.iter.next()?;
            if let Some(err) = parse_line(line) {
                return Some(err);
            }
        }
    }
}

fn parse_file(file: Box<str>) -> L {
    L::new(file)
}

enum X<E = Report> {
    Err(Option<E>),
    Iter(L),
}

impl Iterator for X {
    type Item = Result<Errno>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Err(ref mut s) => s.take().map(Err),
            Self::Iter(ref mut iter) => iter.next().map(Ok),
        }
    }
}

fn fetch_parse_errno<'a, I>(version: &'a str, paths: I) -> impl Iterator<Item = Result<Errno>> + 'a
where
    I: IntoIterator<Item = &'a str> + 'a,
{
    fetch_errno(version, paths).flat_map(|x| match x {
        Ok(file) => X::Iter(parse_file(file)),
        Err(err) => X::Err(Some(err)),
    })
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let version = 'kversion: {
        let releases: Releases = serde_json::from_reader(
            ureq::get("https://www.kernel.org/releases.json")
                .call()
                .wrap_err("Failed to fetch releases.json")?
                .into_reader(),
        )
        .wrap_err("Failed to parse releases.json")?;
        for release in releases.releases {
            if release.moniker == "mainline" {
                break 'kversion Some(release.version);
            }
        }
        None
    };
    let version = if let Some(version) = version {
        version.into_boxed_str()
    } else {
        bail!("mainline kernel version not found");
    };

    let mut defs = HashMap::new();
    let mut aliases = HashMap::new();
    for err in fetch_parse_errno(
        &version,
        [
            "include/uapi/asm-generic/errno-base.h",
            "include/uapi/asm-generic/errno.h",
            "include/linux/errno.h",
        ],
    ) {
        match err? {
            Errno::Define(name, value, desc) => {
                let desc = if let Some(desc) = desc {
                    desc
                } else {
                    match name.as_ref() {
                        "ERESTARTSYS" => "Restart syscall",
                        "ERESTARTNOINTR" => "Restart if no interrupt",
                        _ => bail!("No description for {name}"),
                    }
                    .to_string()
                    .into_boxed_str()
                };

                match defs.insert(name, (value, desc)) {
                    Some((other, _)) if other != value => {
                        bail!("{} defined multiple times", other)
                    }
                    _ => (),
                }
            }
            Errno::Alias(alias, original, _desc) => {
                // if let Some(desc) = desc {
                //     println!("{alias} = {desc}");
                // }
                aliases.insert(alias, original);
            }
        }
    }

    println!("#![allow(dead_code)]");
    println!();
    println!("// This file automatically generate. Do not edit.");
    println!();
    println!("use super::Errno;");
    println!();

    println!("impl Errno {{");
    for (name, (no, desc)) in &defs {
        println!("    /// {}", desc);
        println!("    pub const {}: Self = Self({});", name, no);
    }
    for (alias, name) in aliases {
        if defs.contains_key(name.as_ref()) {
            println!("    pub const {}: Self = Self::{};", alias, name);
        } else {
            bail!("Cannot find alias for {}: {}", alias, name);
        }
    }
    println!();
    println!(
        "    pub(crate) fn name_and_description(&self) -> Option<(&'static str, &'static str)> {{"
    );
    println!("        match self.0 {{");
    for (name, (no, desc)) in defs {
        println!("            {} => Some(({:?}, {:?})),", no, name, desc);
    }
    println!("            _ => None,");
    println!("        }}");
    println!("    }}");
    println!("}}");

    Ok(())
}
