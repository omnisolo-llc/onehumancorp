# Hybrid MCP Telemetry Visual Walkthrough

This guide outlines how to observe and correlate metrics across the OHC Hybrid Architecture using the Telemetry-MCP Bridge.

## Dashboard Aesthetic (OHC-SIP)
All telemetry dashboards strictly adhere to the OHC Premium aesthetic.

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 24px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

### Telemetry Correlation View
**Environment:** Cloud & Standalone
**Status:** Connected (mTLS verified)

| Metric | Cloud Value | Local Value | Delta |
| :--- | :--- | :--- | :--- |
| RAG Latency | 0.25s | 0.85s | +0.60s |
| Sync Queue | 0 | 14 | +14 |

</div>

## Agent Troubleshooting Flow
1. KAIROS Orchestrator detects high RAG latency in local Standalone mode.
2. It uses `telemetry.query` via MCP to pull comparative metrics.
3. The agent autonomously generates a report and queues a sync task to offload compute when internet connection is restored.
