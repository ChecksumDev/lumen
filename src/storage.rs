use anyhow::Result;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

pub struct Storage {
    pub path: String,
}

impl Storage {
    pub async fn new(path: impl Into<String>) -> Result<Self> {
        let path = path.into();

        tokio::fs::create_dir_all(&path).await?;
        Ok(Storage { path })
    }

    pub async fn save(&self, id: impl Into<String>, bytes: &[u8]) -> Result<()> {
        let mut file = File::create(format!("{}/{}.lumen", self.path, id.into())).await?;
        file.write_all(bytes).await?;
        Ok(())
    }

    pub async fn load(&self, id: impl Into<String>) -> Result<Vec<u8>> {
        let mut file = File::open(format!("{}/{}.lumen", self.path, id.into())).await?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).await?;
        Ok(bytes)
    }

    pub async fn delete(&self, id: impl Into<String>) -> Result<()> {
        tokio::fs::remove_file(format!("{}/{}.lumen", self.path, id.into())).await?;
        Ok(())
    }
}
