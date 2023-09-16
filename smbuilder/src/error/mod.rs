mod cause;

pub use cause::*;
use colored::Colorize;
use std::fmt;

/// Error macros to shortuct the creation of error types.
pub mod macros {
    pub use super::cause::{c_comp_failed, c_copy_rom, c_other, c_repo_clone};
    #[allow(unused_imports)]
    use super::Error;

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
}

type AnyError = Box<dyn std::error::Error>;

#[derive(Debug)]
/// An smbuilder-related error.
pub struct Error {
    /// The type of error.
    pub cause: ErrorCause,

    /// The description of the error.
    pub description: Option<String>,
}

fn format_any_error(e: &Option<AnyError>) -> String {
    match e {
        None => "".to_owned(),
        Some(e) => format!(": ({})", e),
    }
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
                format_any_error(ctx)
            ),
            C::CopyRom { from, to, ctx } => write!(
                f,
                "whilst trying to copy a ROM from {} to {}{}",
                from.display(),
                to.display(),
                format_any_error(ctx)
            ),
            C::CompilationFailed { msg } => write!(f, "compilation failed: {}", msg),
            C::Other { ctx } => write!(f, "an unexpected error occured{}", format_any_error(ctx),),
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
