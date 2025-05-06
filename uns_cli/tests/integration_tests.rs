use std::{collections::HashSet, sync::Arc, time::Duration};

use rumqttc::{AsyncClient, MqttOptions, QoS};
use serde_json;
use tokio;

use uns_cli::{
    application::TagServiceImpl,
    domain::{Tag, TagService},
    infrastructure::{
        mqtt::{client::RumqttcClient, publisher::MqttTagPublisher, MqttClient, MqttPublisher},
        repositories::JsonTagRepository,
    },
};

// Helper function to create a test MQTT client
async fn create_test_mqtt_client(client_id: &str) -> (AsyncClient, rumqttc::EventLoop) {
    let mut mqtt_options = MqttOptions::new(client_id, "localhost", 1883);
    mqtt_options.set_keep_alive(Duration::from_secs(5));
    AsyncClient::new(mqtt_options, 10)
}

#[tokio::test]
async fn test_end_to_end_tag_loading() {
    // Create the components
    let mqtt_client = RumqttcClient::new("test_client", "localhost", 1883).await.unwrap();
    let mqtt_client: Arc<dyn MqttClient> = Arc::new(mqtt_client);
    
    let mqtt_publisher: Arc<dyn MqttPublisher> = Arc::new(MqttTagPublisher::new(mqtt_client));
    
    let tag_repository = JsonTagRepository::new();
    
    let tag_service: Arc<dyn TagService> = Arc::new(TagServiceImpl::new(
        Arc::new(tag_repository),
        mqtt_publisher,
    ));
    
    // Create a test MQTT client to subscribe to tag topics
    let (client, mut eventloop) = create_test_mqtt_client("test_subscriber").await;
    
    // Subscribe to all tag topics
    client.subscribe("tags/#", QoS::AtLeastOnce).await.unwrap();
    
    // Set up a channel to receive MQTT messages
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    
    // Spawn a task to process MQTT events
    tokio::spawn(async move {
        while let Ok(notification) = eventloop.poll().await {
            if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(publish)) = notification {
                let _ = tx.send(publish).await;
            }
        }
    });
    
    // Wait a moment for the subscription to be established
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    // Load tags from the test JSON file
    tag_service.load_and_publish_tags("tests/test_tags.json").await.unwrap();
    
    // Wait for and verify messages
    let mut received_tags = HashSet::new();
    let mut database_received = false;
    
    // Wait for a reasonable time to receive all messages
    for _ in 0..10 {
        if let Ok(publish) = tokio::time::timeout(
            Duration::from_secs(1), 
            rx.recv()
        ).await {
            if let Some(publish) = publish {
                // Extract tag path from topic
                let topic = publish.topic.clone();
                if topic == "tags/database" {
                    database_received = true;
                } else if topic.starts_with("tags/") {
                    let tag_path = topic.replace("tags/", "").replace(".", "/");
                    received_tags.insert(tag_path);
                }
            }
        }
    }
    
    // Verify all tags were published
    assert!(received_tags.contains("US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE"));
    assert!(received_tags.contains("US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/STATUS"));
    assert!(database_received, "Full database was not published");
}

#[tokio::test]
async fn test_end_to_end_tag_update() {
    // Create the components
    let mqtt_client = RumqttcClient::new("test_client_update", "localhost", 1883).await.unwrap();
    let mqtt_client: Arc<dyn MqttClient> = Arc::new(mqtt_client);
    
    let mqtt_publisher: Arc<dyn MqttPublisher> = Arc::new(MqttTagPublisher::new(mqtt_client));
    
    let tag_repository = JsonTagRepository::new();
    
    let tag_service: Arc<dyn TagService> = Arc::new(TagServiceImpl::new(
        Arc::new(tag_repository),
        mqtt_publisher,
    ));
    
    // Create a test MQTT client to subscribe to tag topics
    let (client, mut eventloop) = create_test_mqtt_client("test_subscriber_update").await;
    
    // Subscribe to the specific tag topic we'll update
    let test_tag_path = "US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE";
    let test_tag_topic = format!("tags/{}", test_tag_path.replace("/", "."));
    client.subscribe(&test_tag_topic, QoS::AtLeastOnce).await.unwrap();
    
    // Also subscribe to the database topic to verify the update is reflected there
    client.subscribe("tags/database", QoS::AtLeastOnce).await.unwrap();
    
    // Set up a channel to receive MQTT messages
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    
    // Spawn a task to process MQTT events
    tokio::spawn(async move {
        while let Ok(notification) = eventloop.poll().await {
            if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(publish)) = notification {
                let _ = tx.send(publish).await;
            }
        }
    });
    
    // Wait a moment for the subscription to be established
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    // Load tags from the test JSON file
    tag_service.load_and_publish_tags("tests/test_tags.json").await.unwrap();
    
    // Clear initial messages
    while let Ok(Some(_)) = tokio::time::timeout(
        Duration::from_millis(500), 
        rx.recv()
    ).await {}
    
    // Update the tag
    let new_value = "75.3";
    tag_service.update_and_publish_tag(test_tag_path, new_value.to_string()).await.unwrap();
    
    // Wait for the update message
    let mut updated_value_received = false;
    let mut database_updated = false;
    
    for _ in 0..10 {
        if let Ok(Some(publish)) = tokio::time::timeout(
            Duration::from_secs(1), 
            rx.recv()
        ).await {
            if publish.topic == test_tag_topic {
                let payload = String::from_utf8_lossy(&publish.payload);
                let tag: Tag = serde_json::from_str(&payload).unwrap();
                if tag.value == new_value {
                    updated_value_received = true;
                }
            } else if publish.topic == "tags/database" {
                let payload = String::from_utf8_lossy(&publish.payload);
                let data: serde_json::Value = serde_json::from_str(&payload).unwrap();
                
                // Check if the database contains the updated tag value
                if let Some(tags) = data.get("tags") {
                    if let Some(tag) = tags.get(test_tag_path) {
                        if let Some(value) = tag.get("value") {
                            if value.as_str() == Some(new_value) {
                                database_updated = true;
                            }
                        }
                    }
                }
            }
            
            // If we've verified both conditions, we can break early
            if updated_value_received && database_updated {
                break;
            }
        }
    }
    
    assert!(updated_value_received, "Tag update was not propagated through MQTT");
    assert!(database_updated, "Tag update was not reflected in the database publication");
}
