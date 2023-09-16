use std::{path::PathBuf, thread};

use clap::Parser;
use colored::Colorize;
use smbuilder::prelude::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    base_dir: PathBuf,
}

fn get_builder<'b>(base_dir: PathBuf) -> Builder<'b> {
    let mut callbacks = Callbacks::new()
        .log(|log_type, text| {
            use callback_types::LogType as L;
            match log_type {
                L::Error => eprintln!("{}{}", "error: ".bold().red(), text),
                L::Warn => eprintln!("{}{}", "warn: ".bold().magenta(), text),
                L::BuildOutput => println!("{}{}", "make: ".bold().cyan(), text),
                L::Info => println!("{}{}", "info: ".bold().blue(), text),
            }
        })
        .repo_clone_progress(|recv_objs, total_objs, bytes_transferred| {
            print!(
                "{} {}/{} ({}%) objects transferred ({} KiB transferred)\r",
                "clone:".bold().green(),
                recv_objs,
                total_objs,
                (recv_objs * 100) / total_objs,
                (bytes_transferred as f64 / 1024_f64).floor(),
            )
        })
        .new_setup_stage(|stage| {
            println!("{}{}", "stage: ".bold().green(), stage.to_string());
        });

    let spec_path = base_dir.join("build.yaml");
    let spec = Spec::from_file_checked(spec_path, &mut callbacks).unwrap();
    Builder::new(spec, base_dir.clone(), callbacks).unwrap()
}

fn build<'b>(mut builder: Builder<'b>) {
    builder.build();
}

fn main() {
    color_eyre::install().unwrap();

    let args = Args::parse();

    if !args.base_dir.is_dir() {
        panic!("{} is not a directory! please enter the path to a directory with an `build.yaml` in the root of it.", args.base_dir.display());
    }

    let builder = get_builder(args.base_dir);

    thread::spawn(move || {
        build(builder);
    })
    .join()
    .unwrap();
}
