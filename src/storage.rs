use anyhow::Result;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

pub struct Storage {
    pub base_path: String,
}

impl Storage {
    pub async fn new(path: impl Into<String>) -> Result<Self> {
        let base_path = path.into();

        // todo: more elegant way to do this?
        tokio::fs::create_dir_all(format!("{}/uploads", &base_path)).await?;
        Ok(Storage { base_path })
    }

    pub async fn save(&self, id: impl Into<String>, bytes: &[u8]) -> Result<()> {
        let mut file =
            File::create(format!("{}/uploads/{}.lumen", self.base_path, id.into())).await?;
        file.write_all(bytes).await?;
        Ok(())
    }

    pub async fn load(&self, id: impl Into<String>) -> Result<Vec<u8>> {
        let mut file =
            File::open(format!("{}/uploads/{}.lumen", self.base_path, id.into())).await?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).await?;
        Ok(bytes)
    }

    pub async fn delete(&self, id: impl Into<String>) -> Result<()> {
        tokio::fs::remove_file(format!("{}/uploads/{}.lumen", self.base_path, id.into())).await?;
        Ok(())
    }

    pub async fn exists(&self, id: impl Into<String>) -> Result<bool> {
        Ok(
            tokio::fs::metadata(format!("{}/uploads/{}.lumen", self.base_path, id.into()))
                .await
                .is_ok(),
        )
    }
}
