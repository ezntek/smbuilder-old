use crate::builder_types::PostBuildStage::*;
use crate::callback_types::LogType::{self, *};
use crate::callbacks::run_callback;
use crate::error::ErrorCause;
use crate::prelude::error_macros::*;
use crate::prelude::{err, Callbacks, Error, Spec};

use duct::cmd;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use super::deps::{PackManager, RepoManager};

/// The main builder class which takes care of building
/// a spec.
///
/// Includes fields and methods that makes up the core of
/// any smbuilder-derived app.
///
/// # Example
///
/// ```rust
/// use smbuilder::prelude::*;
///
/// // set your callbacks up first
/// let mut callbacks = Callbacks::empty();
///
/// // and your spec
/// let my_spec = Spec::from_file("path/to/my/smbuilder.yaml")
///
/// // set up your builder
/// let mut builder = Builder::new(my_spec, "path/to/the/base/dir", mut callbacks);
///
/// // compile the spec, with the specified callbacks.
/// builder.build();
///
/// ```
///
pub struct Builder<'cb> {
    /// The spec that the build will be built with.
    pub spec: Spec,

    /// The base directory, the dir where the spec lives
    pub base_dir: PathBuf,

    /// The logger.
    pub callbacks: Callbacks<'cb>,

    /// Repository manager
    repo_manager: Box<dyn RepoManager>,

    /// Pack manager
    pack_manager: Box<dyn PackManager>,
}

impl<'a> Builder<'a> {
    /// Creates a new `Builder`.
    ///
    /// It takes in the callbacks, for events that
    /// may happen during the build process.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # fn main() {
    /// let mut builder = Builder::new(my_spec, my_base_dir, my_callbacks)
    /// // you must have your spec, base dir and callbacks set up beforehand!
    /// # }
    /// ```
    pub fn new<P: Into<PathBuf>>(
        spec: Spec,
        base_dir: P,
        callbacks: Callbacks,
        repo_manager: Box<dyn RepoManager>,
        pack_manager: Box<dyn PackManager>,
    ) -> Result<Builder, Error> {
        let result = Builder {
            spec,
            base_dir: base_dir.into(),
            callbacks,
            repo_manager,
            pack_manager,
        };

        Ok(result)
    }

    fn compile(&mut self) {
        let build_script_path = self.base_dir.join("build.sh").canonicalize().unwrap();
        dbg!(&build_script_path);
        let build_cmd = cmd!(build_script_path).stderr_to_stdout();
        let output = build_cmd
            .reader()
            .unwrap_or_else(|e| panic!("failed to get a reader from the command: {}", e));
        let reader = BufReader::new(output);

        for line in reader.lines() {
            let ln = match line {
                Ok(line) => line,
                Err(e) => panic!("something went wrong: {}", e),
            }; // exit when there is no more output

            run_callback!(self.callbacks.log, BuildOutput, &ln);
        }
    }

    fn install_texture_pack(&mut self) -> crate::Result<()> {
        run_callback!(self.callbacks.new_postbuild_stage, TexturePack);
        Ok(()) // FIXME:: add logic
    }

    fn install_dynos_packs(&mut self) -> crate::Result<()> {
        run_callback!(self.callbacks.new_postbuild_stage, DynOSPacks);

        Ok(()) // FIXME: add logic
    }

    fn run_postbuild_scripts(&mut self) -> crate::Result<()> {
        run_callback!(self.callbacks.new_postbuild_stage, PostBuildScripts);

        let scripts = if let Some(scripts) = &self.spec.scripts {
            scripts
        } else {
            return Ok(());
        };

        for script in scripts {
            run_callback!(
                self.callbacks.new_postbuild_script,
                &script.name,
                &script.description
            );

            let script_path = script.path.as_ref().unwrap_or_else(|| {
                panic!("failed to unwrap the script path (please report this bug!)")
            });

            let cmd = cmd!(script_path);
            match cmd.run() {
                Ok(_) => (),
                Err(e) => {
                    return Err(err!(
                        err_variant_cmdlaunch!(
                            script_path.to_string_lossy().to_string(),
                            "failed to run the script",
                            e
                        ),
                        format!("whilst trying to run script {}", script.name)
                    ))
                }
            };
        }

        Ok(())
    }

    fn setup_build(&mut self) -> crate::Result<()> {
        // FIXME: later
        self.repo_manager.clone()?;
        Ok(())
    }

    fn post_build(&mut self) -> crate::Result<()> {
        // FIXME: later
        self.pack_manager.install_all()?;
        Ok(())
    }

    /// Build the spec.
    ///
    /// # Example
    ///
    /// ```no_run
    /// let mut builder = Builder::new(my_spec, my_base_dir, my_callbacks);
    /// // you must have your spec, base dir and callbacks set up beforehand!
    ///
    /// // builds the spec, takes a mutable reference
    /// // to itself for the callbacks.
    /// builder.build();
    /// ```
    pub fn build(&mut self) -> crate::Result<()> {
        self.setup_build()?;

        let executable_name = format!("sm64.{}.f3dex2e", self.spec.rom.region.to_string());

        let executable_path = self
            .base_dir
            .join(&self.spec.repo.name)
            .join("build")
            .join(format!("{}_pc", self.spec.rom.region.to_string()))
            .join(executable_name);

        if !executable_path.exists() {
            self.compile();
        } else {
            run_callback!(
                self.callbacks.log,
                LogType::Warn,
                &format!(
                    "not building the spec: the executable at {} already exists!",
                    executable_path.display()
                )
            );
        }

        self.post_build()?;
        Ok(())
    }
}
