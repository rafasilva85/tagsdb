use clap::{Parser, Subcommand};
use std::sync::Arc;

use crate::application::commands::{CommandFactory, CommandHandler};
use crate::domain::TagService;
use crate::infrastructure::UnsError;

/// UNS CLI command-line interface
#[derive(Parser, Debug)]
#[clap(author = "Your Name", version = "0.1", about = "UNS CLI", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

/// CLI commands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Loads tags from a JSON file and keeps running
    Run {
        #[clap(long, value_parser, default_value = "tags.json")]
        tags_file: String,
        
        #[clap(long, value_parser, default_value = "hivemq")]
        mqtt_host: String,
        
        #[clap(long, value_parser, default_value_t = 1883)]
        mqtt_port: u16,
    },
    
    /// Updates a tag value (for testing, requires a running instance)
    Update {
        #[clap(value_parser)]
        path: String,
        
        #[clap(value_parser)]
        value: String,
        
        #[clap(long, value_parser, default_value = "localhost")]
        mqtt_host: String,
        
        #[clap(long, value_parser, default_value_t = 1883)]
        mqtt_port: u16,
    },
}

/// CLI handler
pub struct CliHandler {
    command_factory: CommandFactory,
}

impl CliHandler {
    /// Creates a new CLI handler
    pub fn new(tag_service: Arc<dyn TagService>) -> Self {
        Self {
            command_factory: CommandFactory::new(tag_service),
        }
    }
    
    /// Runs the CLI
    pub async fn run(&self) -> Result<(), UnsError> {
        let cli = Cli::parse();
        
        match cli.command {
            Commands::Run { tags_file, .. } => {
                let command = self.command_factory.create_run_command(tags_file);
                command.execute().await
            }
            Commands::Update { path, value, .. } => {
                let command = self.command_factory.create_update_command(path, value);
                command.execute().await
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::tag_service::MockTagService;
    use mockall::predicate::*;
    
    #[test]
    fn test_cli_parsing_run() {
        let args = vec!["uns_cli", "run", "--tags-file", "test.json"];
        let cli = Cli::parse_from(args);
        
        match cli.command {
            Commands::Run { tags_file, .. } => {
                assert_eq!(tags_file, "test.json");
            }
            _ => panic!("Expected Run command"),
        }
    }
    
    #[test]
    fn test_cli_parsing_update() {
        let args = vec![
            "uns_cli",
            "update",
            "US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE",
            "50.2",
        ];
        let cli = Cli::parse_from(args);
        
        match cli.command {
            Commands::Update { path, value, .. } => {
                assert_eq!(path, "US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE");
                assert_eq!(value, "50.2");
            }
            _ => panic!("Expected Update command"),
        }
    }
    
    #[tokio::test]
    async fn test_cli_handler_run() {
        // Create mock tag service
        let mut mock_service = MockTagService::new();
        
        // Set up expectations
        mock_service
            .expect_load_and_publish_tags()
            .with(eq("test.json"))
            .times(1)
            .returning(|_| Box::pin(async { Ok(()) }));
        
        // Create the CLI handler
        let handler = CliHandler::new(Arc::new(mock_service));
        
        // We can't easily test the actual CLI parsing, but we can test the command execution
        let command = handler.command_factory.create_run_command_test_mode("test.json".to_string());
        let result = command.execute().await;
        
        // Verify the result
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_cli_handler_update() {
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
            .returning(|_, _| Box::pin(async { Ok(()) }));
        
        // Create the CLI handler
        let handler = CliHandler::new(Arc::new(mock_service));
        
        // We can't easily test the actual CLI parsing, but we can test the command execution
        let command = handler.command_factory.create_update_command(
            "US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE".to_string(),
            "50.2".to_string(),
        );
        let result = command.execute().await;
        
        // Verify the result
        assert!(result.is_ok());
    }
}
