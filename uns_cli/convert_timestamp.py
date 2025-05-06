import json
import datetime

with open("tags.json", "r") as f:
    data = json.load(f)

for tag in data["tags"].values():
    tag["timestamp"] = int(datetime.datetime.fromisoformat(tag["timestamp"].replace("Z", "+00:00")).timestamp())

with open("tags.json", "w") as f:
    json.dump(data, f, indent=2)
