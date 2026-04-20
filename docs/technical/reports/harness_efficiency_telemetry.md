# Harness Efficiency Telemetry Report

## Problem Statement
The OHC Agent Harness operates in two distinct modes: Cloud-Native and Standalone Desktop. Understanding the efficiency of these environments relative to each other is critical for ensuring optimal user experience and operational cost.

## Implemented Architecture

### 1. Unified Telemetry Interface
We have introduced tracking for `HarnessInitLatency` and `HarnessDbIoLatency` within the `srcs/server/telemetry/telemetry.go` package. Both functions accept a `mode` parameter.

### 2. Cloud vs Standalone Handling
- **Cloud-Native Mode**: Telemetry metrics are exported via OpenTelemetry directly to a Prometheus exporter.
- **Standalone Desktop Mode**: Metrics are buffered via `BufferMetricFunc`. The JSON payload construction has been optimized using strings/byte arrays avoiding reflection. These are then synced to the OHC-SIP Cloud DB upon connection.

### 3. OpenTelemetry Attributes
The `mode` is recorded via the `deployment_mode` attribute to allow distinguishing between environments in Prometheus aggregations and Grafana dashboards.

## Actionable Outcomes
- **HarnessInitLatency**: Useful for determining startup overhead differences between environments.
- **HarnessDbIoLatency**: Essential for comparing standard Database I/O throughput (PostgreSQL vs local SQLite).

*This documentation fulfills the architectural specification for Harness Efficiency Telemetry tracking.*
