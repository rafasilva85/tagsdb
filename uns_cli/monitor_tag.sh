#!/bin/bash

# Script to monitor changes to a specific tag
# Usage: ./monitor_tag.sh <tag_path> [mqtt_host] [mqtt_port]

# Check if tag path is provided
if [ -z "$1" ]; then
    echo "Error: Tag path is required"
    echo "Usage: ./monitor_tag.sh <tag_path> [mqtt_host] [mqtt_port]"
    exit 1
fi

# Get parameters
TAG_PATH=$1
MQTT_HOST=${2:-"hivemq"}
MQTT_PORT=${3:-1883}

# Convert tag path to MQTT topic
MQTT_TOPIC="tags/$(echo $TAG_PATH | tr '/' '.')"

echo "Monitoring tag: $TAG_PATH"
echo "MQTT Topic: $MQTT_TOPIC"
echo "MQTT Broker: $MQTT_HOST:$MQTT_PORT"
echo "Press Ctrl+C to stop monitoring"
echo "-----------------------------------"

# Use mosquitto_sub to subscribe to the tag's topic
mosquitto_sub -h $MQTT_HOST -p $MQTT_PORT -t "$MQTT_TOPIC" -v | \
while read -r topic message; do
  # Extract the timestamp and value from the JSON
  TIMESTAMP=$(date +"%Y-%m-%d %H:%M:%S")
  VALUE=$(echo "$message" | jq -r '.value')
  
  # Print the timestamp and value
  echo "[$TIMESTAMP] $TAG_PATH = $VALUE"
done
