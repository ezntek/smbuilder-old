// Copyright 2023 Eason Qin <eason@ezntek.com>.
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//  http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde_derive;
use std::fs;
use std::path::{PathBuf, Path};

use crate::makeopts::MakeoptsType;

#[cfg(test)]
mod tests {
    use crate::{makeopts::Render96exMakeopts, prelude::{DynOSPack, TomlSpec}};

    #[test]
    fn test_de_spec() {
        let toml_str = "
[build_settings]
    jobs = 4
    name = \"foobar\"
    executable_path = \"/path/to/sm64.region.f3dex2e\"

    [build_settings.repo]
        name = \"My SM64 PC Port\"
        url = \"protocol://link.to/the/repo.git\"
        branch = \"master\"
        supports_packs = true
        supports_textures = false

    [build_settings.rom]
        path = \"/path/to/baserom.z64\"
        region = \"us\"

    [[build_settings.additional_makeopts]]
        opt = \"DISCORDRPC\"

    [[build_settings.additional_makeopts]]
        opt = \"OSX_BUILD\"

    [[build_settings.additional_makeopts]]
        opt = \"TARGET_ARCH\"
        arg = \"native\"

[[dynos_packs]]
    path = \"/path/to/pack\"
    label = \"my pack\"
    enabled = false

[[dynos_packs]]
    path = \"/path/to/pack\"
    label = \"my pack 2\"
    enabled = true
";

        let s = toml::from_str::<TomlSpec<Render96exMakeopts>>(&toml_str).unwrap();
        let mut spec = s.build_settings;
        spec.dynos_packs = s.dynos_packs;
        println!("{:?}",spec);
    }
}

fn get_dummy_base_path() -> PathBuf {
    Path::new(std::env!("HOME")).join(".local/share/smbuilder")
}

pub enum Versions {
    Render96ex,
    Sm64ex,
    Sm64exCoop,
}

#[derive(serde_derive::Deserialize, serde_derive::Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Region {
    US,
    EU,
    JP,
    SH
}


#[derive(serde_derive::Deserialize, serde_derive::Serialize, Debug)]
pub struct Rom {
    pub region: Region,
    pub path: PathBuf,
}

impl Rom {
    pub fn default() -> Rom {
        Rom {
            path: get_dummy_base_path(),
            region: Region::US,
        }
    }
}

#[derive(serde_derive::Deserialize, serde_derive::Serialize, Debug)]
pub struct Repo {
    pub name: String,
    pub url: String,
    pub branch: String,
    pub supports_packs: bool,
    pub supports_textures: bool
}

impl Repo {
    pub fn default() -> Repo {
        Repo {
            name: "dummy repo".to_string(),
            url: "https://github.com/ezntek/smbuilder".to_string(),
            branch: "main".to_string(),
            supports_packs: false,
            supports_textures: false
        }
    }
}

#[derive(serde_derive::Deserialize, serde_derive::Serialize, Debug)]
pub struct DynOSPack {
    pub enabled: bool,
    pub label: String,
    pub path: PathBuf,
}

// The Build Specification Structure. Contains all the metadata required to run the Smbuilder class and the SmbuilderBuilder class, etc.
//
// Supports:
//
// * Jobs (jobs = [make] -jX)
// * Name (A custom name can be used, else it is repo.name)
// * Additional Make Options (eg. FOO=1 BAR=baz QUUX=0, make  FOO=1 BAR=baz QUUX=0 -jX)
// * A custom texture pack (think Render96)
// * DynOS data packs (also think Render96, but other ports like sm64ex-coop supports them too)
//

#[derive(serde_derive::Deserialize, serde_derive::Serialize, Debug)]
pub struct BuildSpec<M: MakeoptsType> {
    // The number of jobs to be put together with the MAKEOPTS during the compile stage.
    pub jobs: u8,
    // The name of the build, it will default to the name of the repo if left empty.
    pub name: String,
    // Any additional makeopts to be added to the make call. Will include the jobs.
    pub additional_makeopts: Vec<M>,
    // The executable path. Not playable if empty, playable if not empty.
    pub executable_path: Option<String>,
    // A custom texture pack (There can only be one!)
    pub texture_pack_path: Option<String>,
    // The repo struct
    pub repo: Repo,
    // The rom struct
    pub rom: Rom,
    // Any DynOS packs the user wishes to add
    pub dynos_packs: Vec<DynOSPack>,
}

impl<M: MakeoptsType + for<'a> serde::Deserialize<'a>> BuildSpec<M> {
    pub fn from_file<P: AsRef<Path>>(path: P) -> BuildSpec<M>{
        let toml_str = fs::read_to_string(path).unwrap();
        let s = toml::from_str::<TomlSpec<M>>(&toml_str).unwrap();
        let mut spec = s.build_settings;
        spec.dynos_packs = s.dynos_packs;
        spec
    }
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TomlSpec<M: MakeoptsType> {
    pub dynos_packs: Vec<DynOSPack>,
    pub build_settings: BuildSpec<M>,
}