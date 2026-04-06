import json

with open("deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json", "r") as f:
    dashboard = json.load(f)

for panel in dashboard.get("panels", []):
    if panel.get("title") == "Token Burn Rate Forecast":
        panel["transparent"] = True
        panel["fieldConfig"] = {
            "defaults": {
                "custom": {
                    "fillOpacity": 10,
                    "gradientMode": "opacity",
                    "lineWidth": 1
                }
            },
            "overrides": []
        }
        panel["options"] = {
            "legend": {
                "displayMode": "list",
                "placement": "bottom"
            },
            "tooltip": {
                "mode": "single"
            }
        }
        panel["description"] = "Forecasted token burn rate per organization using OHC Glassmorphism styling (backdrop-filter: blur(20px))."

with open("deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json", "w") as f:
    json.dump(dashboard, f, indent=2)
