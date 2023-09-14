/// Re-export functions from the `n64romconvert` crate.
pub mod romconvert {
    pub use n64romconvert::{byte_endian_swap, byte_swap, determine_format, endian_swap, RomType};
}

// Builder stuff
pub use crate::builder::builder::Builder;
pub use crate::builder::types as builder_types;

// callbacks
pub use crate::callbacks::types as callback_types;
pub use crate::callbacks::*;

// spec
pub use crate::spec::*;

// core types
pub use crate::types::*;

// other stuff
pub use crate::{get_makeopts_string, make_file_executable, run_callback};
pub use errors::SmbuilderError;
pub use romconvert::*;
