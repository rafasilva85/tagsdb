#!/bin/bash

# Script to run the UNS system with docker-compose
# Usage: ./run_uns_system.sh

echo "Starting UNS system with docker-compose..."
echo "-----------------------------------"

# Check if docker-compose.yml exists
if [ ! -f "docker-compose.yml" ]; then
    echo "Error: docker-compose.yml not found"
    exit 1
fi

# Start the docker-compose setup
docker-compose up -d

echo "-----------------------------------"
echo "UNS system started successfully."
echo "To stop the system, run: docker-compose down"
echo "To view logs, run: docker-compose logs -f"
echo ""
echo "Available tools:"
echo "1. Extract available tags: ./extract_tags.sh"
echo "2. Monitor a specific tag: ./monitor_tag.sh <tag_path>"
echo "3. Monitor all tag changes: ./monitor_tag_changes.sh"
echo "4. Update a tag: ./update_tag.sh <tag_path> <value>"
