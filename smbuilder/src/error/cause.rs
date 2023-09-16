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
    /// Indicates any error that relates to the FS.
    Filesystem {
        /// Any related message
        msg: Option<String>,
        /// Cause
        ctx: AnyError,
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
