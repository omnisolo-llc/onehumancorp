<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# SPIFFE Identity Onboarding Walkthrough

Welcome to the SPIFFE Identity Onboarding walkthrough. This guide demonstrates how OHC secures inter-agent communication using SPIFFE/SPIRE for mTLS identity validation.

## 1. Zero-Trust Architecture

Every agent in the Swarm receives a cryptographic identity. This ensures that a delegating agent (e.g., Engineering Director) can explicitly verify the identity of a worker agent (e.g., QA Tester) before sharing sensitive `.agent-task/memory` contexts.

```mermaid
sequenceDiagram
    participant Swarm as Orchestration Hub
    participant SPIRE as SPIRE Server
    participant Agent as Worker Agent
    participant Mesh as Teammate Mesh

    Swarm->>SPIRE: Request SVID for new Worker
    SPIRE-->>Swarm: Issue x509-SVID
    Swarm->>Agent: Inject SVID via Volume Mount
    Agent->>Mesh: Connect via mTLS using SVID
    Mesh-->>Agent: Connection Established
```

## 2. Validation Flow
Once onboarded, all Teammate Mesh events are cryptographically signed. If an agent attempts to publish to `mesh:tasks` without a valid SVID, the Centrifuge proxy will reject the request, preserving the integrity of the Hybrid Architecture.

</div>
