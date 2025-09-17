use anyhow::Result;
use std::path::{Path, PathBuf};
use tokio::fs;
use sha2::{Digest, Sha256};

/// File cache for temporary storage during transfers
pub struct FileCache {
    cache_dir: PathBuf,
}

impl FileCache {
    pub fn new(cache_dir: PathBuf) -> Self {
        Self { cache_dir }
    }
    
    /// Initialize the cache directory
    pub async fn init(&self) -> Result<()> {
        fs::create_dir_all(&self.cache_dir).await?;
        Ok(())
    }
    
    /// Store a file in the cache and return its hash
    pub async fn store_file(&self, source_path: &Path) -> Result<String> {
        let file_data = fs::read(source_path).await?;
        let hash = self.calculate_hash(&file_data);
        
        let cache_path = self.cache_dir.join(&hash);
        fs::write(&cache_path, &file_data).await?;
        
        Ok(hash)
    }
    
    /// Retrieve a file from cache by hash
    pub async fn get_file(&self, hash: &str) -> Result<Vec<u8>> {
        let cache_path = self.cache_dir.join(hash);
        let data = fs::read(&cache_path).await?;
        Ok(data)
    }
    
    /// Check if a file exists in cache
    pub async fn has_file(&self, hash: &str) -> bool {
        let cache_path = self.cache_dir.join(hash);
        cache_path.exists()
    }
    
    /// Clean up old cache files
    pub async fn cleanup(&self, max_age_secs: u64) -> Result<()> {
        let mut entries = fs::read_dir(&self.cache_dir).await?;
        let cutoff = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() - max_age_secs;
        
        while let Some(entry) = entries.next_entry().await? {
            let metadata = entry.metadata().await?;
            if let Ok(modified) = metadata.modified() {
                let modified_secs = modified
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_secs();
                
                if modified_secs < cutoff {
                    let _ = fs::remove_file(entry.path()).await;
                }
            }
        }
        
        Ok(())
    }
    
    /// Calculate SHA256 hash of data
    fn calculate_hash(&self, data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
}