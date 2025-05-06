#!/usr/bin/env python3

"""
Script to publish a tag update to the UNS system.
This script connects to the MQTT broker and publishes a tag update.

Usage:
    python publish_tag.py <tag_path> <value> [--host MQTT_HOST] [--port MQTT_PORT]
"""

import argparse
import json
import time
from datetime import datetime
import paho.mqtt.client as mqtt

# ANSI color codes for terminal output
COLORS = {
    "RESET": "\033[0m",
    "RED": "\033[91m",
    "GREEN": "\033[92m",
    "YELLOW": "\033[93m",
    "BLUE": "\033[94m",
    "MAGENTA": "\033[95m",
    "CYAN": "\033[96m",
}

def on_connect(client, userdata, flags, rc):
    """Callback for when the client connects to the MQTT broker."""
    print(f"{COLORS['GREEN']}Connected to MQTT broker with result code {rc}{COLORS['RESET']}")

def publish_tag_update(client, tag_path, value):
    """Publishes a tag update to the MQTT broker."""
    # Convert tag path to MQTT topic
    topic = f"tags/{tag_path.replace('/', '.')}"
    
    # Create the payload
    payload = {
        "path": tag_path,
        "value": value,
        "timestamp": datetime.now().isoformat()
    }
    
    # Convert payload to JSON
    payload_json = json.dumps(payload)
    
    # Publish the message
    result = client.publish(topic, payload_json, qos=1, retain=True)
    
    # Check if the message was published successfully
    if result.rc == mqtt.MQTT_ERR_SUCCESS:
        print(f"{COLORS['GREEN']}Successfully published tag update:{COLORS['RESET']}")
        print(f"  Topic: {COLORS['CYAN']}{topic}{COLORS['RESET']}")
        print(f"  Path: {COLORS['CYAN']}{tag_path}{COLORS['RESET']}")
        print(f"  Value: {COLORS['YELLOW']}{value}{COLORS['RESET']}")
        print(f"  Timestamp: {COLORS['MAGENTA']}{payload['timestamp']}{COLORS['RESET']}")
    else:
        print(f"{COLORS['RED']}Failed to publish tag update: {result.rc}{COLORS['RESET']}")

def main():
    """Main function."""
    # Parse command line arguments
    parser = argparse.ArgumentParser(description="Publish a tag update to the UNS system")
    parser.add_argument("tag_path", help="Path of the tag to update")
    parser.add_argument("value", help="New value for the tag")
    parser.add_argument("--host", default="hivemq", help="MQTT broker host")
    parser.add_argument("--port", type=int, default=1883, help="MQTT broker port")
    args = parser.parse_args()
    
    # Create MQTT client
    client = mqtt.Client()
    client.on_connect = on_connect
    
    # Connect to MQTT broker
    print(f"{COLORS['CYAN']}Connecting to MQTT broker at {args.host}:{args.port}...{COLORS['RESET']}")
    client.connect(args.host, args.port, 60)
    
    # Start the MQTT loop
    client.loop_start()
    
    # Wait for the connection to be established
    time.sleep(1)
    
    # Publish the tag update
    publish_tag_update(client, args.tag_path, args.value)
    
    # Wait for the message to be published
    time.sleep(1)
    
    # Stop the MQTT loop and disconnect
    client.loop_stop()
    client.disconnect()

if __name__ == "__main__":
    main()
