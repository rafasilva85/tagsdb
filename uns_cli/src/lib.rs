// UNS CLI Library
// This file exports the public API of the library

// Module declarations
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;

// Re-export key types for easier access
pub use domain::{Tag, TagRepository, TagService};
pub use application::TagServiceImpl;
pub use infrastructure::{
    mqtt::{MqttClient, MqttPublisher},
    repositories::JsonTagRepository,
    UnsError,
};
pub use presentation::Cli;
