import os
import requests

GITHUB_TOKEN = os.environ.get("GITHUB_TOKEN", "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")
headers = {"Authorization": f"token {GITHUB_TOKEN}", "Accept": "application/vnd.github.v3+json"}
issue_url = "https://api.github.com/repos/onehumancorp/mono/issues/5713" # Wait, the issue is #5713 parent, the actual task issue number was... I need to find it from the earlier fetch
