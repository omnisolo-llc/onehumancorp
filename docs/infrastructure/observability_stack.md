# OHC Infrastructure Observability Stack

## Cloud Observability (Kube-Prometheus-Stack)
The cloud deployments utilize the `kube-prometheus-stack` Helm chart. This automatically provisions:
1. **Prometheus Operator**: Manages Prometheus and Alertmanager clusters.
2. **Grafana**: Provisioned automatically.
3. **Dashboards**: Grafana dashboards are injected automatically via sidecar using the label `grafana_dashboard: "1"`.

## Standalone Observability (Prometheus Agent)
The Standalone Desktop wrapper uses a lightweight Prometheus Agent running in Docker to push metrics:
- Remote Write URL: `http://localhost:9090/api/v1/write`
- Local Targets:
  - `localhost:8080` (Backend Server)
  - `localhost:18789` (OHC Core)
  - `localhost:9091` (Pushgateway)

## HPA and VPA Optimization
- **HPA Thresholds**: Tightened for optimal multi-tenant scaling (CPU: 60%, Memory: 70%).
- **VPA (Vertical Pod Autoscaler)**: Enabled across `backend`, `ohcCore`, `chatwoot`, and `powersync` for robust container sizing adjustments.
- **Resource Adjustments**: Memory and CPU limits have been decreased significantly to prevent idle resource hogging.
