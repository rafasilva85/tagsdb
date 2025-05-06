# UNS (Unified Naming System)

This project implements a Unified Naming System (UNS) for industrial automation systems. It provides a way to manage and monitor tags in a distributed system using MQTT.

## System Architecture

The UNS system consists of the following components:

1. **UNS CLI**: A command-line interface for managing tags
2. **HiveMQ**: An MQTT broker for communication between components
3. **Monitoring Tools**: Scripts for monitoring and updating tags

## Getting Started

### Prerequisites

- Docker and Docker Compose installed
- Python 3.6+ with the `paho-mqtt` package installed (for Python scripts)

### Running the System

To start the UNS system, run:

```bash
./run_uns_system.sh
```

This will start the Docker containers for the UNS CLI and HiveMQ.

## Available Tools

### 1. Extract Available Tags

The `extract_tags.sh` script extracts all available tags from the MQTT broker and saves them to a file.

```bash
./extract_tags.sh [mqtt_host] [mqtt_port]
```

- `mqtt_host`: MQTT broker host (default: "hivemq")
- `mqtt_port`: MQTT broker port (default: 1883)

This script will:
- Extract the tag paths and save them to `available_tags.txt`
- Save the full tag database to `tags_database.json`

### 2. Monitor a Specific Tag

The `monitor_tag.sh` script monitors changes to a specific tag.

```bash
./monitor_tag.sh <tag_path> [mqtt_host] [mqtt_port]
```

- `tag_path`: Path of the tag to monitor (required)
- `mqtt_host`: MQTT broker host (default: "hivemq")
- `mqtt_port`: MQTT broker port (default: 1883)

This script will subscribe to the tag's topic and print any changes to the tag's value.

### 3. Monitor All Tag Changes

The `monitor_tag_changes.sh` script monitors changes to all tags in the system.

```bash
./monitor_tag_changes.sh [mqtt_host] [mqtt_port]
```

- `mqtt_host`: MQTT broker host (default: "hivemq")
- `mqtt_port`: MQTT broker port (default: 1883)

This script will:
- Subscribe to all tag topics
- Print new tags and tag updates with color-coded output
- Track the last known value of each tag

### 4. Update a Tag

The `update_tag.sh` script updates a tag.

```bash
./update_tag.sh <tag_path> <value> [mqtt_host] [mqtt_port]
```

- `tag_path`: Path of the tag to update (required)
- `value`: New value for the tag (required)
- `mqtt_host`: MQTT broker host (default: "hivemq")
- `mqtt_port`: MQTT broker port (default: 1883)

This script will publish a message to the tag's topic with the new value.

## Stopping the System

To stop the UNS system, run:

```bash
docker-compose down
```

## Tag Format

Tags in the UNS system have the following format:

```json
{
  "path": "US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE",
  "name": "Pump 1 Pressure",
  "description": "Pressure sensor for Pump 1",
  "value": "45.7"
}
```

- `path`: The hierarchical path of the tag
- `name`: A human-readable name for the tag
- `description`: A description of the tag
- `value`: The current value of the tag

## MQTT Topics

The UNS system uses the following MQTT topics:

- `tags/<tag_path>`: Individual tag topics, where `<tag_path>` is the tag path with slashes replaced by dots
- `tags/database`: Full tag database topic
