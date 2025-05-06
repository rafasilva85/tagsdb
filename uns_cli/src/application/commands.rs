use std::sync::Arc;

use crate::domain::TagService;
use crate::infrastructure::UnsError;

/// Command handler trait
pub trait CommandHandler: Send + Sync {
    /// Executes the command
    fn execute(&self) -> impl std::future::Future<Output = Result<(), UnsError>> + Send;
}

/// Run command handler
pub struct RunCommandHandler {
    tag_service: Arc<dyn TagService>,
    tags_file: String,
    #[cfg(test)]
    test_mode: bool,
}

impl RunCommandHandler {
    /// Creates a new RunCommandHandler
    pub fn new(tag_service: Arc<dyn TagService>, tags_file: String) -> Self {
        Self {
            tag_service,
            tags_file,
            #[cfg(test)]
            test_mode: false,
        }
    }
    
    #[cfg(test)]
    /// Creates a new RunCommandHandler in test mode
    pub fn new_test_mode(tag_service: Arc<dyn TagService>, tags_file: String) -> Self {
        Self {
            tag_service,
            tags_file,
            test_mode: true,
        }
    }
}

impl CommandHandler for RunCommandHandler {
    fn execute(&self) -> impl std::future::Future<Output = Result<(), UnsError>> + Send {
        async move {
            println!("Starting UNS CLI...");
            println!("Loading tags from: {}", self.tags_file);
            
            match self.tag_service.load_and_publish_tags(&self.tags_file).await {
                Ok(_) => {
                    println!("Tags loaded and published successfully.");
                    println!("UNS CLI running. Waiting for updates or termination...");
                    
                    // In test mode, we don't wait for Ctrl+C
                    #[cfg(test)]
                    if self.test_mode {
                        println!("Test mode: not waiting for Ctrl+C");
                        return Ok(());
                    }
                    
                    // Keep the application running indefinitely
                    // In a real application, you might have a loop listening for external updates or commands
                    tokio::signal::ctrl_c().await?;
                    println!("Shutting down...");
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Error loading tags: {}", e);
                    Err(e)
                }
            }
        }
    }
}

/// Update command handler
pub struct UpdateCommandHandler {
    tag_service: Arc<dyn TagService>,
    path: String,
    value: String,
}

impl UpdateCommandHandler {
    /// Creates a new UpdateCommandHandler
    pub fn new(tag_service: Arc<dyn TagService>, path: String, value: String) -> Self {
        Self {
            tag_service,
            path,
            value,
        }
    }
}

impl CommandHandler for UpdateCommandHandler {
    fn execute(&self) -> impl std::future::Future<Output = Result<(), UnsError>> + Send {
        async move {
            println!("Attempting to update tag: {} with value: {}", self.path, self.value);
            
            match self.tag_service.update_and_publish_tag(&self.path, self.value.clone()).await {
                Ok(_) => {
                    println!("Tag updated successfully: {} = {}", self.path, self.value);
                    
                    // Wait a moment to ensure the update is published
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    
                    println!("Update command finished.");
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Error updating tag: {}", e);
                    Err(e)
                }
            }
        }
    }
}

/// Command factory for creating command handlers
pub struct CommandFactory {
    tag_service: Arc<dyn TagService>,
}

impl CommandFactory {
    /// Creates a new CommandFactory
    pub fn new(tag_service: Arc<dyn TagService>) -> Self {
        Self { tag_service }
    }
    
    /// Creates a RunCommandHandler
    pub fn create_run_command(&self, tags_file: String) -> RunCommandHandler {
        RunCommandHandler::new(self.tag_service.clone(), tags_file)
    }
    
    #[cfg(test)]
    /// Creates a RunCommandHandler in test mode
    pub fn create_run_command_test_mode(&self, tags_file: String) -> RunCommandHandler {
        RunCommandHandler::new_test_mode(self.tag_service.clone(), tags_file)
    }
    
    /// Creates an UpdateCommandHandler
    pub fn create_update_command(&self, path: String, value: String) -> UpdateCommandHandler {
        UpdateCommandHandler::new(self.tag_service.clone(), path, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Tag;
    use async_trait::async_trait;
    use mockall::predicate::*;
    use mockall::*;
    use std::collections::HashMap;
    
    // Mock the TagService
    mock! {
        pub TagService {}
        
        #[async_trait]
        impl TagService for TagService {
            async fn load_and_publish_tags(&self, source: &str) -> Result<(), UnsError>;
            async fn update_and_publish_tag(&self, path: &str, value: String) -> Result<(), UnsError>;
            async fn get_all_tags(&self) -> Result<HashMap<String, Tag>, UnsError>;
            async fn get_tag(&self, path: &str) -> Result<Option<Tag>, UnsError>;
        }
    }
    
    #[tokio::test]
    async fn test_run_command() {
        // Create mock tag service
        let mut mock_service = MockTagService::new();
        
        // Set up expectations
        mock_service
            .expect_load_and_publish_tags()
            .with(eq("test.json"))
            .times(1)
            .returning(|_| Ok(()));
        
        // Create the command handler in test mode
        let handler = RunCommandHandler::new_test_mode(
            Arc::new(mock_service),
            "test.json".to_string(),
        );
        
        // Call the method
        let result = handler.execute().await;
        
        // Verify the result
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_update_command() {
        // Create mock tag service
        let mut mock_service = MockTagService::new();
        
        // Set up expectations
        mock_service
            .expect_update_and_publish_tag()
            .with(
                eq("US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE"),
                eq("50.2".to_string())
            )
            .times(1)
            .returning(|_, _| Ok(()));
        
        // Create the command handler
        let handler = UpdateCommandHandler::new(
            Arc::new(mock_service),
            "US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE".to_string(),
            "50.2".to_string(),
        );
        
        // Call the method
        let result = handler.execute().await;
        
        // Verify the result
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_command_factory() {
        // Create mock tag service
        let mock_service = MockTagService::new();
        
        // Create the factory
        let factory = CommandFactory::new(Arc::new(mock_service));
        
        // Create a run command
        let run_command = factory.create_run_command("test.json".to_string());
        
        // Create an update command
        let update_command = factory.create_update_command(
            "US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE".to_string(),
            "50.2".to_string(),
        );
        
        // Verify the commands were created correctly
        assert_eq!(run_command.tags_file, "test.json");
        assert_eq!(update_command.path, "US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE");
        assert_eq!(update_command.value, "50.2");
    }
}
