use crate::prelude::Error;

/// Error type.
///
/// Aliases a `Result<_, SmbuilderError>` to
/// something more sensible.
pub type BuilderResult<T> = Result<T, Error>;

#[derive(Debug)]
/// An enum to represent the different "setup stages"
/// involved in building a port.
///
/// It includes critical steps to be able to
/// build a basic, vanilla port.
pub enum SetupStage {
    /// Clone the repository from
    /// the spec.
    CloneRepo,

    /// Copy the base ROM (and converts
    /// its format, if necessary) into
    /// the repo's root for asset extraction.
    CopyRom,

    /// Create the build script.
    CreateBuildScript,

    /// Create the post-build scripts directory,
    CreateScriptsDir,

    /// Write the Post-build scripts to disk.
    WritePostBuildScripts,
}

#[derive(Debug)]
/// An enum to represent the different post-build
/// stages involved in building a port.
///
/// Represents actions such as installing model
/// packs and texture packs, running scripts, etc.
pub enum PostBuildStage {
    /// Install the texture pack
    TexturePack,
    /// Install the DynOS packs(s)
    DynOSPacks,
    /// Run the Post-Build scripts
    PostBuildScripts,
}

impl ToString for SetupStage {
    fn to_string(&self) -> String {
        use SetupStage::*;

        let result = match self {
            CloneRepo => "clone the repository",
            CopyRom => "copy the base ROM",
            CreateBuildScript => "create the build script",
            CreateScriptsDir => "create the post-build script folder",
            WritePostBuildScripts => "write the post-build scripts",
        };

        result.to_owned()
    }
}

impl ToString for PostBuildStage {
    fn to_string(&self) -> String {
        use PostBuildStage::*;

        let result = match self {
            TexturePack => "install the texture pack",
            DynOSPacks => "install the DynOS pack(s)",
            PostBuildScripts => "run the post-build script(s)",
        };

        result.to_owned()
    }
}
