import os
import requests

GITHUB_TOKEN = os.environ.get("GITHUB_TOKEN", "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")
headers = {"Authorization": f"token {GITHUB_TOKEN}", "Accept": "application/vnd.github.v3+json"}
response = requests.get("https://api.github.com/repos/onehumancorp/mono/issues", headers=headers)
for issue in response.json():
    if "Implementer: Implement KAIROS Master Orchestration DB schemas" not in issue["title"]: # Note: The task was "Implement and refine the Wizard / Onboarding domain features" but let's check the assignees and title.
        pass
    print(issue["number"], issue["title"])
