#!/usr/bin/env python3

"""
Script to monitor changes to all tags in the UNS system.
This script subscribes to the MQTT broker and listens for tag updates.

Usage:
    python monitor_tag_changes.py [--host MQTT_HOST] [--port MQTT_PORT]
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

# Store the last known values of tags
last_values = {}

def on_connect(client, userdata, flags, rc):
    """Callback for when the client connects to the MQTT broker."""
    print(f"{COLORS['GREEN']}Connected to MQTT broker with result code {rc}{COLORS['RESET']}")
    # Subscribe to all tag topics
    client.subscribe("tags/#")
    print(f"{COLORS['CYAN']}Monitoring all tag changes...{COLORS['RESET']}")
    print(f"{COLORS['CYAN']}Press Ctrl+C to stop monitoring{COLORS['RESET']}")
    print("-" * 80)

def on_message(client, userdata, msg):
    """Callback for when a message is received from the MQTT broker."""
    try:
        # Convert the topic to a tag path
        topic = msg.topic
        if topic == "tags/database":
            # Skip the database topic
            return
        
        # Convert MQTT topic to tag path
        tag_path = topic.replace("tags/", "").replace(".", "/")
        
        # Parse the payload as JSON
        payload = json.loads(msg.payload.decode())
        
        # Get the current value
        current_value = payload.get("value", "")
        
        # Get the timestamp
        timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        
        # Check if this is a new tag or an update
        if tag_path in last_values:
            old_value = last_values[tag_path]
            if old_value != current_value:
                # This is an update
                print(f"[{timestamp}] {COLORS['YELLOW']}UPDATED{COLORS['RESET']} {tag_path}: {COLORS['RED']}{old_value}{COLORS['RESET']} -> {COLORS['GREEN']}{current_value}{COLORS['RESET']}")
        else:
            # This is a new tag
            print(f"[{timestamp}] {COLORS['BLUE']}NEW{COLORS['RESET']} {tag_path}: {COLORS['GREEN']}{current_value}{COLORS['RESET']}")
        
        # Update the last known value
        last_values[tag_path] = current_value
        
    except Exception as e:
        print(f"{COLORS['RED']}Error processing message: {e}{COLORS['RESET']}")

def main():
    """Main function."""
    # Parse command line arguments
    parser = argparse.ArgumentParser(description="Monitor tag changes in the UNS system")
    parser.add_argument("--host", default="hivemq", help="MQTT broker host")
    parser.add_argument("--port", type=int, default=1883, help="MQTT broker port")
    args = parser.parse_args()
    
    # Create MQTT client
    client = mqtt.Client()
    client.on_connect = on_connect
    client.on_message = on_message
    
    # Connect to MQTT broker
    print(f"{COLORS['CYAN']}Connecting to MQTT broker at {args.host}:{args.port}...{COLORS['RESET']}")
    client.connect(args.host, args.port, 60)
    
    # Start the MQTT loop
    try:
        client.loop_forever()
    except KeyboardInterrupt:
        print(f"\n{COLORS['YELLOW']}Monitoring stopped by user{COLORS['RESET']}")
    finally:
        client.disconnect()

if __name__ == "__main__":
    main()
