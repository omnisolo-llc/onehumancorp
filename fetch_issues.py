import urllib.request
import json

req = urllib.request.Request("https://api.github.com/repos/onehumancorp/mono/issues")
req.add_header("Authorization", "token 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")
req.add_header("Accept", "application/vnd.github.v3+json")

try:
    with urllib.request.urlopen(req) as response:
        issues = json.loads(response.read())
        for issue in issues:
            print(f"#{issue['number']}: {issue['title']}")
except Exception as e:
    print(f"Error: {e}")
