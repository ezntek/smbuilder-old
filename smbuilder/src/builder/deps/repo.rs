use std::path::{Path, PathBuf};

use git2::{build::RepoBuilder, FetchOptions, RemoteCallbacks};

use super::RepoManager;
use crate::{
    callbacks::{run_callback, Callbacks},
    err, err_variant_fs, err_variant_repo_clone,
    prelude::RepoData,
};

pub struct GitRepo<'d, 'cb> {
    repo_dir: PathBuf,
    callbacks: &'cb mut Callbacks<'cb>,
    repo_data: &'d RepoData,
}

impl<'d, 'cb> RepoManager for GitRepo<'d, 'cb> {
    fn clone(&mut self) -> crate::Result<()> {
        // Implement progress reporting

        let repo_name = &self.repo_data.name;
        let repo_dir = self.repo_dir.join(repo_name);

        let mut remote_cbs = RemoteCallbacks::new();
        remote_cbs.transfer_progress(|progress| {
            run_callback!(
                self.callbacks.repo_clone_progress,
                progress.received_objects(),
                progress.total_objects(),
                progress.received_bytes()
            );

            true
        });

        let mut fetch_options = FetchOptions::new();
        let depth = if self.repo_data.deep_clone { 0 } else { 1 };
        fetch_options.remote_callbacks(remote_cbs).depth(depth);

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

impl<'d, 'cb> GitRepo<'d, 'cb> {
    pub fn new(
        repo_base_dir: impl Into<PathBuf>,
        repo_data: &'d RepoData,
        callbacks: &'cb mut Callbacks<'cb>,
    ) -> Self {
        let repo_dir = repo_base_dir.into();
        let repo_dir = repo_dir.join(&repo_data.name);

        Self {
            repo_dir,
            repo_data,
            callbacks,
        }
    }
}
