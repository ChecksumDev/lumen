use anyhow::Result;
use std::path::PathBuf;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

pub struct Storage {
    pub base_path: PathBuf,
}

impl Storage {
    pub async fn new(path: impl Into<PathBuf>) -> Result<Self> {
        let base_path = path.into();
        tokio::fs::create_dir_all(&base_path.join("uploads")).await?;
        Ok(Storage { base_path })
    }

    fn get_path(&self, id: impl Into<String>) -> PathBuf {
        self.base_path.join(format!("uploads/{}.lumen", id.into()))
    }

    pub async fn save(&self, id: impl Into<String>, bytes: Vec<u8>) -> Result<()> {
        let path = self.get_path(id);
        let mut file = File::create(path).await?;
        file.write_all(&bytes).await?;
        Ok(())
    }

    pub async fn load(&self, id: impl Into<String>) -> Result<Vec<u8>> {
        let mut file = File::open(self.get_path(id)).await?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).await?;
        Ok(bytes)
    }

    pub async fn delete(&self, id: impl Into<String>) -> Result<()> {
        tokio::fs::remove_file(self.get_path(id)).await?;
        Ok(())
    }

    pub async fn exists(&self, id: impl Into<String>) -> Result<bool> {
        Ok(tokio::fs::metadata(self.get_path(id)).await.is_ok())
    }
}