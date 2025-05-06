#!/usr/bin/env python3
"""
Monitor Tag Changes Script

This script monitors changes to a specific tag in the UNS CLI system.
It connects to the MQTT broker and subscribes to the tag's topic.
"""

import argparse
import json
import time
import paho.mqtt.client as mqtt
import sys

# Global variables
tag_updates = []
database_value = None

def on_connect(client, userdata, flags, rc):
    """Callback for when the client connects to the broker."""
    if rc == 0:
        print(f"Connected to MQTT broker")
        
        # Subscribe to the specific tag topic
        tag_topic = f"tags/{args.tag_path.replace('/', '.')}"
        client.subscribe(tag_topic)
        print(f"Subscribed to {tag_topic}")
        
        # Also subscribe to the database topic to get the initial value
        client.subscribe("tags/database")
        print("Subscribed to tags/database")
    else:
        print(f"Failed to connect to MQTT broker with code {rc}")
        sys.exit(1)

def on_message(client, userdata, msg):
    """Callback for when a message is received from the broker."""
    global tag_updates, database_value
    
    topic = msg.topic
    payload = msg.payload.decode('utf-8')
    
    try:
        data = json.loads(payload)
        
        if topic == f"tags/{args.tag_path.replace('/', '.')}":
            # Individual tag update
            timestamp = time.strftime("%Y-%m-%d %H:%M:%S")
            value = data.get("value", "N/A")
            tag_updates.append({"timestamp": timestamp, "value": value})
            print(f"[{timestamp}] {args.tag_path} = {value}")
        
        elif topic == "tags/database" and "tags" in data:
            # Full database update
            if args.tag_path in data["tags"]:
                tag = data["tags"][args.tag_path]
                database_value = tag.get("value", "N/A")
                timestamp = time.strftime("%Y-%m-%d %H:%M:%S")
                print(f"[{timestamp}] {args.tag_path} = {database_value} (from database)")
    
    except json.JSONDecodeError:
        print(f"Error decoding JSON from topic {topic}")

def main():
    """Main function."""
    # Set up MQTT client
    client = mqtt.Client()
    client.on_connect = on_connect
    client.on_message = on_message
    
    # Connect to the broker
    try:
        client.connect(args.host, args.port, 60)
    except Exception as e:
        print(f"Error connecting to MQTT broker: {e}")
        sys.exit(1)
    
    # Start the network loop
    client.loop_start()
    
    print(f"Monitoring tag: {args.tag_path}")
    print("Press Ctrl+C to exit")
    
    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        print("\nExiting...")
    finally:
        client.loop_stop()
        client.disconnect()

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Monitor Tag Changes")
    parser.add_argument("tag_path", help="Path of the tag to monitor")
    parser.add_argument("--host", default="localhost", help="MQTT broker host")
    parser.add_argument("--port", type=int, default=1883, help="MQTT broker port")
    args = parser.parse_args()
    
    main()
