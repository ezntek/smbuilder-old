use fs_extra::dir::{CopyOptions, TransitProcessResult};

use super::PackManager;
use crate::{
    callbacks::{run_callback, Callbacks},
    err, err_variant_fs,
    prelude::PackData,
};

use std::path::Path;

use crate::callback_types::LogType as L;

pub struct DefaultPackManager<'d, 'cb> {
    out_dir: &'d Path,
    callbacks: &'cb mut Callbacks<'cb>,
    pack_data: &'d PackData,
}

impl<'d, 'cb> PackManager for DefaultPackManager<'d, 'cb> {
    fn install_dynos(&mut self) -> crate::Result<()> {
        let target_path = self.out_dir.join("dynos").join("packs");
        let copy_opts = fs_extra::dir::CopyOptions::new()
            .overwrite(true)
            .copy_inside(true);

        if let Some(ref packs) = self.pack_data.dynos {
            for (packn, pack) in packs.iter().enumerate() {
                let msg = format!("copying pack {} of {}", packn + 1, packs.len());
                run_callback!(self.callbacks.log, L::Info, &msg);

                let target_path = target_path.join(&pack.name);
                match std::fs::create_dir(&target_path) {
                    Ok(_) => {}
                    Err(e) => {
                        let msg = format!(
                            "whilst creating a directory at {} for the DynOS pack",
                            target_path.display()
                        );

                        let err = err!(
                            err_variant_fs!(e, msg),
                            "failed to create a directory for the DynOS pack"
                        );

                        return Err(err);
                    }
                };

                let res = fs_extra::dir::copy_with_progress(
                    &pack.path,
                    &target_path,
                    &copy_opts,
                    |progress| {
                        run_callback!(
                            self.callbacks.fs_operation_progress,
                            progress.copied_bytes,
                            progress.total_bytes
                        );

                        TransitProcessResult::ContinueOrAbort
                    },
                );

                match res {
                    Ok(_) => {}
                    Err(e) => {
                        let msg = format!(
                            "whilst copying the DynOS pack from {} to {}",
                            pack.path.display(),
                            target_path.display(),
                        );

                        let err = err!(err_variant_fs!(e, msg), "failed to copy the DynOS Pack");

                        return Err(err);
                    }
                };
            }
        }

        Ok(())
    }

    fn install_texture(&mut self) -> crate::Result<()> {
        let texture_pack = match self.pack_data.texture {
            Some(ref p) => p,
            None => return Ok(()),
        };

        run_callback!(self.callbacks.log, L::Info, "installing texture pack");

        let target_path = self.out_dir.join("res").join("gfx");
        let pack_path = texture_pack.path.join("gfx");

        if !pack_path.exists() {
            let err_inner = std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "could not find the gfx directory in the texture pack",
            );

            let msg = format!(
                "whilst copying the texture pack from {} to {}",
                pack_path.display(),
                target_path.display()
            );

            let err = err!(err_variant_fs!(err_inner, msg), "invalid texture pack");
            return Err(err);
        }

        let res = fs_extra::dir::copy(&pack_path, &target_path, &CopyOptions::new());

        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                let msg = format!(
                    "whilst trying to copy the texture pack from {} to {}",
                    texture_pack.path.display(),
                    target_path.display()
                );
                let err = err!(err_variant_fs!(e, msg), "failed to copy the DynOS pack");
                return Err(err);
            }
        }
    }

    fn remove_dynos(&mut self) -> crate::Result<()> {
        let target_path = self.out_dir.join("dynos").join("packs");

        if let Some(ref packs) = self.pack_data.dynos {
            for (packn, pack) in packs.iter().enumerate() {
                let msg = format!("removing pack {} of {}", packn + 1, packs.len());
                run_callback!(self.callbacks.log, L::Info, &msg);

                let installed_pack_path = target_path.join(&pack.name);

                match std::fs::remove_dir_all(&installed_pack_path) {
                    Ok(_) => {}
                    Err(e) => {
                        let msg = format!(
                            "whilst trying to remove the DynOS pack at {}",
                            pack.path.display()
                        );

                        let err = err!(err_variant_fs!(e, msg), "failed to remove the DynOS pack");

                        return Err(err);
                    }
                }
            }
        }

        Ok(())
    }

    fn remove_texture(&mut self) -> crate::Result<()> {
        let texture_pack = match self.pack_data.texture {
            Some(ref p) => p,
            None => return Ok(()),
        };

        let target_path = self.out_dir.join("res").join("gfx");
        let pack_path = texture_pack.path.join("gfx");

        match std::fs::remove_dir_all(&target_path) {
            Ok(_) => Ok(()),
            Err(e) => {
                let msg = format!(
                    "whilst trying to remove the texture pack at {}",
                    pack_path.display()
                );

                let err = err!(err_variant_fs!(e, msg), "failed to remove the texture pack");

                Err(err)
            }
        }
    }
}
