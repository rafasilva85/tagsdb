use std::sync::Arc;

use uns_cli::{
    application::TagServiceImpl,
    domain::TagService,
    infrastructure::{
        mqtt::{client::RumqttcClient, publisher::MqttTagPublisher, MqttClient, MqttPublisher},
        repositories::JsonTagRepository,
        UnsError,
    },
    presentation::cli::CliHandler,
};

#[tokio::main]
async fn main() -> Result<(), UnsError> {
    // Parse command-line arguments
    let cli = clap::Command::new("uns_cli")
        .version("0.1")
        .author("Your Name")
        .about("UNS CLI")
        .subcommand_required(true)
        .get_matches();
    
    // Get MQTT connection parameters
    let mqtt_host = match cli.subcommand() {
        Some(("run", args)) => args.get_one::<String>("mqtt-host").unwrap_or(&"hivemq".to_string()).clone(),
        Some(("update", args)) => args.get_one::<String>("mqtt-host").unwrap_or(&"localhost".to_string()).clone(),
        _ => "localhost".to_string(),
    };
    
    let mqtt_port = match cli.subcommand() {
        Some(("run", args)) => *args.get_one::<u16>("mqtt-port").unwrap_or(&1883),
        Some(("update", args)) => *args.get_one::<u16>("mqtt-port").unwrap_or(&1883),
        _ => 1883,
    };
    
    // Create the MQTT client
    let mqtt_client = RumqttcClient::new("uns_cli_publisher", &mqtt_host, mqtt_port).await?;
    let mqtt_client: Arc<dyn MqttClient> = Arc::new(mqtt_client);
    
    // Create the MQTT publisher
    let mqtt_publisher: Arc<dyn MqttPublisher> = Arc::new(MqttTagPublisher::new(mqtt_client));
    
    // Create the tag repository
    let tag_repository = JsonTagRepository::new();
    
    // Create the tag service
    let tag_service: Arc<dyn TagService> = Arc::new(TagServiceImpl::new(
        Arc::new(tag_repository),
        mqtt_publisher,
    ));
    
    // Create the CLI handler
    let cli_handler = CliHandler::new(tag_service);
    
    // Run the CLI
    cli_handler.run().await
}
