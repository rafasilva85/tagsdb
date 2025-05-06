#!/usr/bin/env python3
"""
MQTT Monitor for UNS CLI

This script connects to HiveMQ and monitors tag changes.
It can be used to:
1. Extract all available tags
2. Monitor changes to specific tags or all tags
"""

import argparse
import json
import time
import paho.mqtt.client as mqtt

# Global variables
all_tags = {}
tag_updates = []
database_received = False

def on_connect(client, userdata, flags, rc):
    """Callback for when the client connects to the broker."""
    if rc == 0:
        print("Connected to MQTT broker")
        # Subscribe to all tag topics
        client.subscribe("tags/#")
        print("Subscribed to tags/#")
    else:
        print(f"Failed to connect to MQTT broker with code {rc}")

def on_message(client, userdata, msg):
    """Callback for when a message is received from the broker."""
    global all_tags, tag_updates, database_received
    
    topic = msg.topic
    payload = msg.payload.decode('utf-8')
    
    try:
        data = json.loads(payload)
        
        # Handle database updates
        if topic == "tags/database":
            database_received = True
            if "tags" in data:
                all_tags = data["tags"]
                print(f"Received full tag database with {len(all_tags)} tags")
        
        # Handle individual tag updates
        elif topic.startswith("tags/"):
            tag_path = topic.replace("tags/", "").replace(".", "/")
            tag_updates.append({
                "timestamp": time.strftime("%Y-%m-%d %H:%M:%S"),
                "path": tag_path,
                "value": data.get("value", "N/A")
            })
            print(f"Tag update: {tag_path} = {data.get('value', 'N/A')}")
    
    except json.JSONDecodeError:
        print(f"Error decoding JSON from topic {topic}: {payload}")

def list_tags():
    """Print all available tags."""
    if not all_tags:
        print("No tags available yet. Waiting for database update...")
        return
    
    print("\n=== Available Tags ===")
    for path, tag in all_tags.items():
        print(f"Path: {path}")
        print(f"  Name: {tag.get('name', 'N/A')}")
        print(f"  Description: {tag.get('description', 'N/A')}")
        print(f"  Value: {tag.get('value', 'N/A')}")
        print()

def show_updates():
    """Print recent tag updates."""
    if not tag_updates:
        print("No tag updates received yet.")
        return
    
    print("\n=== Recent Tag Updates ===")
    for update in tag_updates[-10:]:  # Show last 10 updates
        print(f"[{update['timestamp']}] {update['path']} = {update['value']}")

def main():
    parser = argparse.ArgumentParser(description="MQTT Monitor for UNS CLI")
    parser.add_argument("--host", default="localhost", help="MQTT broker host")
    parser.add_argument("--port", type=int, default=1883, help="MQTT broker port")
    parser.add_argument("--interval", type=int, default=5, help="Interval in seconds to display updates")
    args = parser.parse_args()
    
    # Set up MQTT client
    client = mqtt.Client()
    client.on_connect = on_connect
    client.on_message = on_message
    
    # Connect to the broker
    try:
        client.connect(args.host, args.port, 60)
    except Exception as e:
        print(f"Error connecting to MQTT broker: {e}")
        return
    
    # Start the network loop in a background thread
    client.loop_start()
    
    print(f"Monitoring MQTT broker at {args.host}:{args.port}")
    print("Press Ctrl+C to exit")
    
    try:
        while True:
            # Wait for the database to be received
            if not database_received:
                print("Waiting for tag database...")
                time.sleep(2)
                continue
            
            # Display available tags and recent updates
            list_tags()
            show_updates()
            
            # Wait for the specified interval
            print(f"\nWaiting {args.interval} seconds for updates...")
            time.sleep(args.interval)
    
    except KeyboardInterrupt:
        print("\nExiting...")
    
    finally:
        # Clean up
        client.loop_stop()
        client.disconnect()

if __name__ == "__main__":
    main()
