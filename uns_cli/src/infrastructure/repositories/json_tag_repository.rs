use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::Path,
    sync::{Arc, RwLock},
};

use crate::domain::{Tag, TagRepository};
use crate::infrastructure::UnsError;

/// Data structure for serializing/deserializing tags
#[derive(Serialize, Deserialize, Clone, Debug)]
struct TagData {
    tags: HashMap<String, Tag>,
}

/// JSON file implementation of TagRepository
pub struct JsonTagRepository {
    tags: Arc<RwLock<HashMap<String, Tag>>>,
}

impl JsonTagRepository {
    /// Creates a new JSON tag repository
    pub fn new() -> Self {
        Self {
            tags: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Creates a new JSON tag repository with pre-loaded tags
    pub fn with_tags(tags: HashMap<String, Tag>) -> Self {
        Self {
            tags: Arc::new(RwLock::new(tags)),
        }
    }
}

#[async_trait]
impl TagRepository for JsonTagRepository {
    async fn load_tags(&self, source: &str) -> Result<HashMap<String, Tag>, UnsError> {
        // Check if the file exists
        if !Path::new(source).exists() {
            return Err(UnsError::Repository(format!("File not found: {}", source)));
        }
        
        // Read the file
        let contents = fs::read_to_string(source)
            .map_err(|e| UnsError::Repository(format!("Failed to read file {}: {}", source, e)))?;
        
        // Parse the JSON
        let data: TagData = serde_json::from_str(&contents)
            .map_err(|e| UnsError::Serialization(format!("Failed to parse JSON: {}", e)))?;
        
        // Update the internal tags map
        {
            let mut tags_map = self.tags.write().unwrap();
            *tags_map = data.tags.clone();
        }
        
        Ok(data.tags)
    }
    
    async fn save_tags(&self, tags: &HashMap<String, Tag>, destination: &str) -> Result<(), UnsError> {
        // Create the TagData structure
        let data = TagData {
            tags: tags.clone(),
        };
        
        // Serialize to JSON
        let json = serde_json::to_string_pretty(&data)
            .map_err(|e| UnsError::Serialization(format!("Failed to serialize to JSON: {}", e)))?;
        
        // Write to file
        fs::write(destination, json)
            .map_err(|e| UnsError::Repository(format!("Failed to write to file {}: {}", destination, e)))?;
        
        Ok(())
    }
    
    async fn get_tag(&self, path: &str) -> Result<Option<Tag>, UnsError> {
        let tags_map = self.tags.read().unwrap();
        Ok(tags_map.get(path).cloned())
    }
    
    async fn update_tag(&self, path: &str, value: String) -> Result<Option<Tag>, UnsError> {
        let mut tags_map = self.tags.write().unwrap();
        
        if let Some(tag) = tags_map.get_mut(path) {
            // Log the change before updating
            println!("Updating tag: {} from '{}' to '{}'", path, tag.value, value);
            
            // Update the value
            tag.value = value;
            
            // Return the updated tag
            Ok(Some(tag.clone()))
        } else {
            Err(UnsError::NotFound(format!("Tag not found: {}", path)))
        }
    }
    
    async fn get_all_tags(&self) -> Result<HashMap<String, Tag>, UnsError> {
        let tags_map = self.tags.read().unwrap();
        Ok(tags_map.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    
    #[tokio::test]
    async fn test_load_tags() {
        // Create a temporary file with test data
        let mut temp_file = NamedTempFile::new().unwrap();
        let test_data = r#"{
            "tags": {
                "US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE": {
                    "path": "US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE",
                    "name": "Pump 1 Pressure",
                    "description": "Pressure sensor for Pump 1",
                    "value": "45.7"
                }
            }
        }"#;
        use std::io::Write;
        write!(temp_file, "{}", test_data).unwrap();
        
        // Create a repository and load the tags
        let repo = JsonTagRepository::new();
        let tags = repo.load_tags(temp_file.path().to_str().unwrap()).await.unwrap();
        
        // Verify the tags were loaded correctly
        assert_eq!(tags.len(), 1);
        assert!(tags.contains_key("US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE"));
        
        let tag = tags.get("US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE").unwrap();
        assert_eq!(tag.name, "Pump 1 Pressure");
        assert_eq!(tag.value, "45.7");
    }
    
    #[tokio::test]
    async fn test_update_tag() {
        // Create a repository with a test tag
        let mut tags = HashMap::new();
        tags.insert(
            "US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE".to_string(),
            Tag::new(
                "US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE".to_string(),
                "Pump 1 Pressure".to_string(),
                "Pressure sensor for Pump 1".to_string(),
                "45.7".to_string(),
            ),
        );
        
        let repo = JsonTagRepository::with_tags(tags);
        
        // Update the tag
        let updated_tag = repo.update_tag(
            "US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE",
            "50.2".to_string(),
        ).await.unwrap().unwrap();
        
        // Verify the tag was updated
        assert_eq!(updated_tag.value, "50.2");
        
        // Verify the tag was updated in the repository
        let all_tags = repo.get_all_tags().await.unwrap();
        let tag = all_tags.get("US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE").unwrap();
        assert_eq!(tag.value, "50.2");
    }
    
    #[tokio::test]
    async fn test_save_tags() {
        // Create a repository with a test tag
        let mut tags = HashMap::new();
        tags.insert(
            "US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE".to_string(),
            Tag::new(
                "US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE".to_string(),
                "Pump 1 Pressure".to_string(),
                "Pressure sensor for Pump 1".to_string(),
                "45.7".to_string(),
            ),
        );
        
        let repo = JsonTagRepository::with_tags(tags.clone());
        
        // Create a temporary file to save to
        let temp_file = NamedTempFile::new().unwrap();
        
        // Save the tags
        repo.save_tags(&tags, temp_file.path().to_str().unwrap()).await.unwrap();
        
        // Read the file and verify the contents
        let contents = fs::read_to_string(temp_file.path()).unwrap();
        let data: TagData = serde_json::from_str(&contents).unwrap();
        
        assert_eq!(data.tags.len(), 1);
        assert!(data.tags.contains_key("US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE"));
        
        let tag = data.tags.get("US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE").unwrap();
        assert_eq!(tag.name, "Pump 1 Pressure");
        assert_eq!(tag.value, "45.7");
    }
}
