mod cause;

pub use cause::*;
use colored::Colorize;
use std::fmt;

/// Error macros to shortuct the creation of error types.
pub mod macros;

type AnyError = Box<dyn std::error::Error>;

#[derive(Debug)]
/// An smbuilder-related error.
pub struct Error {
    /// The type of error.
    pub cause: ErrorCause,

    /// The description of the error.
    pub description: Option<String>,
}

macro_rules! fmt_anyerr {
    ($e:expr) => {
        match $e {
            None => "".to_owned(),
            Some(err) => format!(": ({})", err),
        }
    };
}

impl fmt::Display for ErrorCause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ErrorCause as C;
        match self {
            C::RepoClone { url, dir, ctx } => write!(
                f,
                "whilst trying to clone from {} to {}{}",
                url,
                dir.display(),
                fmt_anyerr!(ctx)
            ),
            C::Filesystem { msg, ctx } => {
                write!(
                    f,
                    "whilst working with the filesystem{} ({})",
                    ctx,
                    msg.clone().unwrap_or(String::new()),
                )
            }
            C::LaunchCmdError { cmd, msg, ctx } => {
                write!(
                    f,
                    "launching the command `{}` failed{} ({})",
                    cmd,
                    fmt_anyerr!(ctx),
                    msg.clone().unwrap_or(String::new())
                )
            }
            C::CompilationFailed { msg } => write!(f, "compilation failed: {}", msg),
            C::Other { ctx } => write!(f, "an unexpected error occured{}", fmt_anyerr!(ctx),),
        }
    }
}

impl Error {
    /// Creates a new `SmbuilderError`.
    pub fn new(cause: ErrorCause, description: Option<String>) -> Self {
        Error { cause, description }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let displayed_string = if let Some(d) = &self.description {
            format!("{}{}: {}", "error: ".bold().red(), self.cause, d)
        } else {
            format!("{}{}", "error: ".bold().red(), self.cause)
        };

        write!(f, "{}", displayed_string)
    }
}

impl std::error::Error for Error {}
