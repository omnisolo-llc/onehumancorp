import os
import requests

GITHUB_TOKEN = os.environ.get("GITHUB_TOKEN", "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")
headers = {"Authorization": f"token {GITHUB_TOKEN}", "Accept": "application/vnd.github.v3+json"}
response = requests.get("https://api.github.com/repos/onehumancorp/mono/issues?state=all", headers=headers)
for issue in response.json():
    if "Wizard" in issue["title"] or "5713" in str(issue["number"]) or "5756" in str(issue["number"]) or "Implementer" in issue["title"]:
        print(issue["number"], issue["title"])
