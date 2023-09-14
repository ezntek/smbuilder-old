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

/// A struct to store callbacks
/// to hook various events during
/// the build lifecycle to various
/// functions.
///
// TODO: example
pub struct Callbacks<'cb> {
    /// The log callback.
    pub log_cb: Option<Box<LogCb<'cb>>>,
    /// A callback that is invoked
    /// on a new setup stage.
    pub new_setup_stage_cb: Option<Box<NewSetupStageCb<'cb>>>,
    /// A callback that is invoked
    /// on a new post-build stage.
    pub new_postbuild_stage_cb: Option<Box<NewPostBuildStageCb<'cb>>>,
    /// A callback that is invoked when
    /// a new post-build script is being
    /// run.
    pub new_postbuild_script_cb: Option<Box<NewPostBuildScriptCb<'cb>>>,
    /// A callback that pipes information
    /// from git2's `RemoteCallbacks` to
    /// provide info on clone progress.
    pub repo_clone_progress_cb: Option<Box<RepoCloneProgressCb<'cb>>>,
}

impl<'cb> Callbacks<'cb> {
    /// Create an empty callbacks set.
    pub fn new() -> Self {
        Callbacks {
            log_cb: None,
            new_setup_stage_cb: None,
            new_postbuild_stage_cb: None,
            new_postbuild_script_cb: None,
            repo_clone_progress_cb: None,
        }
    }

    /// Set the log callback.
    ///
    /// See the docs on `[LogCb]`
    /// for more information on arguments.
    ///
    // TODO: example
    pub fn log<F>(mut self, callback: F) -> Self
    where
        F: FnMut(LogType, &str) + 'cb,
    {
        self.log_cb = Some(Box::new(callback) as Box<LogCb<'cb>>);
        self
    }

    /// Set the new setup stage
    /// callback.
    ///
    /// See the docs on `[NewSetupStageCb]`
    /// for more information on arguments.
    ///
    // TODO: example.
    pub fn new_setup_stage<F>(mut self, callback: F) -> Self
    where
        F: FnMut(SetupStage) + 'cb,
    {
        self.new_setup_stage_cb = Some(Box::new(callback) as Box<NewSetupStageCb<'cb>>);
        self
    }

    /// Set the new post-build stage
    /// callback.
    ///
    /// See the docs on `[NewPostBuildStageCb]`
    /// for more information on arguments.
    pub fn new_postbuild_stage<F>(mut self, callback: F) -> Self
    where
        F: FnMut(PostBuildStage) + 'cb,
    {
        self.new_postbuild_stage_cb = Some(Box::new(callback) as Box<NewPostBuildStageCb<'cb>>);
        self
    }

    /// Set the repo clone progress
    /// callback.
    ///
    /// See the docs on `RepoCloneProgressCb`
    /// for more information on arguments.
    ///
    // TODO: example
    pub fn repo_clone_progress<F>(mut self, callback: F) -> Self
    where
        F: FnMut(usize, usize, usize) + 'cb,
    {
        self.repo_clone_progress_cb = Some(Box::new(callback) as Box<RepoCloneProgressCb<'cb>>);
        self
    }

    /// Set the new post-build script
    /// callback.
    ///
    /// See the docs on `[NewPostBuildScriptCb]`
    /// for more information on arguments.
    ///
    // TODO: example
    pub fn new_postbuild_script<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&str, &str) + 'cb,
    {
        self.new_postbuild_script_cb = Some(Box::new(callback) as Box<NewPostBuildScriptCb<'cb>>);
        self
    }
}
