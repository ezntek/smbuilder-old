use super::AnyError;
use std::path::PathBuf;

#[derive(Debug)]
/// Possible causes for an error
pub enum ErrorCause {
    /// Indicates a failure in cloning a repo.
    RepoClone {
        /// the URL of the repo being cloned.
        url: String,
        /// The target directory of the clone.
        dir: PathBuf,
        /// Context (possible cause)
        ctx: Option<AnyError>,
    },
    /// Indicates a failure when copying the ROM.
    CopyRom {
        /// The path of the rom
        from: PathBuf,
        /// The new path of the rom.
        to: PathBuf,
        /// Context (possible cause)
        ctx: Option<AnyError>,
    },
    /// Indicates a failure in running the build command.
    ///
    /// the `duct` crate does not provide exit status
    /// codes when running with `stderr_to_stdout`, which
    /// is why little context can be provided.
    CompilationFailed {
        /// The message that the program would like to give.
        msg: &'static str,
    },

    /// An error that doesnt apply to any of the variants
    Other {
        /// Context (cause, if any)
        ctx: Option<AnyError>,
    },
}

#[macro_export]
/// Repo Clone error cause.
///
/// Rules:
///  * `url: String, dir: PathBuf`
///  * same as above but with `ctx: impl std::error::Error`
macro_rules! c_repo_clone {
    ($url:expr, $dir:expr) => {
        ErrorCause::RepoClone {
            url: $url,
            dir: $dir,
            ctx: None,
        }
    };

    ($url:expr, $dir:expr, $ctx:expr) => {
        ErrorCause::RepoClone {
            url: $url,
            dir: $dir,
            ctx: Some(Box::new($ctx)),
        }
    };
}

#[macro_export]
/// Rom Copy error cause
///
// TODO: Document
macro_rules! c_copy_rom {
    ($from:expr, $to:expr) => {
        ErrorCause::CopyRom {
            from: $from,
            to: $to,
            ctx: None,
        }
    };

    ($from:expr, $to:expr, $ctx:expr) => {
        ErrorCause::CopyRom {
            from: $from,
            to: $to,
            ctx: Some(Box::new($ctx)),
        }
    };
}

#[macro_export]
/// Compilation failed error cause
///
/// `msg: String`
macro_rules! c_comp_failed {
    ($msg:expr) => {
        ErrorCause::CompilationFailed { msg: $msg }
    };
}

#[macro_export]
/// Generic error cause
///
/// `ctx: impl std::error::Error`
macro_rules! c_other {
    ($ctx:expr) => {
        ErrorCause::Other {
            ctx: Some(Box::new($ctx)),
        }
    };
}

pub use {c_comp_failed, c_copy_rom, c_other, c_repo_clone};
