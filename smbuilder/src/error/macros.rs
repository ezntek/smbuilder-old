pub use super::cause::*;
#[allow(unused_imports)]
use super::Error;

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

/// A filesystem error.
///
/// Rules:
///  * `ctx: std::io::Error`
///  * same as above but with `msg: String`
#[macro_export]
macro_rules! c_fs {
    ($ctx:expr) => {
        ErrorCause::Filesystem {
            msg: None,
            ctx: Box::new($ctx),
        }
    };

    ($ctx:expr,$msg:expr) => {
        ErrorCause::Filesystem {
            msg: Some($msg.to_string()),
            ctx: Box::new($ctx),
        }
    };
}

pub use {c_comp_failed, c_copy_rom, c_fs, c_other, c_repo_clone};

#[macro_export]
/// Instantiate an Error struct.
///
/// Variants:
///  * `cause: ErrorCause` (can be used with `c_` macros)
///  * same as above, but with `desc: impl ToString`
macro_rules! err {
    ($cause:expr) => {
        Error::new($cause, None)
    };

    ($cause:expr, $desc:expr) => {
        Error::new($cause, Some($desc.to_string()))
    };
}

pub use err;
