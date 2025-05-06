use async_trait::async_trait;
use std::{collections::HashMap, sync::Arc};

use crate::domain::{Tag, TagRepository, TagService};
use crate::infrastructure::{mqtt::MqttPublisher, UnsError};

/// Implementation of the TagService interface
pub struct TagServiceImpl {
    repository: Arc<dyn TagRepository>,
    publisher: Arc<dyn MqttPublisher>,
}

impl TagServiceImpl {
    /// Creates a new TagServiceImpl
    pub fn new(repository: Arc<dyn TagRepository>, publisher: Arc<dyn MqttPublisher>) -> Self {
        Self {
            repository,
            publisher,
        }
    }
}

#[async_trait]
impl TagService for TagServiceImpl {
    async fn load_and_publish_tags(&self, source: &str) -> Result<(), UnsError> {
        // Load tags from the repository
        let tags = self.repository.load_tags(source).await?;
        
        // Convert to a vector for publishing
        let tags_vec: Vec<Tag> = tags.values().cloned().collect();
        
        // Publish individual tags
        self.publisher.publish_tags(&tags_vec).await?;
        
        // Create a TagDatabase for publishing
        let tag_data = crate::infrastructure::mqtt::publisher::TagDatabase { tags };
        
        // Publish the full database
        self.publisher.publish_database(&tag_data).await?;
        
        Ok(())
    }
    
    async fn update_and_publish_tag(&self, path: &str, value: String) -> Result<(), UnsError> {
        // Update the tag in the repository
        let updated_tag = self.repository.update_tag(path, value).await?;
        
        if let Some(tag) = updated_tag {
            // Publish the updated tag
            self.publisher.publish_tag(&tag).await?;
            
            // Get all tags and publish the full database
            let all_tags = self.repository.get_all_tags().await?;
            
            // Create a TagDatabase for publishing
            let tag_data = crate::infrastructure::mqtt::publisher::TagDatabase { tags: all_tags };
            
            // Publish the full database
            self.publisher.publish_database(&tag_data).await?;
            
            Ok(())
        } else {
            Err(UnsError::NotFound(format!("Tag not found: {}", path)))
        }
    }
    
    async fn get_all_tags(&self) -> Result<HashMap<String, Tag>, UnsError> {
        self.repository.get_all_tags().await
    }
    
    async fn get_tag(&self, path: &str) -> Result<Option<Tag>, UnsError> {
        self.repository.get_tag(path).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;
    
    // Mock the TagRepository
    mock! {
        pub TagRepository {}
        
        #[async_trait]
        impl TagRepository for TagRepository {
            async fn load_tags(&self, source: &str) -> Result<HashMap<String, Tag>, UnsError>;
            async fn save_tags(&self, tags: &HashMap<String, Tag>, destination: &str) -> Result<(), UnsError>;
            async fn get_tag(&self, path: &str) -> Result<Option<Tag>, UnsError>;
            async fn update_tag(&self, path: &str, value: String) -> Result<Option<Tag>, UnsError>;
            async fn get_all_tags(&self) -> Result<HashMap<String, Tag>, UnsError>;
        }
    }
    
    // Mock the MqttPublisher
    mock! {
        pub MqttPublisher {}
        
        #[async_trait]
        impl MqttPublisher for MqttPublisher {
            async fn publish_tag(&self, tag: &Tag) -> Result<(), UnsError>;
            async fn publish_tags(&self, tags: &[Tag]) -> Result<(), UnsError>;
            async fn publish_database(&self, data: &crate::infrastructure::mqtt::publisher::TagDatabase) -> Result<(), UnsError>;
        }
    }
    
    #[tokio::test]
    async fn test_load_and_publish_tags() {
        // Create mock repository
        let mut mock_repo = MockTagRepository::new();
        
        // Set up expectations for load_tags
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
        
        mock_repo
            .expect_load_tags()
            .with(eq("test.json"))
            .times(1)
            .returning(move |_| Ok(tags.clone()));
        
        // Create mock publisher
        let mut mock_publisher = MockMqttPublisher::new();
        
        // Set up expectations for publish_tags
        mock_publisher
            .expect_publish_tags()
            .times(1)
            .returning(|_| Ok(()));
        
        // Set up expectations for publish_database
        mock_publisher
            .expect_publish_database()
            .times(1)
            .returning(|_| Ok(()));
        
        // Create the service
        let service = TagServiceImpl::new(
            Arc::new(mock_repo),
            Arc::new(mock_publisher),
        );
        
        // Call the method
        let result = service.load_and_publish_tags("test.json").await;
        
        // Verify the result
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_update_and_publish_tag() {
        // Create mock repository
        let mut mock_repo = MockTagRepository::new();
        
        // Set up expectations for update_tag
        let tag_path = "US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE".to_string();
        let tag_name = "Pump 1 Pressure".to_string();
        let tag_desc = "Pressure sensor for Pump 1".to_string();
        let tag_value = "50.2".to_string();
        
        // Create a tag for testing
        Tag::new(
            tag_path.clone(),
            tag_name.clone(),
            tag_desc.clone(),
            tag_value.clone(),
        );
        
        mock_repo
            .expect_update_tag()
            .with(eq("US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE"), eq("50.2".to_string()))
            .times(1)
            .returning(move |_, _| {
                let tag = Tag::new(
                    tag_path.clone(),
                    tag_name.clone(),
                    tag_desc.clone(),
                    tag_value.clone(),
                );
                Ok(Some(tag))
            });
        
        // Set up expectations for get_all_tags
        let mut tags = HashMap::new();
        tags.insert(
            "US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE".to_string(),
            Tag::new(
                "US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE".to_string(),
                "Pump 1 Pressure".to_string(),
                "Pressure sensor for Pump 1".to_string(),
                "50.2".to_string(),
            ),
        );
        
        mock_repo
            .expect_get_all_tags()
            .times(1)
            .returning(move || Ok(tags.clone()));
        
        // Create mock publisher
        let mut mock_publisher = MockMqttPublisher::new();
        
        // Set up expectations for publish_tag
        mock_publisher
            .expect_publish_tag()
            .times(1)
            .returning(|_| Ok(()));
        
        // Set up expectations for publish_database
        mock_publisher
            .expect_publish_database()
            .times(1)
            .returning(|_| Ok(()));
        
        // Create the service
        let service = TagServiceImpl::new(
            Arc::new(mock_repo),
            Arc::new(mock_publisher),
        );
        
        // Call the method
        let result = service.update_and_publish_tag(
            "US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE",
            "50.2".to_string(),
        ).await;
        
        // Verify the result
        assert!(result.is_ok());
    }
}
