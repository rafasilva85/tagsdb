use std::collections::HashMap;
use async_trait::async_trait;
use crate::domain::Tag;
use crate::infrastructure::UnsError;

/// Repository interface for tag data access
#[async_trait]
pub trait TagRepository: Send + Sync {
    /// Loads tags from a source
    async fn load_tags(&self, source: &str) -> Result<HashMap<String, Tag>, UnsError>;
    
    /// Saves tags to a destination
    async fn save_tags(&self, tags: &HashMap<String, Tag>, destination: &str) -> Result<(), UnsError>;
    
    /// Gets a tag by its path
    async fn get_tag(&self, path: &str) -> Result<Option<Tag>, UnsError>;
    
    /// Updates a tag's value
    async fn update_tag(&self, path: &str, value: String) -> Result<Option<Tag>, UnsError>;
    
    /// Gets all tags
    async fn get_all_tags(&self) -> Result<HashMap<String, Tag>, UnsError>;
}
