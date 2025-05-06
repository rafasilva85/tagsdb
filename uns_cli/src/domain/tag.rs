use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a tag in the UNS system
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Tag {
    /// Hierarchical path of the tag (e.g., "US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE")
    pub path: String,
    
    /// Human-readable name of the tag
    pub name: String,
    
    /// Description of what the tag represents
    pub description: String,
    
    /// Current value of the tag as a string
    pub value: String,
    
    // Potential future fields (commented out for now)
    // pub quality: String,
    // #[serde(with = "chrono::serde::ts_seconds")]
    // pub timestamp: DateTime<Utc>,
    // pub units: Option<String>,
    // pub min: Option<String>,
    // pub max: Option<String>,
    // pub alarm_low: Option<f64>,
    // pub alarm_high: Option<f64>,
    // pub keywords: Vec<String>,
}

impl Tag {
    /// Creates a new tag with the given properties
    pub fn new(path: String, name: String, description: String, value: String) -> Self {
        Self {
            path,
            name,
            description,
            value,
        }
    }
    
    /// Updates the value of the tag
    pub fn update_value(&mut self, new_value: String) -> String {
        let old_value = self.value.clone();
        self.value = new_value;
        old_value
    }
    
    /// Converts the tag path to an MQTT topic format (replacing '/' with '.')
    pub fn to_mqtt_topic(&self) -> String {
        format!("tags/{}", self.path.replace("/", "."))
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tag {{ path: {}, name: {}, description: {}, value: {} }}",
            self.path, self.name, self.description, self.value
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tag() {
        let tag = Tag::new(
            "US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE".to_string(),
            "Pump 1 Pressure".to_string(),
            "Pressure sensor for Pump 1".to_string(),
            "45.7".to_string(),
        );
        
        assert_eq!(tag.path, "US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE");
        assert_eq!(tag.name, "Pump 1 Pressure");
        assert_eq!(tag.description, "Pressure sensor for Pump 1");
        assert_eq!(tag.value, "45.7");
    }
    
    #[test]
    fn test_update_value() {
        let mut tag = Tag::new(
            "US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE".to_string(),
            "Pump 1 Pressure".to_string(),
            "Pressure sensor for Pump 1".to_string(),
            "45.7".to_string(),
        );
        
        let old_value = tag.update_value("50.2".to_string());
        
        assert_eq!(old_value, "45.7");
        assert_eq!(tag.value, "50.2");
    }
    
    #[test]
    fn test_to_mqtt_topic() {
        let tag = Tag::new(
            "US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE".to_string(),
            "Pump 1 Pressure".to_string(),
            "Pressure sensor for Pump 1".to_string(),
            "45.7".to_string(),
        );
        
        assert_eq!(
            tag.to_mqtt_topic(),
            "tags/US.TX.AUSTIN.AREA1.LINE1.MACHINE1.PUMP1.PRESSURE"
        );
    }
}
