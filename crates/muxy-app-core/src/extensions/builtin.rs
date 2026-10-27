//! Codesh ships these extensions inside the app. Boot copies them into the
//! profile, where they are always enabled and never prompt for permissions.

use std::fs;
use std::io;
use std::path::{Component, Path};

pub const BUILTINS: [&str; 2] = ["files", "git"];

/// Names the bundle a built-in folder was copied from.
const STAMP: &str = ".codesh-builtin";
/// The runtime log each extension writes inside its own folder.
const LOG: &str = "logs/output.log";

pub fn is_builtin(name: &str) -> bool {
    BUILTINS.contains(&name)
}

/// Installs each built-in from `bundle` (paths `<name>/<file>`) into
/// `<profile>/extensions/<name>`, unless that folder already carries `stamp`.
/// Each folder is staged beside its target and swapped in whole, keeping the
/// previous log; leftovers from an interrupted attempt are discarded first.
pub fn seed(profile: &Path, bundle: &[(&str, &[u8])], stamp: &str) -> io::Result<()> {
    let root = profile.join("extensions");
    fs::create_dir_all(&root)?;
    for name in BUILTINS {
        let target = root.join(name);
        if fs::read_to_string(target.join(STAMP)).is_ok_and(|current| current == stamp) {
            continue;
        }
        let stage = root.join(format!(".codesh-stage-{name}"));
        let old = root.join(format!(".codesh-old-{name}"));
        remove(&stage)?;
        let prefix = format!("{name}/");
        let mut written = false;
        for (path, bytes) in bundle {
            let Some(relative) = path.strip_prefix(&prefix) else {
                continue;
            };
            let relative = Path::new(relative);
            if !relative
                .components()
                .all(|component| matches!(component, Component::Normal(_)))
            {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("{path}: invalid built-in extension path"),
                ));
            }
            let file = stage.join(relative);
            if let Some(parent) = file.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(file, bytes)?;
            written = true;
        }
        if !written {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("{name}: missing from the built-in bundle"),
            ));
        }
        // A crash mid-swap leaves the previous folder only under `old`.
        if let Some(log) = [target.join(LOG), old.join(LOG)]
            .into_iter()
            .find(|log| log.is_file())
        {
            fs::create_dir_all(stage.join("logs"))?;
            fs::copy(log, stage.join(LOG))?;
        }
        fs::write(stage.join(STAMP), stamp)?;
        remove(&old)?;
        if let Err(error) = fs::rename(&target, &old)
            && error.kind() != io::ErrorKind::NotFound
        {
            return Err(error);
        }
        if let Err(error) = fs::rename(&stage, &target) {
            let _ = fs::rename(&old, &target);
            return Err(error);
        }
        let _ = remove(&old);
    }
    Ok(())
}

/// Deletes a file or folder, if there is one.
fn remove(path: &Path) -> io::Result<()> {
    let result = match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.is_dir() => fs::remove_dir_all(path),
        Ok(_) => fs::remove_file(path),
        Err(error) => Err(error),
    };
    match result {
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        result => result,
    }
}
