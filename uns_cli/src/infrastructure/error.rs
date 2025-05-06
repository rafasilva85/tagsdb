use std::fmt;
use std::error::Error;

/// Custom error type for UNS CLI
#[derive(Debug)]
pub enum UnsError {
    /// Error when loading or saving tags
    Repository(String),
    
    /// Error when communicating with MQTT broker
    Mqtt(String),
    
    /// Error when serializing or deserializing data
    Serialization(String),
    
    /// Error when a tag is not found
    NotFound(String),
    
    /// Any other error
    Other(String),
}

impl fmt::Display for UnsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnsError::Repository(msg) => write!(f, "Repository error: {}", msg),
            UnsError::Mqtt(msg) => write!(f, "MQTT error: {}", msg),
            UnsError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            UnsError::NotFound(msg) => write!(f, "Not found: {}", msg),
            UnsError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl Error for UnsError {}

impl From<std::io::Error> for UnsError {
    fn from(error: std::io::Error) -> Self {
        UnsError::Repository(error.to_string())
    }
}

impl From<serde_json::Error> for UnsError {
    fn from(error: serde_json::Error) -> Self {
        UnsError::Serialization(error.to_string())
    }
}

impl From<rumqttc::ClientError> for UnsError {
    fn from(error: rumqttc::ClientError) -> Self {
        UnsError::Mqtt(error.to_string())
    }
}

impl From<String> for UnsError {
    fn from(error: String) -> Self {
        UnsError::Other(error)
    }
}

impl From<&str> for UnsError {
    fn from(error: &str) -> Self {
        UnsError::Other(error.to_string())
    }
}
