/// Houses callback types.
pub mod types;

use crate::builder_types::{PostBuildStage, SetupStage};
use types::*;

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

pub(crate) use run_callback;

/// A struct to store callbacks to hook various events
/// during the build process to various functions.
///
// TODO: example
pub struct Callbacks<'cb> {
    /// The log callback.
    ///
    /// Args:
    ///  * log type (error, warning, info, etc.)
    ///  * text to log
    pub log: Option<Box<Log<'cb>>>,

    /// A callback that is invoked on a new setup stage.
    ///
    /// Args:
    ///  * setup stage
    pub new_setup_stage: Option<Box<NewSetupStage<'cb>>>,

    /// A callback that is invoked on a new post-build stage.
    ///
    /// Args:
    ///  * post-build stage
    pub new_postbuild_stage: Option<Box<NewPostBuildStage<'cb>>>,

    /// A callback that is invoked when a new post-build script
    /// is being run.
    ///
    /// Args:
    ///  * filename of the script
    ///  * description of the script
    pub new_postbuild_script: Option<Box<NewPostBuildScript<'cb>>>,

    /// A callback that pipes information from git2's `RemoteCallbacks`
    /// to provide info on clone progress.
    ///
    /// Args:
    ///  * recieved objects
    ///  * total objects
    ///  * bytes transferred
    pub repo_clone_progress: Option<Box<RepoCloneProgress<'cb>>>,

    /// A callback that reports information on the progress of
    /// filesystem operations, particularly data transfer.
    ///
    /// Args:
    ///  * bytes transferred
    ///  * total byte count
    pub fs_operation_progress: Option<Box<FsOperationProgress<'cb>>>,
}

impl<'cb> Callbacks<'cb> {
    /// Create an empty callbacks set.
    pub fn new() -> Self {
        Callbacks {
            log: None,
            new_setup_stage: None,
            new_postbuild_stage: None,
            new_postbuild_script: None,
            repo_clone_progress: None,
            fs_operation_progress: None,
        }
    }

    /// Set the log callback.
    ///
    /// See the docs on [Log]
    /// for more information on arguments.
    ///
    // TODO: example
    pub fn log<F>(mut self, callback: F) -> Self
    where
        F: FnMut(LogType, &str) + Send + Sync + 'cb,
    {
        self.log = Some(Box::new(callback) as Box<Log<'cb>>);
        self
    }

    /// Set the new setup stage
    /// callback.
    ///
    /// See the docs on [NewSetupStage]
    /// for more information on arguments.
    ///
    // TODO: example.
    pub fn new_setup_stage<F>(mut self, callback: F) -> Self
    where
        F: FnMut(SetupStage) + Send + Sync + 'cb,
    {
        self.new_setup_stage = Some(Box::new(callback) as Box<NewSetupStage<'cb>>);
        self
    }

    /// Set the new post-build stage
    /// callback.
    ///
    /// See the docs on [NewPostBuildStage]
    /// for more information on arguments.
    ///
    // TODO: example.
    pub fn new_postbuild_stage<F>(mut self, callback: F) -> Self
    where
        F: FnMut(PostBuildStage) + Send + Sync + 'cb,
    {
        self.new_postbuild_stage = Some(Box::new(callback) as Box<NewPostBuildStage<'cb>>);
        self
    }

    /// Set the repo clone progress
    /// callback.
    ///
    /// See the docs on [RepoCloneProgress]
    /// for more information on arguments.
    ///
    // TODO: example
    pub fn repo_clone_progress<F>(mut self, callback: F) -> Self
    where
        F: FnMut(usize, usize, usize) + Send + Sync + 'cb,
    {
        self.repo_clone_progress = Some(Box::new(callback) as Box<RepoCloneProgress<'cb>>);
        self
    }

    /// Set the new post-build script
    /// callback.
    ///
    /// See the docs on [NewPostBuildScript]
    /// for more information on arguments.
    ///
    // TODO: example
    pub fn new_postbuild_script<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&str, &str) + Send + Sync + 'cb,
    {
        self.new_postbuild_script = Some(Box::new(callback) as Box<NewPostBuildScript<'cb>>);
        self
    }

    /// Set the file system operation progress
    /// callback.
    ///
    /// See the docs on [FsOperationProgress] for
    /// more information on arguments
    ///
    pub fn fs_operation_progress<F>(mut self, callback: F) -> Self
    where
        F: FnMut(u64, u64) + Send + Sync + 'cb,
    {
        self.fs_operation_progress = Some(Box::new(callback) as Box<FsOperationProgress<'cb>>);
        self
    }
}
