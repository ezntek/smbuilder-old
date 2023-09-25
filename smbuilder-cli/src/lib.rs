use colored::Colorize;
use smbuilder::prelude::*;
use std::path::PathBuf;

pub fn get_builder<'b>(base_dir: PathBuf, mut callbacks: Callbacks<'b>) -> Builder<'b> {
    let spec_path = base_dir.join("build.yaml");
    let spec = Spec::from_file_checked(spec_path, &mut callbacks).unwrap();
    Builder::new(spec, base_dir.clone(), callbacks).unwrap()
}
