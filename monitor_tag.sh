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

echo "Monitoring tag: $TAG_PATH"
echo "MQTT Broker: $MQTT_HOST:$MQTT_PORT"
echo "Press Ctrl+C to stop monitoring"
echo "-----------------------------------"

# Install required packages in the container
docker exec -it uns_cli apt-get update
docker exec -it uns_cli apt-get install -y mosquitto-clients jq

# Copy the monitor_tag.sh script to the container
docker cp uns_cli/monitor_tag.sh uns_cli:/usr/src/uns_cli/monitor_tag.sh
docker exec -it uns_cli chmod +x /usr/src/uns_cli/monitor_tag.sh

# Run the monitor_tag.sh script inside the Docker container
docker exec -it uns_cli /bin/bash -c "cd /usr/src/uns_cli && ./monitor_tag.sh $TAG_PATH $MQTT_HOST $MQTT_PORT"
