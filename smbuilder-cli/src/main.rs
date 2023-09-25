use clap::{Parser, Subcommand};
use colored::Colorize;
use smbuilder::prelude::{callback_types::LogType, *};
use std::{fmt::format, path::PathBuf, thread};

use smbuilder_cli::get_builder;

#[derive(Clone, Subcommand)]
enum Action {
    Build { spec_path: PathBuf },
    Run { spec_path: PathBuf },
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    subcommand: Action,
}

fn build(base_dir: PathBuf, callbacks: Callbacks<'static>) {
    if !base_dir.is_dir() {
        panic!("{} is not a directory! please enter the path to a directory with an `build.yaml` in the root of it.", base_dir.display());
    }

    let mut builder = get_builder(base_dir.clone(), callbacks);

    thread::spawn(move || match builder.build() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e.to_string());
            std::process::exit(1);
        }
    })
    .join()
    .unwrap();
}

fn run(base_dir: PathBuf, mut callbacks: Callbacks) {
    let spec = Spec::from_file_checked(base_dir.join("build.yaml"), &mut callbacks);
    let spec = match spec {
        Ok(s) => s,
        Err(e) => {
            if let Some(cb) = &mut callbacks.log_cb {
                cb(
                    LogType::Error,
                    format!("failed to open the specfile: {}", e).as_str(),
                );
            }
            std::process::exit(1)
        }
    };
    let region = spec.rom.region.to_string();
    let path = base_dir
        .join(&spec.repo.name)
        .join("build")
        .join(format!("{}_pc", &region))
        .join(format!("sm64.{}.f3dex2e", &region));
    let mut cmd = std::process::Command::new(path);
    let mut child = cmd.spawn().unwrap_or_else(|e| {
        // FIXME: integrate into log
        if let Some(cb) = &mut callbacks.log_cb {
            cb(LogType::Error, "failed to spawn the command");
        };
        std::process::exit(1);
    });
    child.wait().unwrap(); // TODO: macro out the errors
}

fn main() {
    color_eyre::install().unwrap();

    let args = Args::parse();

    let callbacks = Callbacks::new()
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

    match args.subcommand {
        Action::Build {
            spec_path: base_dir,
        } => build(base_dir, callbacks),
        Action::Run { spec_path } => run(spec_path, callbacks),
    };
}
