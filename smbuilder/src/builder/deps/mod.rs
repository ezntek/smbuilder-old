pub mod packs;
pub mod repo;

pub trait RepoManager {
    fn clone(&mut self) -> crate::Result<()>;
    fn clean(&self) -> crate::Result<()>;
}

pub trait PackManager {
    fn install_texture(&mut self) -> crate::Result<()>;
    fn install_dynos(&mut self) -> crate::Result<()>;

    fn remove_texture(&mut self) -> crate::Result<()>;
    fn remove_dynos(&mut self) -> crate::Result<()>;

    fn install_all(&mut self) -> crate::Result<()> {
        self.install_texture()?;
        self.install_dynos()?;

        Ok(())
    }

    fn remove_all(&mut self) -> crate::Result<()> {
        self.remove_texture()?;
        self.remove_dynos()?;

        Ok(())
    }
}
