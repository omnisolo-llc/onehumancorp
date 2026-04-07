---
title: "🎨 Palette: [Hybrid UX improvement] Agent Swarm Observability Dashboard"
status: DONE
agent: Palette
priority: P1
estimated_scope: Large
---

# Problem Statement
We need a real-time UI widget connecting to the Teammate Mesh so the Human user can monitor the agent swarm working 24/7. This falls under the "Swarm Observability Dashboard" autonomous task definition.

# Design Doc
- **Swarm Observability Dashboard**: Implement the `AgentSwarmDashboard` widget in Flutter (`srcs/app/lib/widgets/agent_swarm_dashboard.dart`).
- **GlassCard Integration**: Use the existing `GlassCard` widget (which provides the OHC visual identity) for each agent item in the list.
- **Teammate Mesh Mock**: Since we don't have a backend ready for this specific websocket stream yet, create a mock Teammate Mesh data stream using `Stream.periodic` to simulate agent activities (e.g. "Drafting PR", "Running tests", "Idle", "Analyzing metrics").
- **Agent Models**: Define an `AgentStatus` model to hold agent name, current task, status enum (idle, working, blocked), and avatar/icon.
- **Micro-animations**: Implement high-fidelity, performant Flutter animations for premium tactile feedback when agents pass message states natively (e.g. subtle pulse animation on status indicators, implicit animations when task text changes).

# Implementation Details
- Create `srcs/app/lib/models/agent_status.dart`.
- Create `srcs/app/lib/widgets/agent_swarm_dashboard.dart`.
- Update a test screen or main dashboard to include this new widget.
