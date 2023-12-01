pub use super::cause::*;
#[allow(unused_imports)]
use super::Error;

#[macro_export]
/// Repo Clone error cause.
///
/// Rules:
///  * `url: String, dir: PathBuf`
///  * same as above but with `ctx: impl std::error::Error`
macro_rules! err_variant_repo_clone {
    ($url:expr, $dir:expr) => {
        crate::error::cause::ErrorCause::RepoClone {
            url: $url,
            dir: $dir,
            ctx: None,
        }
    };

    ($url:expr, $dir:expr, $ctx:expr) => {
        crate::error::cause::ErrorCause::RepoClone {
            url: $url,
            dir: $dir,
            ctx: Some(Box::new($ctx)),
        }
    };
}

#[macro_export]
/// Compilation failed error cause
///
/// `msg: String`
macro_rules! err_variant_comp_failed {
    ($msg:expr) => {
        ErrorCause::CompilationFailed { msg: $msg }
    };
}

#[macro_export]
/// Generic error cause
///
/// `ctx: impl std::error::Error`
macro_rules! err_generic {
    ($ctx:expr) => {
        ErrorCause::Other {
            ctx: Some(Box::new($ctx)),
        }
    };
}

/// A filesystem error.
///
/// Rules:
///  * `ctx: std::io::Error`
///  * `ctx: std::io::Error, msg: String`
#[macro_export]
macro_rules! err_variant_fs {
    ($ctx:expr) => {
        crate::error::cause::ErrorCause::Filesystem {
            msg: None,
            ctx: Box::new($ctx),
        }
    };

    ($ctx:expr,$msg:expr) => {
        crate::error::cause::ErrorCause::Filesystem {
            msg: Some($msg.to_string()),
            ctx: Box::new($ctx),
        }
    };
}

#[macro_export]
/// A command-spawning error.
///
/// Rules:
///  * `cmd: String`
///  * `cmd: String, ctx: impl std::error::Error`
///  * `cmd: String, msg: String, ctx: impl std::error::Error`
macro_rules! err_variant_cmdlaunch {
    ($cmd:expr) => {
        ErrorCause::LaunchCmdError {
            cmd: $cmd,
            msg: None,
            ctx: None,
        }
    };

    ($cmd:expr,$ctx:expr) => {
        ErrorCause::LaunchCmdError {
            cmd: $cmd,
            msg: None,
            ctx: Some($ctx),
        }
    };

    ($cmd:expr,$msg:expr,$ctx:expr) => {
        ErrorCause::LaunchCmdError {
            cmd: $cmd,
            msg: Some($msg.to_string()),
            ctx: Some(Box::new($ctx)),
        }
    };
}

pub use {
    err_generic, err_variant_cmdlaunch, err_variant_comp_failed, err_variant_fs,
    err_variant_repo_clone,
};

#[macro_export]
/// Instantiate an Error struct.
///
/// Variants:
///  * `cause: ErrorCause` (can be used with `err_` macros)
///  * same as above, but with `desc: impl ToString`
macro_rules! err {
    ($cause:expr) => {
        crate::error::Error::new($cause, None)
    };

    ($cause:expr, $desc:expr) => {
        crate::error::Error::new($cause, Some($desc.to_string()))
    };
}

pub use err;
