use async_trait::async_trait;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::time::Duration;
use tokio;

use crate::infrastructure::UnsError;

/// MQTT client interface
#[async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait MqttClient: Send + Sync {
    /// Publishes a message to a topic
    async fn publish(&self, topic: &str, payload: Vec<u8>, retain: bool) -> Result<(), UnsError>;
    
    /// Subscribes to a topic
    async fn subscribe(&self, topic: &str) -> Result<(), UnsError>;
}

/// Implementation of MQTT client using rumqttc
pub struct RumqttcClient {
    client: AsyncClient,
}

impl RumqttcClient {
    /// Creates a new MQTT client
    pub async fn new(client_id: &str, host: &str, port: u16) -> Result<Self, UnsError> {
        let mut mqtt_options = MqttOptions::new(client_id, host, port);
        mqtt_options.set_keep_alive(Duration::from_secs(5));

        let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);

        // Spawn the event loop in a separate task
        tokio::spawn(async move {
            loop {
                match eventloop.poll().await {
                    Ok(_notification) => { /* Handle notifications if needed */ }
                    Err(e) => {
                        eprintln!("Error in MQTT event loop: {:?}", e);
                        // Implement reconnection logic here if needed
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    }
                }
            }
        });

        Ok(Self { client })
    }
}

#[async_trait]
impl MqttClient for RumqttcClient {
    async fn publish(&self, topic: &str, payload: Vec<u8>, retain: bool) -> Result<(), UnsError> {
        self.client
            .publish(topic, QoS::AtLeastOnce, retain, payload)
            .await
            .map_err(|e| UnsError::Mqtt(e.to_string()))
    }

    async fn subscribe(&self, topic: &str) -> Result<(), UnsError> {
        self.client
            .subscribe(topic, QoS::AtLeastOnce)
            .await
            .map_err(|e| UnsError::Mqtt(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;

    mock! {
        pub MqttClient {}
        
        #[async_trait]
        impl MqttClient for MqttClient {
            async fn publish(&self, topic: &str, payload: Vec<u8>, retain: bool) -> Result<(), UnsError>;
            async fn subscribe(&self, topic: &str) -> Result<(), UnsError>;
        }
    }
}
