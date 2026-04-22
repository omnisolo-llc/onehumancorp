# OHC Hybrid Architecture

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255, 255, 255, 0.1); border-radius: 12px; padding: 24px; margin-bottom: 32px; font-family: 'Outfit', 'Inter', sans-serif;">
  <h2>The "Hybrid Agentic OS" Mandate</h2>
  <p>The OneHumanCorp (OHC) platform is designed as a seamlessly unified <b>Hybrid Agentic OS</b>. It uniquely bridges the power of Cloud-Native multi-tenant availability with the resilience of Edge-first, zero-infrastructure deployment.</p>
</div>

The architecture supports two primary operational modes:

1. **Cloud-Native Mode:** The default operational mode for multi-tenant SaaS environments, powered by Kubernetes, PostgreSQL, and Redis.
2. **Standalone Desktop Mode:** A zero-infrastructure fallback designed for local autonomy, leveraging SQLite and the Teammate Mesh.

## Architecture Overview

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255, 255, 255, 0.1); border-radius: 12px; padding: 24px; margin-bottom: 32px;">
```mermaid
flowchart TD
    subgraph "Cloud-Native Mode (HA)"
        A[API Gateway / gRPC] --> B(Go Backend Pods)
        B --> C[(PostgreSQL with RLS)]
        B --> D[Redis Redlock / Queue]
    end

    subgraph "Standalone Desktop Mode (Edge)"
        E[Local API Server] --> F(Local Go Binary)
        F --> G[(SQLite Local State)]
        F --> H[Memory Queue / Lock]
    end

    subgraph "Sync Engines"
        I(Teammate Mesh) -.->|P2P Sync| F
        J(AutoDream Pipeline) -.->|Episodic Memory| C
    end

    G --> J
    F <--> I
```
</div>

## Standalone Desktop Mode

When operating in environments with intermittent connectivity or where strict local autonomy is required, the system gracefully degrades to **Standalone Desktop Mode**.

### Key Components

- **SQLite Fallback**: In the absence of a managed PostgreSQL instance, the application utilizes SQLite as a robust local data store, maintaining schema compatibility where possible (e.g., using `VARCHAR` for UUIDs).
- **In-Memory Coordinators**: Replaces Redis-backed Distributed Locks and Queues with fast, local in-memory alternatives, preserving agent functionality without external dependencies.

## Teammate Mesh

The **Teammate Mesh** provides the essential communication fabric for distributed agents operating at the Edge.

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255, 255, 255, 0.1); border-radius: 12px; padding: 24px; margin-bottom: 32px;">
```mermaid
sequenceDiagram
    participant AgentA as Operations Agent (Local)
    participant Mesh as Teammate Mesh Broker
    participant AgentB as Advisory Agent (Peer)

    AgentA->>Mesh: Publish Event (Order Processed)
    Mesh-->>AgentA: Ack
    Mesh->>AgentB: Broadcast (Order Event)
    AgentB->>AgentB: Update Local Context
```
</div>

- **Peer-to-Peer Synchronization**: Enables real-time event distribution and state synchronization across multiple local agents without routing through the central cloud.
- **Resilient Delivery**: Ensures messages are delivered even in unstable network conditions.

## AutoDream Data Pipeline

The **AutoDream Data Pipeline** is responsible for consolidating edge telemetry and episodic memory back into long-term cloud storage.

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255, 255, 255, 0.1); border-radius: 12px; padding: 24px; margin-bottom: 32px;">
```mermaid
flowchart LR
    A[(Edge SQLite)] -->|Extract Episodic Logs| B(AutoDream Worker)
    B -->|Generate Embeddings| C(Vector DB / pgvector)
    C -->|RAG Retrieval| D(Cloud AI Agents)
```
</div>

- **Episodic Memory**: Collects completed tasks, session logs, and agent interactions stored locally in SQLite.
- **Consolidation**: Asynchronously transmits and translates episodic data into vector embeddings within the Cloud's `autodream_memories` table (using PostgreSQL's `pgvector` with `hnsw` indexes).
- **RAG Enablement**: Empowers cloud-based AI agents with deep contextual awareness of edge operations.

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255, 255, 255, 0.1); border-radius: 12px; padding: 24px; margin-top: 32px; font-family: 'Outfit', 'Inter', sans-serif;">
  <p style="text-align: center; color: #a0aec0;"><i>The Hybrid Architecture ensures OHC remains resilient, scalable, and ever-present, regardless of the deployment environment.</i></p>
</div>
