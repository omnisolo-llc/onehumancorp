# OHC Hybrid Agentic OS: KAIROS Shared Task List Design Doc

## 1. Executive Summary
This design document details the architecture for the "Shared Task List" and underlying agent coordination systems in the One Human Corp (OHC) Hybrid Agentic OS. As part of KAIROS Orchestration, the system provides real-time decomposition, claiming, and synchronization of swarm tasks using a hybrid approach suitable for both Cloud-Native and Standalone modes.

## 2. Competitive Context & Market Strategy
Current systems like Claude Code and Replit Agent operate primarily as single-agent conversational loops or stateless tools. OHC's "Unfair Advantage" is **Swarm Intelligence via Shared State**. By enabling agents to dynamically discover, bid on, and claim granular tasks through a Teammate Mesh, OHC transforms parallel task execution from an orchestrated script into a resilient, autonomous swarm.

## 3. Core Architecture
The Shared Task List relies on three primary pillars of the KAIROS Orchestration engine:
1. **Teammate Mesh (Redis Pub/Sub)**: For dynamic task broadcasting and bidding.
2. **AutoDream Memory Consolidation**: To process historical tasks into embeddings, improving future task decomposition.
3. **Hybrid MCP State Sync**: To ensure that tools utilized by the swarm share consistent state, whether running locally (SQLite) or in the cloud (PostgreSQL).

### 3.1 Teammate Mesh Task Claiming
When an Orchestrator agent decomposes an epic into sub-tasks, these tasks are written to the database (e.g., `shared_tasks`). To eliminate polling bottlenecks:
- **Event Broadcast**: The `DynamicTaskRouter` broadcasts a `task.available` event over Redis.
- **Agent Bidding**: Available Implementer agents receive the broadcast, evaluate their capabilities against the required skills, and respond with a `task.claim` event over Redis.
- **Lock & Assignment**: The `DynamicTaskRouter` processes claims on a first-come, highest-capability basis, locking the specific row in `shared_tasks` (`SELECT FOR UPDATE`) to prevent concurrent assignment.

### 3.2 AutoDream Background Processing
Agents generate massive amounts of unstructured episodic memories during task execution.
- **Daemon Worker**: The `AutoDreamConsolidator` daemon wakes up during idle CPU cycles.
- **Batch Processing**: It claims batches (e.g., 100) of unprocessed memories using distributed Redis locks.
- **Embedding Storage**: It generates vector embeddings and stores them in the OHC Central Database (pgvector/Pinecone) for long-term semantic retrieval.

### 3.3 Hybrid MCP Tool State Synchronization
The swarm frequently utilizes external tools via the Model Context Protocol (MCP).
- **Cloud-Native Mode**: Tool state is written directly to the high-concurrency PostgreSQL cluster.
- **Standalone Mode**: Tool state is written to a local SQLite database.
- **Sync Bridge**: The `HybridMCPStateBridge` ensures that when a Standalone user reconnects to the OHC Cloud, local tool execution states (e.g., cached auth tokens, file indices) are synchronized securely using `lib/sync`.

## 4. Database Schema Changes
- **`mcp_tool_state` (New Table)**: `tool_id`, `key`, `value`, `updated_at`.
- **`shared_tasks` (Modified Table)**: Add `claimed_by` and `claim_status` with an index on status.
- **`autodream_memories` (Index Addition)**: Add index on `processed_at` to support efficient batch polling.

## 5. Security & Isolation
- **Tenant Isolation**: In Cloud-Native mode (`OHC_MULTITENANT=true`), all shared task lists and agent communication channels are strictly scoped by `organization_id` via the `TenantRegistry`.
- **Identity**: System relies exclusively on SPIFFE/SPIRE for agent-to-agent and agent-to-database authentication.

## 6. Visual Excellence Guidelines (UI Integration)
The Shared Task List UI must adhere to the OHC Premium Feel:
- Background: `rgba(255, 255, 255, 0.03)` with `backdrop-filter: blur(20px) saturate(200%)`.
- Typography: Use 'Outfit' for headers and 'Inter' for task details.
- Interactions: Real-time updates via WebSockets when tasks shift from "Available" to "Claimed".
