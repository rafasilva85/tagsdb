#!/bin/bash

# Script to monitor changes to all tags
# Usage: ./monitor_tag_changes.sh [mqtt_host] [mqtt_port]

# Default values
MQTT_HOST=${1:-"hivemq"}
MQTT_PORT=${2:-1883}

echo "Monitoring all tag changes on MQTT broker at $MQTT_HOST:$MQTT_PORT..."
echo "Press Ctrl+C to stop monitoring"
echo "-----------------------------------"

# Install required packages in the container
docker exec -it uns_cli apt-get update
docker exec -it uns_cli apt-get install -y python3-pip
docker exec -it uns_cli pip3 install paho-mqtt

# Copy the monitor_tag_changes.py script to the container
docker cp uns_cli/monitor_tag_changes.py uns_cli:/usr/src/uns_cli/monitor_tag_changes.py

# Run the monitor_tag_changes.py script inside the Docker container
docker exec -it uns_cli /bin/bash -c "cd /usr/src/uns_cli && python3 monitor_tag_changes.py --host $MQTT_HOST --port $MQTT_PORT"
