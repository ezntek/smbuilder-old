use crate::prelude::*;
use std::{fs, os::unix::prelude::PermissionsExt, path::Path};

#[macro_export]
macro_rules! try_or_return {
    ($result:expr, $or:expr) => {
        match $result {
            Ok(r) => r,
            Err(e) => return Err($or),
        }
    };
}

/// Get a string of options in the format of
/// bourne shell variables from a list of `makeopt`,
/// for use with the `make` command.
pub fn get_makeopts_string(makeopts: &[Makeopt]) -> String {
    let mut result = makeopts
        .iter()
        .map(|makeopt| format!("{}={}", makeopt.key, makeopt.value))
        .collect::<Vec<String>>()
        .join(" ");

    result.push(' '); // pad the last character out, just in case
    result
}

/// Make a file executable.
/// Equivalent to `chmod +x`.
pub fn make_file_executable(path: &Path) {
    let file_metadata = fs::metadata(path).unwrap_or_else(|e| {
        panic!(
            "failed to get the metadata of the file at {}: {}",
            &path.display(),
            e
        )
    });

    fs::set_permissions(
        path,
        fs::Permissions::from_mode(
            file_metadata.permissions().mode() + 0o111, // equivalent of a chmod +x.
        ),
    )
    .unwrap_or_else(|e| {
        panic!(
            "failed to set permissions on the file at {}: {}",
            &path.display(),
            e
        )
    });
}
