# UNS CLI

A command-line interface for managing tags in the UNS system.

## Overview

UNS CLI is a Rust application that allows you to manage tags in the UNS system. It provides a command-line interface for loading tags from JSON files and publishing them to an MQTT broker, as well as updating tag values.

## Architecture

The project follows a clean architecture approach with the following layers:

### Domain Layer

The domain layer contains the core business entities and logic:

- `Tag`: Represents a tag in the UNS system
- `TagRepository`: Interface for tag data access
- `TagService`: Interface for tag operations

### Application Layer

The application layer contains the use cases and application logic:

- `TagServiceImpl`: Implementation of the `TagService` interface
- `CommandHandler`: Interface for command handlers
- `RunCommandHandler`: Handler for the `run` command
- `UpdateCommandHandler`: Handler for the `update` command
- `CommandFactory`: Factory for creating command handlers

### Infrastructure Layer

The infrastructure layer contains the external systems and implementations:

- `MqttClient`: Interface for MQTT client
- `RumqttcClient`: Implementation of the `MqttClient` interface using rumqttc
- `MqttPublisher`: Interface for MQTT publisher
- `MqttTagPublisher`: Implementation of the `MqttPublisher` interface
- `JsonTagRepository`: Implementation of the `TagRepository` interface using JSON files
- `UnsError`: Custom error type for UNS CLI

### Presentation Layer

The presentation layer contains the user interfaces:

- `Cli`: Command-line interface using clap
- `CliHandler`: Handler for the CLI

## SOLID Principles

The project follows the SOLID principles:

- **Single Responsibility Principle**: Each class has a single responsibility
- **Open/Closed Principle**: The code is open for extension but closed for modification
- **Liskov Substitution Principle**: Implementations can be substituted for their interfaces
- **Interface Segregation Principle**: Interfaces are focused on specific needs
- **Dependency Inversion Principle**: High-level modules depend on abstractions

## Design Patterns

The project uses the following design patterns:

- **Repository Pattern**: Abstracts data access through the `TagRepository` interface
- **Dependency Injection**: Dependencies are passed through constructors
- **Command Pattern**: Encapsulates CLI commands as objects
- **Factory Pattern**: Creates command handlers
- **Observer Pattern**: For MQTT publish/subscribe mechanism

## Usage

### Building

```bash
cargo build --release
```

### Running

```bash
# Load tags from a JSON file and keep running
cargo run -- run --tags-file tags.json --mqtt-host hivemq --mqtt-port 1883

# Update a tag value
cargo run -- update US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE 50.2 --mqtt-host localhost --mqtt-port 1883
```

### Testing

```bash
cargo test
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
