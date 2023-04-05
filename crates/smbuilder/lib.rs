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

mod builder;
mod common;

use std::path::Path;

pub use crate::builder::*;
pub use crate::common::*;

fn init() {
    // Set the home dir for easy access
    let home_dir = std::env!("HOME");

    // Create ~/.local/share/smbuilder
    std::fs::create_dir(Path::new(&home_dir).join(".local/share/smbuilder"))
        .expect(format!("Failed to create {}/.local/share/smbuilder! Perhaps the directory already exists?", home_dir).as_str());
}