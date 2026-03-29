# OHC Agentic OS - UX Friction Report

**Author:** Principal UX Strategist & "Chief Dogfooding Officer" (L8)
**Date:** 2026-03-29
**Task:** Build the "AI News Collector" product utilizing OHC stack and evaluate the User Journey.

## Executive Summary
This report analyzes the out-of-the-box user experience and architectural friction encountered while setting up the AI News Collector. During the evaluation, several critical operational blockers were identified, negatively impacting the seamless operation of the agent swarm. Below are the key findings, with corresponding visual evidence of the successfully operating environment.

## 1. Friction Point: Provider Misalignment
- **Description**: The OpenClaw platform orchestration (via `OpenClawProvider`) did not have the necessary definitions to map `SOFTWARE_ENGINEER` agents to execution profiles.
- **Impact**: Any attempt by the `ProductManager` or the orchestrator to hire or spawn a `SOFTWARE_ENGINEER` agent resulted in failure or hallucination loops as the LLM orchestrator rejected the non-existent role mapping.
- **Resolution**: Added `domain.RoleSoftwareEngineer` to the `OpenClawProvider` capabilities within `srcs/agents/provider.go`.

## 2. Friction Point: Missing Backend Origin Routing (CORS/Proxy Routing)
- **Description**: Out-of-the-box, the Flutter web app defaulted to a generic `http://localhost:18789` or similar base URL, failing to natively bind to the backend's `http://localhost:8080/api` endpoints when hosted independently. Standard proxy-pass routing was necessary.
- **Impact**: Authentication attempts and initial telemetry fetching failed via `XMLHttpRequest` CORS errors until the application was compiled with `--dart-define=BACKEND_URL=http://localhost:8081` (pointing back to its Go web server which safely proxies `/api` calls to `:8080`).

## Visual Verification (Visual Proof)
Below are high-fidelity screenshots capturing the application in a running state, successfully demonstrating the Dashboard, Agent Team, and Missions:

### 1. OHC Dashboard
![OHC Dashboard](screenshots/2026-03-29/1_ohc_dashboard.png)

### 2. Agents / Team List
![Agents List](screenshots/2026-03-29/2_agents_list.png)

### 3. AI News Collector / Active Missions
![Missions](screenshots/2026-03-29/3_ai_news_collector.png)

## Recommended Actions
1. **Permanent Architecture Review**: We must ensure that all newly defined Agent Roles automatically propagate across all available orchestration providers without manual codebase adjustments.
2. **Frontend DX Enhancement**: The local development script should standardize `BACKEND_URL` mappings and handle CORS by default, or ship a default Nginx/Go multiplexer to minimize setup friction for local engineers.
