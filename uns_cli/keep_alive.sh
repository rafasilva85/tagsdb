#!/bin/bash

# Script to keep the UNS CLI running in the background
# Usage: ./keep_alive.sh [tags_file] [mqtt_host] [mqtt_port]

# Default values
TAGS_FILE=${1:-"/usr/src/uns_cli/tags.json"}
MQTT_HOST=${2:-"hivemq"}
MQTT_PORT=${3:-1883}

echo "Starting UNS CLI in background mode..."
echo "Tags file: $TAGS_FILE"
echo "MQTT Broker: $MQTT_HOST:$MQTT_PORT"
echo "-----------------------------------"

# Debug information
echo "Current directory: $(pwd)"
echo "Listing files in current directory:"
ls -la

# Check if the tags file exists
if [ ! -f "$TAGS_FILE" ]; then
    echo "Error: Tags file not found: $TAGS_FILE"
    echo "Trying to find the file:"
    find / -name tags.json 2>/dev/null
    exit 1
fi

# Run the UNS CLI in run mode
echo "Running UNS CLI..."
uns_cli run --tags-file $TAGS_FILE --mqtt-host $MQTT_HOST --mqtt-port $MQTT_PORT &

# Keep the container running
echo "UNS CLI is running in the background. Container will stay alive."
tail -f /dev/null
