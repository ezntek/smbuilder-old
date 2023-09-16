use colored::Colorize;
use std::fmt;
use std::path::PathBuf;

type AnyError = Box<dyn std::error::Error>;

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
        message: &'static str,
    },

    /// An error that doesnt apply to any of the variants
    Other {
        /// Context (cause, if any)
        ctx: Option<AnyError>,
    },
}

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
            C::CompilationFailed { message } => write!(f, "compilation failed: {}", message),
            C::Other { ctx } => write!(f, "an unexpected error occured{}", format_any_error(ctx)),
        }
    }
}

impl Error {
    /// Creates a new `SmbuilderError`.
    pub fn new<S: ToString>(cause: ErrorCause, description: Option<S>) -> Self {
        Error {
            cause,
            description: if let Some(s) = description {
                Some(s.to_string())
            } else {
                None
            },
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let displayed_string = if let Some(d) = self.description {
            format!("{}{}: {}", "error: ".bold().red(), self.cause, d)
        } else {
            format!("{}{}", "error: ".bold().red(), self.cause)
        };

        write!(f, "{}", displayed_string)
    }
}

impl std::error::Error for Error {}
