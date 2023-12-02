/// Enums related to some
/// common make flags that people
/// generally set.
pub mod makeopts;

use crate::prelude::builder_types::BuilderResult; //, Error*/};
use crate::{err_variant_fs, prelude::*};
use std::{
    fmt::Debug,
    fs,
    io::{/*self,*/ BufWriter, Write},
    //path::Path,
};

//use fs_extra::dir::CopyOptions;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
extern crate fs_extra;

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct CompilerOptions {
    /// Amount of compile jobs that are allowed for the compiler.
    ///
    /// This will be used to set the `-j` flag during compile time.
    pub jobs: Option<u8>,
    /// Make flags to be passed to the compiler.
    pub makeopts: Option<Vec<Makeopt>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
/// Represents the region of a given ROM file.
pub enum Region {
    #[default]
    /// A rom pulled from a US cartridge.
    Us,
    /// A rom pulled from a European cartridge (EU).
    Eu,
    /// A rom pulled from a Japanese cartridge.
    Jp,
    /// A rom pulled from a Japanese Shindou cartridge.
    Sh,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// Represents a ROM file.
pub struct RomData {
    /// The Region of the ROM Cartridge that
    /// the ROM was pulled from.
    pub region: Region,
    /// The path of the ROM file on disk.
    pub path: PathBuf,
    /// The format of the ROM file.
    pub format: RomType,
}

impl Default for RomData {
    fn default() -> Self {
        RomData {
            region: Region::Us,
            path: PathBuf::new(),
            format: RomType::BigEndian,
        }
    }
}

impl RomData {
    /// Creates a new `Rom`.
    ///
    // TODO: example
    pub fn new(region: Region, path: impl Into<PathBuf>, rom_format: RomType) -> Self {
        RomData {
            region,
            path: path.into(),
            format: rom_format,
        }
    }
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
/// Represents a git repository with the
/// source code of the a port.
pub struct RepoData {
    /// The name of the repository.
    ///
    /// Used for launchers where the name may need to be a little bit more user friendly.
    pub name: String,
    /// The link to the repository.
    pub url: String,
    /// The branch to clone from.
    pub branch: String,
    /// The description of what the
    /// repo is, useful for launchers.
    pub about: String,
    /// Does this repo support DynOS packs?
    pub supports_dynos: bool,
    /// Should the repository be shallow-cloned or not (--depth=1 or no?)
    pub deep_clone: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
/// Represents a key-value pair
/// Make Flag, such as `BETTERCAMERA=1`
pub struct Makeopt {
    /// The key of the flag.
    pub key: String,

    /// The value of the flag.
    pub value: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
/// Represents a patch.
pub struct Patch {
    /// The name (label) of
    /// the patch, for use
    /// with launchers,
    pub name: String,

    /// The location of the
    /// path file on disk.
    pub path: PathBuf,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PackData {
    pub texture: Option<TexturePack>,
    pub dynos: Option<Vec<DynosPack>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
/// Represents a Texture Pack.
pub struct TexturePack {
    /// The name (label) of
    /// the textuer pack,
    /// for use with launchers.
    pub name: String,

    /// The location of the
    /// texture pack on disk,
    pub path: PathBuf,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
/// A structure to represent
/// a DynOS (Dynamic Options
/// System) Datapack.
///
/// These packs can store
/// either just options
/// (sets of settings/options),
/// or other metadata with it
/// (how model packs work).
///
// TODO: example
pub struct DynosPack {
    /// The name of the
    /// DynOS pack, for
    /// use with launchers.
    pub name: String,

    /// The location of
    /// the pack, on disk.
    pub path: PathBuf,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
/// Represents a post build script
pub struct PostBuildScript {
    /// The name of the script,
    /// to be used as the file
    /// name, with a .sh appended.
    pub name: String,
    /// A human readable
    /// description of the
    /// script.
    pub description: String,
    /// The contents of the
    /// script, in shell format.
    pub contents: String,
    /// The path to the build
    /// script on disk. Will be
    /// `None` if the script
    /// has not been written to
    /// disk yet.
    pub path: Option<PathBuf>,
}

impl CompilerOptions {
    /// Creates a new [CompilerOptions] structure.
    pub fn new(jobs: Option<u8>, makeopts: Vec<Makeopt>) -> Self {
        Self {
            jobs,
            makeopts: Some(makeopts),
        }
    }
}

impl Makeopt {
    /// Creates a new [Makeopt].
    ///
    // TODO: example
    pub fn new<S: ToString>(key: S, value: S) -> Self {
        Makeopt {
            key: key.to_string(),
            value: value.to_string(),
        }
    }

    /// Gets a list of default makeopts with
    /// sane defaults, and options for the
    /// current OS.
    // TODO: example
    pub fn get_defaults() -> Vec<Self> {
        // convenience macro
        macro_rules! makeopt {
            ($key:expr,$value:expr) => {
                Makeopt::new($key, $value)
            };
        }

        let mut makeopts: Vec<Makeopt> = vec![
            // enable extrnal data
            makeopt!("EXTERNAL_DATA", "1"),
            // force modern graphics APIs
            makeopt!("RENDER_API", "GL"),
            makeopt!("WINDOW_API", "SDL2"),
            makeopt!("AUDIO_API", "SDL2"),
            makeopt!("CONTROLLER_API", "SDL2"),
        ];

        // macOS stuff
        #[cfg(target_os = "macos")]
        {
            makeopts.push(makeopt!("OSX_BUILD", "1"));
            makeopts.push(makeopt!("TARGET_BITS", "64"));

            #[cfg(target_arch = "x86_64")]
            makeopts.push(makeopt!("TARGET_ARCH", "x86_64-apple-darwin"));

            #[cfg(target_arch = "aarch64")]
            makeopts.push(makeopt!("TARGET_ARCH", "aarch64-apple-darwin"));
        };

        makeopts
    }
}

impl PostBuildScript {
    /// Creates a new `PostBuildScript`.
    pub fn new<S: ToString>(name: S, description: S, contents: S) -> Self {
        PostBuildScript {
            name: name.to_string(),
            description: description.to_string(),
            contents: contents.to_string(),
            path: None,
        }
    }
    /// Creates a post-build script from
    /// a `Path`.
    ///
    // TODO: example
    pub fn from_file(
        name: impl ToString,
        description: String,
        file: impl Into<PathBuf>,
    ) -> BuilderResult<Self> {
        let file = file.into();
        let file_contents = match fs::read_to_string(&file) {
            Ok(f) => f,
            Err(e) => {
                let err = err!(err_variant_fs!(
                    e,
                    format!("failed to read {}", file.display())
                ));
                return Err(err);
            }
        };

        let res = PostBuildScript {
            name: name.to_string(),
            description: description.to_string(),
            contents: file_contents,
            path: None,
        };
        Ok(res)
    }

    /// Saves a post-build script from
    /// a `String` in memory to a
    /// File path.
    ///
    // TODO: example
    pub fn save(&mut self, scripts_dir: impl Into<PathBuf>) -> BuilderResult<PathBuf> {
        let mut script_path = scripts_dir.into().join(&self.name);
        script_path.set_extension("sh");

        let script_file = match fs::File::create(&script_path) {
            Ok(f) => f,
            Err(e) => {
                let err = err!(
                    err_variant_fs!(
                        e,
                        format!("whilst trying to create {}", script_path.display())
                    ),
                    "failed to create the script file"
                );
                return Err(err);
            }
        };

        let mut script_file = BufWriter::new(script_file);

        match script_file.write_all(self.contents.as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                let err = err!(
                    err_variant_fs!(
                        e,
                        format!("whilst trying to write to {}", script_path.display())
                    ),
                    "failed to write to the script file"
                );
                return Err(err);
            }
        };

        self.path = Some(script_path.clone());
        Ok(script_path)
    }
}

impl DynosPack {
    /// Creates a new DynOS pack.
    ///
    // TODO: example
    pub fn new(name: impl ToString, path: impl Into<PathBuf>) -> Self {
        DynosPack {
            name: name.to_string(),
            path: path.into(),
        }
    }

    /*
    /// Installs the DynOS pack (copies it
    /// into the correct location)
    ///
    // TODO: example

    pub fn install(
        &self,
        spec: &Spec,
        repo_dir: impl AsRef<Path>,
        callbacks: &mut Callbacks,
    ) -> BuilderResult<()> {
        if !spec.repo.supports_dynos {
            run_callback!(
                callbacks.log_cb,
                types::LogType::Warn,
                "this build does not support DynOS packs. stopping."
            );
            return Ok(());
        }

        let target_path = repo_dir
            .as_ref()
            .join("build")
            .join(format!("{}_pc", spec.rom.region.to_string()))
            .join("dynos")
            .join("packs");

        match fs_extra::dir::copy(&self.path, &target_path, &CopyOptions::new()) {
            Ok(_) => (),
            Err(e) => {
                let msg = format!(
                    "whilst copying the DynOS pack from {} to {}: {}",
                    &self.path.display(),
                    &target_path.display(),
                    e
                );
                let err = err!(err_variant_fs!(e, msg), "failed to copy the DynOS pack");
                return Err(err);
            }
        };

        Ok(())
    }

    /// Permanently removes the pack
    /// from disk, effectively uninstalling
    /// it.
    pub fn remove<P: AsRef<Path>>(&self, spec: &Spec, repo_dir: P) {
        let pack_filename = self
            .path
            .iter()
            .last()
            .expect("the DynOS pack should have a filename!");

        let target_path = repo_dir
            .as_ref()
            .join("build")
            .join(format!("{}_pc", spec.rom.region.to_string()))
            .join("dynos")
            .join("packs")
            .join(pack_filename);

        fs_extra::dir::remove(target_path)
            .unwrap_or_else(|e| panic!("failed to remove the directory: {}", e));
    }*/
}

impl TexturePack {
    /// Creates a new TexturePack.
    ///
    // TODO: example
    pub fn new<S, P>(name: S, path: P) -> Self
    where
        S: ToString,
        P: Into<PathBuf>,
    {
        TexturePack {
            name: name.to_string(),
            path: path.into(),
        }
    }

    /*
    /// Installs the Texture pack (copies
    /// it into the correct location)
    ///
    // TODO: example

    pub fn install<P: AsRef<Path>>(&self, spec: &Spec, repo_dir: P) -> Result<(), Error> {
        let target_path = repo_dir
            .as_ref()
            .join("build")
            .join(format!("{}_pc", spec.rom.region.to_string()))
            .join("res");
        //.join("gfx")
        // {repo_dir}/build/{region}_pc/res/gfx

        let pack_path = &self.path.join("gfx");

        if !pack_path.exists() {
            let inner_err = io::Error::new(
                io::ErrorKind::NotFound,
                "could not find the gfx directory in the texture pack path!",
            );

            let err = err!(err_variant_fs!(inner_err), "invalid texture pack"); // TODO:
            return Err(err);
        };

        fs_extra::dir::copy(pack_path, &target_path, &CopyOptions::new()).unwrap_or_else(|e| {
            panic!(
                "failed to copy the texture pack from {} to {}: {}",
                &pack_path.display(),
                &target_path.display(),
                e
            )
        });

        Ok(())
    }

    /// Permanently removes the texture
    /// pack from disk, effectively
    /// uninstalling it.
    pub fn remove<P: AsRef<Path>>(&self, spec: &Spec, repo_dir: P) {
        let target_path = repo_dir
            .as_ref()
            .join("build")
            .join(format!("{}_pc", spec.rom.region.to_string()))
            .join("res")
            .join("gfx");

        fs_extra::dir::remove(target_path)
            .unwrap_or_else(|e| panic!("could not find the texture pack to remove: {}", e));
    }
    */
}
/*
impl Patch {
    fn new(name: impl ToString, path: impl Into<PathBuf>) -> Self {
        Patch {
            name: name.to_string(),
            path: path.into(),
        }
    }

    fn patch(&self, spec: &Spec, repo_dir: impl AsRef<Path>) -> Result<(), SmbuilderError> {
        Ok(())
    }
}*/
