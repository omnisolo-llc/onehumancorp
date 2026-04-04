import json

with open('deploy/docker/grafana/provisioning/dashboards/ohc-hybrid.json', 'r') as f:
    dashboard = json.load(f)

# Add a panel for swarm_tasks_created / shared_tasks_created if it doesn't exist
new_panel = {
  "title": "Shared Tasks Created",
  "type": "stat",
  "datasource": "Prometheus",
  "targets": [
    {
      "expr": "sum(rate(shared_tasks_created_total[5m]))",
      "legendFormat": "Tasks Created"
    }
  ],
  "gridPos": {
    "h": 8,
    "w": 12,
    "x": 0,
    "y": 16
  }
}

new_panel_mesh = {
  "title": "Mesh Broadcasts",
  "type": "stat",
  "datasource": "Prometheus",
  "targets": [
    {
      "expr": "sum(rate(mesh_messages_broadcast_total[5m]))",
      "legendFormat": "Broadcasts"
    }
  ],
  "gridPos": {
    "h": 8,
    "w": 12,
    "x": 12,
    "y": 16
  }
}

dashboard['panels'].append(new_panel)
dashboard['panels'].append(new_panel_mesh)

with open('deploy/docker/grafana/provisioning/dashboards/ohc-hybrid.json', 'w') as f:
    json.dump(dashboard, f, indent=2)
