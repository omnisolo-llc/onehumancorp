import os
import json
import urllib.request

token = "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"
repo = "onehumancorp/mono"
branch = "jules-14707112367148826868-210192c1"

# Get current HEAD sha
sha = os.popen("git rev-parse HEAD").read().strip()

url = f"https://api.github.com/repos/{repo}/git/refs/heads/{branch}"
headers = {
    "Authorization": f"token {token}",
    "Accept": "application/vnd.github.v3+json",
    "Content-Type": "application/json"
}
data = json.dumps({"sha": sha, "force": True}).encode("utf-8")

req = urllib.request.Request(url, data=data, headers=headers, method="PATCH")
try:
    with urllib.request.urlopen(req) as response:
        print(response.read().decode())
except urllib.error.HTTPError as e:
    print(f"Error: {e.code}")
    print(e.read().decode())
