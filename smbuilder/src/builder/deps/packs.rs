use crate::prelude::PackData;

use super::PackManager;

pub struct DefaultPackManager<'d> {
    pack_data: &'d PackData,
}

impl<'d> PackManager for DefaultPackManager<'d> {
    fn install_dynos(&self) -> crate::Result<()> {
        Ok(())
    }

    fn install_texture(&self) -> crate::Result<()> {
        Ok(())
    }
}
