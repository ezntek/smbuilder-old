/// Builder-related types.
pub mod types;

/// Various build stages that the build process is dependent on.
pub mod deps;

/// Houses the builder struct that
/// takes care of all the building.
pub mod builder;

use crate::callback_types::LogType;
use crate::prelude::{run_callback, Callbacks, Region, Spec};

use types::SetupStage;
use LogType::*;

use std::path::Path;

/// Get the core setup tasks that are needed.
///
/// Returns a list of [SetupStage].
// TODO: example
pub fn get_needed_setup_tasks<P: AsRef<Path>>(
    spec: &Spec,
    base_dir: P,
    callbacks: &mut Callbacks,
) -> Vec<SetupStage> {
    use SetupStage::*;

    let base_dir = base_dir.as_ref();
    let mut needed_stages: Vec<SetupStage> = Vec::new();

    // check if the repo is cloned
    if !base_dir.join(&spec.repo.name).exists() {
        needed_stages.push(CloneRepo)
    }

    // check if the rom exists
    if !base_dir
        .join(&spec.repo.name)
        .join(format!("baserom.{}.z64", spec.rom.region.to_string()))
        .exists()
    {
        needed_stages.push(CopyRom)
    }

    // check if the build script exists
    if !base_dir.join("build.sh").exists() {
        needed_stages.push(CreateBuildScript)
    }

    // log
    let needed_stages_string = needed_stages
        .iter()
        .map(|elem| elem.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    run_callback!(
        callbacks.log,
        Info,
        &format!("needed tasks: {}", needed_stages_string)
    );

    // post-build script stuff
    if !base_dir.join("scripts").exists() {
        needed_stages.push(CreateScriptsDir)
    }

    if let Some(scripts) = &spec.scripts {
        for script in scripts {
            if script.path.is_none() {
                needed_stages.push(WritePostBuildScripts);
                continue;
            }
        }
    }

    // return
    needed_stages
}

impl ToString for Region {
    fn to_string(&self) -> String {
        use Region::*;

        let retval = match self {
            Us => "us",
            Eu => "eu",
            Jp => "jp",
            Sh => "sh",
        };

        retval.to_owned()
    }
}
