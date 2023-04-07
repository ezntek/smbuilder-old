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
use std::path::{PathBuf, Path};

use crate::makeopts::MakeoptsType;

#[cfg(test)]
mod tests {
    #[test]
    fn test_region_se_de() {
        use crate::common::*;
        use toml;        
        
        #[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
        struct Test {
            region: Region
        }

        let t = Test {
            region: Region::US
        };

        let s = toml::to_string(&t).unwrap();
        println!("{}", s);

        let t2: Test = toml::from_str(&s).unwrap();
        println!("{:?}", t2);
        
        assert_eq!(t2.region, Region::US);
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

#[derive(Debug, PartialEq, serde_derive::Deserialize, serde_derive::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Region {
    US,
    EU,
    JP,
    SH
}


#[derive(serde_derive::Deserialize, serde_derive::Serialize)]
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

#[derive(serde_derive::Deserialize, serde_derive::Serialize)]
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

pub struct TomlSpec<M: MakeoptsType> {
    pub dynos_packs: Vec<DynOSPack>,
    pub build_settings: BuildSpec<M>,
}