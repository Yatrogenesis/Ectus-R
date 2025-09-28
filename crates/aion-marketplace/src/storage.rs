use crate::{config::StorageConfig, errors::*};
use async_trait::async_trait;
use std::path::PathBuf;

#[async_trait]
pub trait StorageBackend: Send + Sync {
    async fn store_file(&self, path: &str, data: &[u8]) -> Result<String>;
    async fn get_file(&self, path: &str) -> Result<Vec<u8>>;
    async fn delete_file(&self, path: &str) -> Result<()>;
    async fn generate_download_url(&self, path: &str) -> Result<String>;
    async fn file_exists(&self, path: &str) -> Result<bool>;
    async fn get_file_size(&self, path: &str) -> Result<u64>;
}

pub struct StorageManager {
    backend: Box<dyn StorageBackend>,
}

impl StorageManager {
    pub async fn new(config: StorageConfig) -> Result<Self> {
        let backend: Box<dyn StorageBackend> = match config.provider {
            crate::config::StorageProvider::Local { path } => {
                Box::new(LocalStorage::new(path).await?)
            },
            crate::config::StorageProvider::S3 => {
                Box::new(S3Storage::new(config).await?)
            },
            crate::config::StorageProvider::MinIO => {
                Box::new(MinIOStorage::new(config).await?)
            },
        };

        Ok(Self { backend })
    }

    pub async fn store_package_files(&self, path: &str, data: &[u8]) -> Result<String> {
        self.backend.store_file(path, data).await
    }

    pub async fn download_package_files(&self, url: &str) -> Result<Vec<u8>> {
        // Extract path from URL and download
        let path = self.extract_path_from_url(url)?;
        self.backend.get_file(&path).await
    }

    pub async fn generate_download_url(&self, path: &str) -> Result<String> {
        self.backend.generate_download_url(path).await
    }

    pub async fn delete_package_files(&self, path: &str) -> Result<()> {
        self.backend.delete_file(path).await
    }

    fn extract_path_from_url(&self, url: &str) -> Result<String> {
        // Simple path extraction - in production this would be more sophisticated
        Ok(url.split('/').last().unwrap_or("").to_string())
    }
}

// Local filesystem storage implementation
pub struct LocalStorage {
    base_path: PathBuf,
}

impl LocalStorage {
    pub async fn new(base_path: PathBuf) -> Result<Self> {
        tokio::fs::create_dir_all(&base_path).await?;
        Ok(Self { base_path })
    }
}

#[async_trait]
impl StorageBackend for LocalStorage {
    async fn store_file(&self, path: &str, data: &[u8]) -> Result<String> {
        let file_path = self.base_path.join(path);

        if let Some(parent) = file_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        tokio::fs::write(&file_path, data).await?;

        Ok(format!("file://{}", file_path.display()))
    }

    async fn get_file(&self, path: &str) -> Result<Vec<u8>> {
        let file_path = self.base_path.join(path);
        tokio::fs::read(file_path).await
            .map_err(|e| MarketplaceError::StorageError(e.to_string()))
    }

    async fn delete_file(&self, path: &str) -> Result<()> {
        let file_path = self.base_path.join(path);
        tokio::fs::remove_file(file_path).await
            .map_err(|e| MarketplaceError::StorageError(e.to_string()))
    }

    async fn generate_download_url(&self, path: &str) -> Result<String> {
        // For local storage, return direct file path
        Ok(format!("file://{}", self.base_path.join(path).display()))
    }

    async fn file_exists(&self, path: &str) -> Result<bool> {
        let file_path = self.base_path.join(path);
        Ok(file_path.exists())
    }

    async fn get_file_size(&self, path: &str) -> Result<u64> {
        let file_path = self.base_path.join(path);
        let metadata = tokio::fs::metadata(file_path).await?;
        Ok(metadata.len())
    }
}

// S3 storage implementation
pub struct S3Storage {
    config: StorageConfig,
}

impl S3Storage {
    pub async fn new(config: StorageConfig) -> Result<Self> {
        Ok(Self { config })
    }
}

#[async_trait]
impl StorageBackend for S3Storage {
    async fn store_file(&self, path: &str, _data: &[u8]) -> Result<String> {
        // TODO: Implement S3 storage
        Err(MarketplaceError::StorageError("S3 storage not implemented".to_string()))
    }

    async fn get_file(&self, _path: &str) -> Result<Vec<u8>> {
        Err(MarketplaceError::StorageError("S3 storage not implemented".to_string()))
    }

    async fn delete_file(&self, _path: &str) -> Result<()> {
        Err(MarketplaceError::StorageError("S3 storage not implemented".to_string()))
    }

    async fn generate_download_url(&self, _path: &str) -> Result<String> {
        Err(MarketplaceError::StorageError("S3 storage not implemented".to_string()))
    }

    async fn file_exists(&self, _path: &str) -> Result<bool> {
        Err(MarketplaceError::StorageError("S3 storage not implemented".to_string()))
    }

    async fn get_file_size(&self, _path: &str) -> Result<u64> {
        Err(MarketplaceError::StorageError("S3 storage not implemented".to_string()))
    }
}

// MinIO storage implementation
pub struct MinIOStorage {
    config: StorageConfig,
}

impl MinIOStorage {
    pub async fn new(config: StorageConfig) -> Result<Self> {
        Ok(Self { config })
    }
}

#[async_trait]
impl StorageBackend for MinIOStorage {
    async fn store_file(&self, path: &str, _data: &[u8]) -> Result<String> {
        // TODO: Implement MinIO storage
        Err(MarketplaceError::StorageError("MinIO storage not implemented".to_string()))
    }

    async fn get_file(&self, _path: &str) -> Result<Vec<u8>> {
        Err(MarketplaceError::StorageError("MinIO storage not implemented".to_string()))
    }

    async fn delete_file(&self, _path: &str) -> Result<()> {
        Err(MarketplaceError::StorageError("MinIO storage not implemented".to_string()))
    }

    async fn generate_download_url(&self, _path: &str) -> Result<String> {
        Err(MarketplaceError::StorageError("MinIO storage not implemented".to_string()))
    }

    async fn file_exists(&self, _path: &str) -> Result<bool> {
        Err(MarketplaceError::StorageError("MinIO storage not implemented".to_string()))
    }

    async fn get_file_size(&self, _path: &str) -> Result<u64> {
        Err(MarketplaceError::StorageError("MinIO storage not implemented".to_string()))
    }
}