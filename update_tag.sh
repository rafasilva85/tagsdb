#!/bin/bash

# Script to update a tag
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

# Install required packages in the container
docker exec -it uns_cli apt-get update
docker exec -it uns_cli apt-get install -y python3-pip
docker exec -it uns_cli pip3 install paho-mqtt

# Copy the update_tag.sh script and publish_tag.py to the container
docker cp uns_cli/update_tag.sh uns_cli:/usr/src/uns_cli/update_tag.sh
docker cp uns_cli/publish_tag.py uns_cli:/usr/src/uns_cli/publish_tag.py
docker exec -it uns_cli chmod +x /usr/src/uns_cli/update_tag.sh
docker exec -it uns_cli chmod +x /usr/src/uns_cli/publish_tag.py

# Run the update_tag.sh script inside the Docker container
docker exec -it uns_cli /bin/bash -c "cd /usr/src/uns_cli && ./update_tag.sh $TAG_PATH $VALUE $MQTT_HOST $MQTT_PORT"

echo "-----------------------------------"
echo "Tag update command completed."
