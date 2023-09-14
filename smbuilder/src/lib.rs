#![warn(missing_docs)]

//! # smbuilder -- API documentation
//!
//! smbuilder is a small rust crate that provides an interface for the compilation of various ports within the family of ports of Super Mario 64 to the PC.
//!
//! The crate:
//!  * Provides strong types and models that wraps around the resources needed and other "moving parts" (texture packs, etc) for a given port
//!  * Allows for the types to be serialized/deserialized into/from yaml for reproducible build specifications (specs)
//!  * Provides classes and traits to provide an interface to build specs.
//!
//! ## Notes
//!
//! * This crate is not quite production quality yet. breaking API changes may come sooner or later.
//! * The bindings of these resources/moving parts for a build cannot be fully complete without actually modifying the port. However, more bindings may be added later.
//! * The choice of repositories and the makeopts supported by those ports should be handled by the app that uses this crate. However, makeopts may be implemented as enums/structs in a later version or in another crate.
//!
//! ## Usage
//!
//! WIP
// TODO: whatever the hell this is

/// The API and rust representation(s)
/// of core build processes that are
/// involved in building a port.
pub mod builder;

/// All the logic and code
/// that relates to the
/// reproducible spec.
pub mod spec;

/// The prelude of this crate.
pub mod prelude;

/// Core types that binds common
/// build resources to rust types.
pub mod types;

/// Build progress callbacks.
pub mod callbacks;

use prelude::*;
use std::{fs, os::unix::prelude::PermissionsExt, path::Path};

#[macro_export]
/// A macro to make writing
/// a makeopt less painful.
///
// TODO: example
macro_rules! makeopt {
    ($key:expr, $value:expr) => {
        Makeopt::new($key, $value)
    };
}

#[macro_export]
/// Run a `Callbacks`
/// callback, if it exists.
///
/// Does not panic if it
/// does not exist. You
/// also do need a mutable
/// reference to the callback,
/// as there are `FnMut`s.
///
// TODO: example
macro_rules! run_callback {
    ($callback:expr, $($cb_arg:tt)*) => {
        if let Some(callback) = &mut $callback {
            callback($($cb_arg)*);
        };
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
