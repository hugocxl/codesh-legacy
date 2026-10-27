use std::env;
use std::fmt::Write as _;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Embeds every built-in extension's build output, keyed by its path below
/// `builtin-extensions/`, with a hash of the whole bundle to stamp installs.
fn main() -> io::Result<()> {
    let output = env::var_os("OUT_DIR").ok_or_else(|| io::Error::other("OUT_DIR is missing"))?;
    let root = Path::new("builtin-extensions");
    let mut files = Vec::new();
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let name = entry.file_name();
        if entry.file_type()?.is_dir() && !name.to_string_lossy().starts_with('.') {
            collect(&entry.path(), &utf8(&name)?, &mut files)?;
        }
    }
    files.sort();
    let mut hash = Fnv::default();
    let mut source = String::from("static BUILTIN_EXTENSIONS: &[(&str, &[u8])] = &[\n");
    for key in &files {
        let bytes = fs::read(root.join(key))?;
        hash.write(key.as_bytes());
        hash.write(&[0]);
        hash.write(&(bytes.len() as u64).to_le_bytes());
        hash.write(&bytes);
        writeln!(
            source,
            "({key:?}, include_bytes!(concat!(env!(\"CARGO_MANIFEST_DIR\"), {path:?}))),",
            path = format!("/builtin-extensions/{key}")
        )
        .map_err(io::Error::other)?;
    }
    source.push_str("];\n");
    writeln!(
        source,
        "const BUILTIN_EXTENSIONS_HASH: &str = \"{:016x}\";",
        hash.0
    )
    .map_err(io::Error::other)?;
    fs::write(PathBuf::from(output).join("builtin_extensions.rs"), source)?;
    writeln!(io::stdout(), "cargo:rerun-if-changed=builtin-extensions")
}

/// Adds the files below `directory` as `/`-separated keys, skipping dot files
/// and each extension's runtime `logs/` folder.
fn collect(directory: &Path, key: &str, files: &mut Vec<String>) -> io::Result<()> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let name = utf8(&entry.file_name())?;
        let kind = entry.file_type()?;
        if name.starts_with('.') || (kind.is_dir() && name == "logs" && !key.contains('/')) {
            continue;
        }
        let child = format!("{key}/{name}");
        if kind.is_dir() {
            collect(&entry.path(), &child, files)?;
        } else if kind.is_file() {
            files.push(child);
        } else {
            return Err(io::Error::other(format!("{child}: not a regular file")));
        }
    }
    Ok(())
}

fn utf8(name: &std::ffi::OsStr) -> io::Result<String> {
    name.to_str()
        .map(str::to_owned)
        .ok_or_else(|| io::Error::other("non-UTF-8 built-in extension path"))
}

/// 64-bit FNV-1a: stable across toolchains, unlike `DefaultHasher`.
struct Fnv(u64);

impl Default for Fnv {
    fn default() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }
}

impl Fnv {
    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 = (self.0 ^ u64::from(*byte)).wrapping_mul(0x0100_0000_01b3);
        }
    }
}
