use std::path::{Path, PathBuf};

use git2::{build::RepoBuilder, FetchOptions, RemoteCallbacks};

use super::Repository;
use crate::{err, err_variant_fs, err_variant_repo_clone, prelude::RepoData};

pub struct GitRepo<'d> {
    repo_dir: PathBuf,
    repo_data: &'d RepoData,
}

impl<'d> Repository for GitRepo<'d> {
    fn clone(&self) -> crate::Result<()> {
        // Implement progress reporting

        let repo_name = &self.repo_data.name;
        let repo_dir = self.repo_dir.join(repo_name);

        let mut remote_cbs = RemoteCallbacks::new();
        remote_cbs.transfer_progress(|progress| {
            // TODO: move this into progress reporter
            let recv_objs = progress.received_objects();
            let total_objs = progress.total_objects();
            print!(
                "Repository Clone: {}/{} ({}%) objects transferred ({} KiB transferred)\r",
                recv_objs,
                total_objs,
                (recv_objs * 100) / total_objs,
                (progress.received_bytes() as f64 / 1024_f64).floor()
            );

            true
        });

        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(remote_cbs);

        let clone_result = RepoBuilder::new()
            .branch(&self.repo_data.branch)
            .fetch_options(fetch_options)
            .clone(&self.repo_data.url, &repo_dir);

        match clone_result {
            Ok(_) => Ok(()),
            Err(e) => {
                let msg = e.message().to_string();
                let err = err!(
                    err_variant_repo_clone!(self.repo_data.url.clone(), repo_dir, e),
                    format!("failed to clone the repository: {msg}")
                );
                Err(err)
            }
        }
    }

    fn clean(&self) -> crate::Result<()> {
        eprintln!("exiting on ctrl-c...");

        if !self.repo_dir.exists() {
            return Ok(());
        }

        let res = std::fs::remove_dir_all(&self.repo_dir);
        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                let err = err!(
                    err_variant_fs!(e),
                    format!(
                        "failed to remove the repository at {}!",
                        self.repo_dir.display()
                    )
                );

                Err(err)
            }
        }
    }
}

impl<'d> GitRepo<'d> {
    pub fn new(repo_base_dir: impl Into<PathBuf>, repo_data: &'d RepoData) -> Self {
        let repo_dir = repo_base_dir.into();
        let repo_dir = repo_dir.join(&repo_data.name);

        Self {
            repo_dir,
            repo_data,
        }
    }
}
