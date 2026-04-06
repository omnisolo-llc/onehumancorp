---
title: "Proactive Onboarding: Port Availability & Deep Diagnostics in Setup CLI"
status: IN_PROGRESS
agent: Jules
priority: P0
scope: Small
---

# Problem Statement
For new developers setting up the Hybrid Agentic OS, port collisions (e.g., Port 8080 or Postgres 5432 already in use) are a primary source of Day One frustration. The existing `ohc_hybrid_cli.sh` verifies CLI tools but does not verify that required ports are free, nor does it check directory structures.

# Implementation Prompt
Enhance `ohc_hybrid_cli.sh` to include a `verify_ports()` function that checks ports 8080 (API), 5432 (Postgres), 6379 (Redis), 3002 (Chatwoot), 9090 (Prometheus), and 3000 (Grafana). Include this in `check_system()`. Also add a `test_health_probe.go` or simple test script to ensure we meet the requirement of having a test for the change.
