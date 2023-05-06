use std::{fs, fmt::{Display, Debug}, path::Path};

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
use crate::*;
use super::builder::get_makeopts_string;
use serde::{Deserialize,Serialize};

#[derive(Default, Builder, Debug, Deserialize, Serialize)]
#[builder(setter(into))]
pub struct Spec {
    pub rom: Rom,
    pub repo: Repo,
    pub jobs: u8,
    pub name: String,
    pub additional_makeopts: Vec<Makeopt>,
    pub packs: Option<Vec<Datapack>>
}

impl Spec {
    pub fn from_file(path: PathBuf) -> Spec {
        toml::from_str(
            &fs::read_to_string(&path)
                .expect(format!("Failed to read {}", &path.display()).as_str())
        ).expect(format!("Failed to parse {} into a toml!", &path.display()).as_str())
    }

    pub fn get_build_script(&self, repo_path: &PathBuf) -> String
    {
        format!("
#!/bin/sh
cd {}
make {} -j{}
        ", repo_path.display(), get_makeopts_string(&self.additional_makeopts), self.jobs)
    } 
}