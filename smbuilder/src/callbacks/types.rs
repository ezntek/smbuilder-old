use crate::prelude::builder_types::{PostBuildStage, SetupStage};

/// Callback for a log event.
///
/// Args:
///  * log type (error, warning, info, etc.)
///  * text to log
pub type Log<'cb> = dyn FnMut(LogType, &str) + Send + Sync + 'cb;

/// Callback for a new setup stage.
///
/// Args:
///  * setup stage
pub type NewSetupStage<'cb> = dyn FnMut(SetupStage) + Send + Sync + 'cb;

/// Callback for a new post-build stage.
///
/// Args:
///  * post-build stage
pub type NewPostBuildStage<'cb> = dyn FnMut(PostBuildStage) + Send + Sync + 'cb;

/// Callback for when a new Post-Build script is run.
///
/// Args:
///  * filename of the script
///  * description of the script
pub type NewPostBuildScript<'cb> = dyn FnMut(&str, &str) + Send + Sync + 'cb;

/// Callback for repository clone progress.
///
/// Args:
///  * recieved objects
///  * total objects
///  * bytes transferred
pub type RepoCloneProgress<'cb> = dyn FnMut(usize, usize, usize) + Send + Sync + 'cb;

/// Callback for any filesystem operation involving the transfer
/// of data.
///
/// Args:
///  * bytes transferred
///  * total bytes to transfer
pub type FsOperationProgress<'cb> = dyn FnMut(u64, u64) + Send + Sync + 'cb;

/// An enum to represent
/// a log type, for the
/// log callback.
pub enum LogType {
    /// Indicates an error.
    Error,
    /// Indicates a warning.
    Warn,
    /// Indicates build output.
    BuildOutput,
    /// Indicates information
    Info,
}
