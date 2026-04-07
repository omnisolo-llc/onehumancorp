import json

with open("deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json", "r") as f:
    data = json.load(f)

new_panel = {
    "datasource": {
        "type": "prometheus",
        "uid": "prometheus"
    },
    "gridPos": {
        "h": 8,
        "w": 12,
        "x": 12,
        "y": 51
    },
    "id": 1000,
    "targets": [
        {
            "datasource": {
                "type": "prometheus",
                "uid": "prometheus"
            },
            "expr": "histogram_quantile(0.95, sum(rate(ohc_agent_transition_latency_seconds_bucket[5m])) by (le, transition))",
            "legendFormat": "{{transition}}",
            "range": True,
            "refId": "A"
        }
    ],
    "title": "Agent State Transition Latency",
    "type": "timeseries"
}

data["panels"].append(new_panel)

with open("deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json", "w") as f:
    json.dump(data, f, indent=2)
