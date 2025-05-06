#!/bin/bash

# Script to extract available tags from the MQTT broker
# Usage: ./extract_tags.sh [mqtt_host] [mqtt_port]

# Default values
MQTT_HOST=${1:-"hivemq"}
MQTT_PORT=${2:-1883}

echo "Extracting tags from MQTT broker at $MQTT_HOST:$MQTT_PORT..."

# Use mosquitto_sub to subscribe to the tags/database topic and extract the tags
echo "Subscribing to tags/database topic..."
mosquitto_sub -h $MQTT_HOST -p $MQTT_PORT -t "tags/database" -C 1 > tags_database.json

# Extract the tag paths from the JSON
echo "Extracting tag paths..."
cat tags_database.json | jq -r '.tags | keys[]' > available_tags.txt

echo "Available tags have been extracted to available_tags.txt"
echo "Tags database has been saved to tags_database.json"
