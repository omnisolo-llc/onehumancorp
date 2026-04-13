---
status: DONE
agent: Jules
priority: P0
scope: Large
---
<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# 🗺️ Guide: Implement KAIROS Swarm Analytics Dashboard UI

## Problem Statement
The OHC Hybrid Agentic OS requires a centralized "Swarm Analytics Dashboard" to visually represent the realtime activity of the KAIROS Orchestrator. Currently, the backend Shared Task List, Teammate Mesh, and AutoDream pipelines are functional, but there is no premium UI to visualize this data. This lack of observability hinders the user's ability to orchestrate the swarm effectively.

## Research Report
- The orchestration backend exposes metrics and states via Redis Pub/Sub (Cloud) and In-Memory channels (Standalone).
- The frontend must connect via WebSockets to stream these updates in realtime.
- Competitors lack deep, real-time visual orchestration of their agents. OHC will provide a "Command Center" feel.
- The UI MUST adhere to the OHC Stylistic Intent Profile (Glassmorphism, 20px blur, Outfit font).

## Design Doc
1. **Frontend Component (`srcs/app/lib/screens/kairos_dashboard.dart`):**
   - A new Flutter screen utilizing the existing `GlassCard` widget (`srcs/app/lib/widgets/glass_card.dart`).
   - Three main panels:
     - **Shared Task Queue:** Visualizing pending, active, and completed tasks.
     - **Teammate Mesh Stream:** A live, scrolling log of agent communication (using `mesh:coordination` channels).
     - **AutoDream Memory:** A visualization of recently embedded vectors and consolidated knowledge.
2. **Backend API (`srcs/server/api/kairos_stream.go`):**
   - Create a WebSocket endpoint `/api/kairos/stream` that securely authenticates the user (via SPIRE/JWT) and subscribes to the relevant Teammate Mesh channels, forwarding JSON payloads to the frontend.

## Implementation Prompt
Hello Implementer agent!
1. Verify the existence of `srcs/app/lib/widgets/glass_card.dart`. If it doesn't exist, create a reusable Glassmorphism card widget applying `backdrop-filter: blur(20px)`.
2. Implement `srcs/app/lib/screens/kairos_dashboard.dart` matching the Design Doc. Ensure Typography uses Outfit/Inter.
3. In `srcs/server/api/`, create `kairos_stream.go` to provide a WebSocket stream of Teammate Mesh events.
4. Ensure the UI degrades gracefully in Standalone Mode.
5. Add Flutter widget tests and Go backend tests (>90% coverage).

</div>
