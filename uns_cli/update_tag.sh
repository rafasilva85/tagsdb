#!/bin/bash

# Script to update a tag using the UNS CLI
# Usage: ./update_tag.sh <tag_path> <value> [mqtt_host] [mqtt_port]

# Check if tag path and value are provided
if [ -z "$1" ] || [ -z "$2" ]; then
    echo "Error: Tag path and value are required"
    echo "Usage: ./update_tag.sh <tag_path> <value> [mqtt_host] [mqtt_port]"
    exit 1
fi

# Get parameters
TAG_PATH=$1
VALUE=$2
MQTT_HOST=${3:-"hivemq"}
MQTT_PORT=${4:-1883}

echo "Updating tag: $TAG_PATH"
echo "New value: $VALUE"
echo "MQTT Broker: $MQTT_HOST:$MQTT_PORT"
echo "-----------------------------------"

# Use the publish_tag.py script to publish the tag update
python3 publish_tag.py $TAG_PATH $VALUE --host $MQTT_HOST --port $MQTT_PORT

echo "-----------------------------------"
echo "Tag update command completed."
