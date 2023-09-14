use colored::Colorize;
use std::fmt;

#[derive(Debug)]
/// An smbuilder-related error.
pub struct SmbuilderError {
    /// The cause of the error.
    pub cause: Option<Box<dyn std::error::Error>>,

    /// The description of the error.
    pub description: String,
}

impl SmbuilderError {
    /// Creates a new `SmbuilderError`.
    pub fn new<S: AsRef<str>>(cause: Option<Box<dyn std::error::Error>>, description: S) -> Self {
        SmbuilderError {
            cause,
            description: description.as_ref().to_owned(),
        }
    }
}

impl fmt::Display for SmbuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let displayed_string = if let Some(e) = &self.cause {
            format!("{}{}: {}", "error: ".bold().red(), self.description, *e)
        } else {
            format!("{}{}", "error: ".bold().red(), self.description,)
        };

        write!(f, "{}", displayed_string)
    }
}

impl std::error::Error for SmbuilderError {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        if let Some(e) = &self.cause {
            Some(&**e)
        } else {
            None
        }
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
