pub mod packs;
pub mod repo;

pub trait Repository {
    fn clone(&self) -> crate::Result<()>;
    fn clean(&self) -> crate::Result<()>;
}

pub trait PackManager {
    fn install_texture(&self) -> crate::Result<()>;
    fn install_dynos(&self) -> crate::Result<()>;

    fn install_all(&self) -> crate::Result<()> {
        self.install_texture()?;
        self.install_dynos()?;

        Ok(())
    }
}
