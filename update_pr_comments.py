import urllib.request
import json

url = "https://api.github.com/repos/onehumancorp/mono/issues/5769"
headers = {
    "Authorization": "token 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",
    "Accept": "application/vnd.github.v3+json",
    "Content-Type": "application/json",
}

data = json.dumps({"state": "closed"}).encode("utf-8")
req = urllib.request.Request(url, data=data, headers=headers, method="PATCH")

try:
    with urllib.request.urlopen(req) as response:
        print("Issue closed successfully.")
except Exception as e:
    print(f"Failed to close issue: {e}")
