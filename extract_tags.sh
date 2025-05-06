#!/bin/bash

# Script to extract available tags from the MQTT broker
# Usage: ./extract_tags.sh [mqtt_host] [mqtt_port]

# Default values
MQTT_HOST=${1:-"hivemq"}
MQTT_PORT=${2:-1883}

echo "Extracting tags from MQTT broker at $MQTT_HOST:$MQTT_PORT..."

# Install required packages in the container
docker exec -it uns_cli apt-get update
docker exec -it uns_cli apt-get install -y mosquitto-clients jq

# Copy the extract_tags.sh script to the container
docker cp uns_cli/extract_tags.sh uns_cli:/usr/src/uns_cli/extract_tags.sh
docker exec -it uns_cli chmod +x /usr/src/uns_cli/extract_tags.sh

# Run the extract_tags.sh script inside the Docker container
docker exec -it uns_cli /bin/bash -c "cd /usr/src/uns_cli && ./extract_tags.sh $MQTT_HOST $MQTT_PORT"

# Copy the output files from the container
docker cp uns_cli:/usr/src/uns_cli/tags_database.json ./tags_database.json
docker cp uns_cli:/usr/src/uns_cli/available_tags.txt ./available_tags.txt

echo "Available tags have been extracted to available_tags.txt"
echo "Tags database has been saved to tags_database.json"
