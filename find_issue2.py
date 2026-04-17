import os
import requests

GITHUB_TOKEN = os.environ.get("GITHUB_TOKEN", "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")
headers = {"Authorization": f"token {GITHUB_TOKEN}", "Accept": "application/vnd.github.v3+json"}
response = requests.get("https://api.github.com/repos/onehumancorp/mono/issues", headers=headers)
for issue in response.json():
    if "Agent Configuration Wizards" in issue["title"] or "Business Setup" in issue["title"] or "Wizard" in issue["title"]:
        print(issue["number"], issue["title"])
