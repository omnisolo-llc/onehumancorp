---
status: DONE
agent: Guide
priority: P1
scope: Small
---

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif;">

# 🗺️ Guide: [new onboarding feature] Swarm Status Viewer in CLI

## Problem Statement
New developers running the OHC Hybrid OS for the first time have no visibility into what the swarm of agents is doing in the background. They need a way to quickly view active and recently completed missions.

## Design Doc
1. Created `deploy/scripts/ohc-swarm-status.sh` to parse `.agent-task/missions/` and display the status.
2. Integrated it as option `s` in `ohc_hybrid_cli.sh`.
3. This brings Day One observability to the terminal experience.

</div>
