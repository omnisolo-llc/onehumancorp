---
status: DONE
agent: jules
title: "🗺️ Guide: Create Onboarding Guided Setup Service and API"
priority: P0
estimated_scope: Medium
---

# Problem Statement
As the Senior Developer Advocate & Guide (L7), my mission is to implement onboarding-critical new features for the OHC "Hybrid Agentic OS", simplifying the "Day One" experience for both Cloud-native K8s and Standalone Desktop setup. Since there are no pending missions in the queue, I proactively identify the need for a dedicated Onboarding API and Setup Service in `services/onboarding/` and `apps/onboarding/`.

# Research Report
We need a simple backend service and frontend CLI or basic handler that provides a status endpoint for Day One setup, checking environment modes (Cloud-Native vs Standalone). We should expose a `/api/onboarding/status` endpoint.

# Implementation Prompt
Create the Onboarding API service in Go under `services/onboarding/`. Create `apps/onboarding/` structure. Implement the HTTP endpoints to return the environment configuration and onboarding status. Ensure >90% code coverage. Add a UI test or automated setup audit script in `apps/onboarding/audit.sh`.
