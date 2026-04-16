import urllib.request
import json
import os

token = "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"
url = "https://api.github.com/repos/onehumancorp/mono/issues"

data = {
    "title": "[ui] Echo: Fix AI News Collector Capitalization and Agent Status UI Friction",
    "body": """status: DONE
agent: Echo

## Problem Statement
The "AI News Collector" and other agents appeared as "Not Running" in the UI despite being correctly assigned `ACTIVE` status in the orchestration hub. Additionally, their roles were displayed as raw unformatted string enums (e.g. `AI_NEWS_COLLECTOR`) in the UI, causing visual friction that violates the OHC premium aesthetic mandates.

## Remediation Applied
1. Updated `Agent` domain model in Flutter to correctly map both `running` and `ACTIVE` status strings to the `isRunning` computed property.
2. Abstracted the `_formatRole` logic into the `Agent` model as `formattedRole` to standardize role text presentation.
3. Updated `agents_screen.dart` and `agent_hire_wizard_screen.dart` to use the standardized `formattedRole` getter, replacing raw enums.
4. Set the `AI News Collector` to `orchestration.StatusActive` in the Go backend seeder, ensuring it launches successfully on app start."""
}

req = urllib.request.Request(url, data=json.dumps(data).encode('utf-8'), headers={
    "Authorization": f"Bearer {token}",
    "Accept": "application/vnd.github.v3+json",
    "Content-Type": "application/json"
})

try:
    with urllib.request.urlopen(req) as response:
        issue = json.loads(response.read().decode())
        print(f"Created Issue #{issue['number']}")
except Exception as e:
    print(f"Error creating issue: {e}")
