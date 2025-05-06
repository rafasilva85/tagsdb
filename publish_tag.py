#!/usr/bin/env python3
import json
import sys
import paho.mqtt.client as mqtt
import time

def main():
    if len(sys.argv) < 3:
        print("Usage: python3 publish_tag.py <tag_path> <value>")
        print("Example: python3 publish_tag.py US/TX/AUSTIN/AREA1/LINE1/MACHINE1/PUMP1/PRESSURE 99.9")
        sys.exit(1)
    
    tag_path = sys.argv[1]
    value = sys.argv[2]
    
    # Connect to MQTT broker
    client = mqtt.Client()
    try:
        client.connect("localhost", 1883, 60)
    except Exception as e:
        print(f"Error connecting to MQTT broker: {e}")
        sys.exit(1)
    
    # Create tag object
    tag = {
        "path": tag_path,
        "name": "Pump 1 Pressure" if "PRESSURE" in tag_path else "Pump 1 Status",
        "description": "Pressure sensor for Pump 1" if "PRESSURE" in tag_path else "Operational status of Pump 1",
        "value": value
    }
    
    # Publish to individual tag topic
    topic = f"tags/{tag_path.replace('/', '.')}"
    print(f"Publishing to topic: {topic}")
    print(f"Payload: {json.dumps(tag)}")
    client.publish(topic, json.dumps(tag), qos=1, retain=True)
    
    # Read the full database
    with open("uns_cli/tags.json", "r") as f:
        data = json.loads(f.read())
    
    # Update the tag in the database
    data["tags"][tag_path]["value"] = value
    
    # Publish the full database
    print("Publishing to topic: tags/database")
    print(f"Payload: {json.dumps(data)}")
    client.publish("tags/database", json.dumps(data), qos=1, retain=True)
    
    # Wait for messages to be delivered
    time.sleep(1)
    client.disconnect()
    
    print(f"Tag {tag_path} updated to {value}")

if __name__ == "__main__":
    main()
