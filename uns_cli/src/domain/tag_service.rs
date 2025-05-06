use async_trait::async_trait;
use crate::domain::Tag;
use crate::infrastructure::UnsError;
use std::collections::HashMap;

/// Service interface for tag operations
#[async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait TagService: Send + Sync {
    /// Loads tags from a source and publishes them
    async fn load_and_publish_tags(&self, source: &str) -> Result<(), UnsError>;
    
    /// Updates a tag's value and publishes the update
    async fn update_and_publish_tag(&self, path: &str, value: String) -> Result<(), UnsError>;
    
    /// Gets all tags
    async fn get_all_tags(&self) -> Result<HashMap<String, Tag>, UnsError>;
    
    /// Gets a tag by its path
    async fn get_tag(&self, path: &str) -> Result<Option<Tag>, UnsError>;
}
