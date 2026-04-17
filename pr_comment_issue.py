import urllib.request
import json

url = "https://api.github.com/repos/onehumancorp/mono/issues/5769/comments"
headers = {
    "Authorization": "token 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",
    "Accept": "application/vnd.github.v3+json",
    "Content-Type": "application/json",
}

data = json.dumps({"body": "Mission accomplished. PR submitted covering MCP SDK tool registration and 100% unit tests."}).encode("utf-8")
req = urllib.request.Request(url, data=data, headers=headers, method="POST")

try:
    with urllib.request.urlopen(req) as response:
        print("Comment added successfully.")
except Exception as e:
    print(f"Failed to add comment: {e}")
