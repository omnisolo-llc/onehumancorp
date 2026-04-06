import json

with open("deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json", "r") as f:
    dashboard = json.load(f)

for panel in dashboard.get("panels", []):
    if panel.get("title") == "Token Burn Rate Forecast":
        print(json.dumps(panel, indent=2))
