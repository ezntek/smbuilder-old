use crate::prelude::*;
use crate::romconvert::determine_format;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Default, Builder, Deserialize, Serialize)]
/// Represents a build spec.
///
/// All of its child structs implements
/// `Deserialize` and `Serialize`, and a
/// spec file is derived directly from this
/// structure.
pub struct Spec {
    /// The ROM to extract assets out of.
    pub rom: Rom,
    /// The repository to build from.
    pub repo: Repo,
    /// Amount of compile jobs that are
    /// allowed for the compiler. Will
    /// be used to set the `-j` flag
    /// during compile time.
    pub jobs: Option<u8>,
    /// A custom name.
    pub name: Option<String>,
    /// Make flags to be passed to the
    /// compiler.
    pub makeopts: Option<Vec<Makeopt>>,
    /// DynOS packs, if supported.
    pub dynos_packs: Option<Vec<DynosPack>>,
    /// Patrhes.
    pub patches: Option<Vec<Patch>>,
    /// Post install scripts.
    pub scripts: Option<Vec<PostBuildScript>>,
    /// A texture pack.
    pub texture_pack: Option<TexturePack>,
}

impl Spec {
    /// Creates a new spec, from a file,
    /// but **doesn't check it**, which **may
    /// lead to random panics**
    ///
    // TODO: example
    pub fn from_file<P: AsRef<Path>>(path: P) -> Spec {
        let file_string =
            fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read the file: {}", e));

        serde_yaml::from_str::<Spec>(&file_string)
            .unwrap_or_else(|e| panic!("failed to parse the file into a yaml: {}", e))
    }

    /// Check the spec if it is valid or not,
    /// returning an `SmbuilderError` if it fails
    /// a mandatory check, and running the `log`
    /// callback with `Warn` if it detects a small
    /// imperfection.
    ///
    /// Designed for use with `from_file_checked`.
    pub fn check_spec(&mut self, callbacks: &mut Callbacks) -> Result<(), SmbuilderError> {
        use LogType::*;

        // Check the ROM format and see
        // if it matches the spec
        let rom_path = if self.rom.path.exists() {
            &self.rom.path
        } else {
            let file_not_found_error = std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("the file at {} was not found!", &self.rom.path.display()),
            );
            return Err(SmbuilderError::new(
                Some(Box::new(file_not_found_error)),
                "the ROM at the given path was not found!",
            ));
        };

        let verified_rom_format = match determine_format(rom_path) {
            Ok(t) => t,
            Err(e) => {
                return Err(SmbuilderError::new(
                    Some(Box::new(e)),
                    "failed to verify the ROM's format",
                ))
            }
        };

        if verified_rom_format != self.rom.format {
            run_callback!(
                callbacks.log_cb,
                Warn,
                &format!(
                    "the ROM format specified in the spec ({:?}) does not match the file ({:?})!",
                    self.rom.format, verified_rom_format
                )
            );
        };

        // Jobs

        if self.jobs.is_none() {
            run_callback!(
                callbacks.log_cb,
                Warn,
                "did not find a value for jobs in the spec!"
            );

            run_callback!(
                callbacks.log_cb,
                Warn,
                "it is highly advised for you to specify the variable!"
            );
        }

        Ok(())
    }

    /// Creates a new spec from a file,
    /// and checks it.
    ///
    // TODO: example
    pub fn from_file_checked<P: AsRef<Path>>(
        path: P,
        callbacks: &mut Callbacks,
    ) -> Result<Spec, SmbuilderError> {
        let mut spec = Spec::from_file(path);

        let check_result = Spec::check_spec(&mut spec, callbacks);

        if let Err(e) = check_result {
            Err(e)
        } else {
            Ok(spec)
        }
    }

    /// Gets a build shell script, ready to be
    /// written to disk.
    ///
    //  TODO: example
    pub fn get_build_script(&self, repo_path: &Path) -> String {
        let makeopts_string = if let Some(makeopts) = &self.makeopts {
            get_makeopts_string(makeopts)
        } else {
            String::new()
        };

        // FreeBSD, macOS and OSes
        // with BSD make by default
        #[allow(unused_variables)]
        let make_cmd = "gmake";

        #[cfg(target_os = "linux")]
        let make_cmd = "make";

        let platform_makeopts = get_makeopts_string(&Makeopt::default_makeopts());

        let jobs = self.jobs.unwrap_or(2);

        let full_repo_dir = fs::canonicalize(repo_path).unwrap_or_else(|e| {
            panic!(
                "failed to get the absolute path from {}: {}",
                &repo_path.display(),
                e
            )
        });

        format!(
            "#!/bin/sh

# Script Generated by smbuilder.
# DO NOT EDIT; YOUR CHANGES
# WILL NOT BE SAVED.

{} -C {} {} {} -j{}
        ",
            make_cmd,
            full_repo_dir.display(),
            platform_makeopts,
            makeopts_string,
            jobs
        )
    }
}
