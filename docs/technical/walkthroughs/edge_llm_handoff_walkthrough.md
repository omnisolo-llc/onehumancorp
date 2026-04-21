<div style="font-family: 'Outfit', 'Inter', sans-serif; padding: 2rem;">

# Edge LLM Handoff Visual Walkthrough

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem; box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.37);">
  <h2 style="margin-top: 0;">Context Transfer Flow</h2>
  <p>This document illustrates the precise mechanism by which the Standalone Desktop transfers its localized RAG context to the Cloud Orchestration Swarm for heavy LLM inference.</p>
</div>

## Handoff Sequence

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem;">
  <p>The sequence diagram below shows the robust offloading protocol, emphasizing fallback capabilities if the cloud is unavailable.</p>

  <div style="background: rgba(0,0,0,0.3); padding: 1rem; border-radius: 8px; margin-top: 1rem;">
    ```mermaid
    sequenceDiagram
        autonumber
        participant L as Local Desktop (SQLite)
        participant G as Cloud Gateway (API)
        participant O as Cloud Orchestrator (PostgreSQL)
        participant M as Cloud LLM Swarm

        L->>L: User requests heavy inference
        L->>L: Package local RAG context
        L->>G: POST /api/mcp/llm/offload

        alt Cloud is saturated
            G-->>L: 503 Service Unavailable
            L->>L: Fallback to local small model
        else Cloud is available
            G->>O: Register Offload Job
            O-->>G: Job ID created
            G-->>L: 202 Accepted (streaming endpoint)
            O->>M: Dispatch context and prompt

            L->>G: Connect WebSocket
            M-->>G: Stream inference tokens
            G-->>L: Stream tokens to UI
        end
    ```
  </div>
</div>

</div>
