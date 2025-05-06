#!/bin/bash

# Script to run the UNS system
# Usage: ./run_uns_system.sh [tags_file] [mqtt_host] [mqtt_port]

# Default values
TAGS_FILE=${1:-"tags.json"}
MQTT_HOST=${2:-"hivemq"}
MQTT_PORT=${3:-1883}

echo "Starting UNS system..."
echo "Tags file: $TAGS_FILE"
echo "MQTT Broker: $MQTT_HOST:$MQTT_PORT"
echo "-----------------------------------"

# Check if the tags file exists
if [ ! -f "$TAGS_FILE" ]; then
    echo "Error: Tags file not found: $TAGS_FILE"
    exit 1
fi

# Run the UNS CLI in run mode
echo "This script is intended to be run inside the Docker container."
echo "The UNS CLI is already running in the Docker container."
