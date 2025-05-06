use async_trait::async_trait;
use serde::Serialize;
use std::{collections::HashMap, sync::Arc};

use crate::domain::Tag;
use crate::infrastructure::UnsError;
use crate::infrastructure::mqtt::MqttClient;

/// Tag database structure for serialization
#[derive(Serialize)]
pub struct TagDatabase {
    pub tags: HashMap<String, Tag>,
}

/// MQTT publisher interface
#[async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait MqttPublisher: Send + Sync {
    /// Publishes a tag to its individual topic
    async fn publish_tag(&self, tag: &Tag) -> Result<(), UnsError>;
    
    /// Publishes a collection of tags to their individual topics
    async fn publish_tags(&self, tags: &[Tag]) -> Result<(), UnsError>;
    
    /// Publishes the full tag database to a single topic
    async fn publish_database(&self, data: &TagDatabase) -> Result<(), UnsError>;
}

/// Implementation of MQTT publisher
pub struct MqttTagPublisher {
    client: Arc<dyn MqttClient>,
}

impl MqttTagPublisher {
    /// Creates a new MQTT publisher
    pub fn new(client: Arc<dyn MqttClient>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl MqttPublisher for MqttTagPublisher {
    async fn publish_tag(&self, tag: &Tag) -> Result<(), UnsError> {
        let topic = tag.to_mqtt_topic();
        let payload = serde_json::to_string(tag)?;
        
        self.client.publish(&topic, payload.into_bytes(), true).await
    }
    
    async fn publish_tags(&self, tags: &[Tag]) -> Result<(), UnsError> {
        for tag in tags {
            self.publish_tag(tag).await?;
        }
        Ok(())
    }
    
    async fn publish_database(&self, data: &TagDatabase) -> Result<(), UnsError> {
        let payload = serde_json::to_string(data)?;
        self.client.publish("tags/database", payload.into_bytes(), true).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::mqtt::client::MockMqttClient;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_publish_tag() {
        let mut mock_client = MockMqttClient::new();
        
        // Set up expectations
        mock_client
            .expect_publish()
            .with(
                eq("tags/US.TX.AUSTIN.AREA1.LINE1.MACHINE1.PUMP1.PRESSURE"),
                always(),
                eq(true)
            )
            .times(1)
            .returning(|_, _, _| Box::pin(async { Ok(()) }));
        
        let publisher = MqttTagPublisher::new(Arc::new(mock_client));
        
        let tag = Tag::new(
            "US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE".to_string(),
            "Pump 1 Pressure".to_string(),
            "Pressure sensor for Pump 1".to_string(),
            "45.7".to_string(),
        );
        
        let result = publisher.publish_tag(&tag).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_publish_database() {
        let mut mock_client = MockMqttClient::new();
        
        // Set up expectations
        mock_client
            .expect_publish()
            .with(
                eq("tags/database"),
                always(),
                eq(true)
            )
            .times(1)
            .returning(|_, _, _| Box::pin(async { Ok(()) }));
        
        let publisher = MqttTagPublisher::new(Arc::new(mock_client));
        
        let mut tags = HashMap::new();
        tags.insert(
            "test/tag".to_string(),
            Tag::new(
                "test/tag".to_string(),
                "Test Tag".to_string(),
                "A test tag".to_string(),
                "test".to_string(),
            ),
        );
        
        let data = TagDatabase { tags };
        
        let result = publisher.publish_database(&data).await;
        assert!(result.is_ok());
    }
}
