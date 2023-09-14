use crate::prelude::builder_types::{PostBuildStage, SetupStage};

/// Callback for a log event.
///
/// Args:
///  * log type (error, warning, info, etc.)
///  * text to log
pub type LogCb<'cb> = dyn FnMut(LogType, &str) + Send + Sync + 'cb;

/// Callback for a new setup stage.
///
/// Args:
///  * setup stage
pub type NewSetupStageCb<'cb> = dyn FnMut(SetupStage) + Send + Sync + 'cb;

/// Callback for a new post-build stage.
///
/// Args:
///  * post-build stage
pub type NewPostBuildStageCb<'cb> = dyn FnMut(PostBuildStage) + Send + Sync + 'cb;

/// Callback for when a new Post-Build script is run.
///
/// Args:
///  * filename of the script
///  * description of the script
pub type NewPostBuildScriptCb<'cb> = dyn FnMut(&str, &str) + Send + Sync + 'cb;

/// Callback for repository clone progress.
///
/// Args:
///  * recieved objects
///  * total objects
///  * recieved bytes
pub type RepoCloneProgressCb<'cb> = dyn FnMut(usize, usize, usize) + Send + Sync + 'cb;

/// An enum to represent
/// a log type, for the
/// log callback.
pub enum LogType {
    /// Indicates an error.
    Error,
    /// Indicates a warning.
    Warn,
    /// Indicates some
    /// build output.
    BuildOutput,
    /// Indicates some
    /// info.
    Info,
}
