<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Edge LLM Offloading Protocol

This interactive walkthrough demonstrates the Edge LLM Offloading Protocol, a critical feature of the OHC Hybrid Architecture that dynamically routes LLM inference requests between the local edge device and the cloud based on task complexity and privacy requirements.

## Protocol Flow

The following Mermaid sequence diagram illustrates the decision-making process for edge LLM offloading:

```mermaid
sequenceDiagram
    participant App as Standalone Client
    participant Router as MCP Inference Router
    participant Edge as Local SQLite Cache / LLM
    participant Cloud as Cloud K8s Pod (pgvector)

    App->>Router: POST /api/v1/inference/route (Payload)
    Router->>Router: Evaluate Prompt Size & Privacy Flags

    alt is_sensitive == true OR low_complexity
        Router->>Edge: Route to Local Inference
        Edge-->>Router: Local Response
    else is_sensitive == false AND high_complexity
        Router->>Router: Check Cloud Load & Auth
        alt Auth Valid & Load OK
            Router->>Cloud: Securely Forward to Cloud Swarm
            Cloud-->>Router: Cloud Assisted Response
        else Network Failed / Rate Limited
            Router->>Edge: Fallback to Local Inference
            Edge-->>Router: Local Response (Degraded)
        end
    end

    Router-->>App: Return Final Inference Response
```

## Setup & Configuration

To enable the Edge LLM Offloading Protocol, you must configure the `mcp_inference_router` tool in your Standalone Desktop Client.

1.  Ensure you are authenticated via SPIFFE/SPIRE.
2.  Set the environment variable `OHC_EDGE_OFFLOADING_ENABLED=true`.

For a full breakdown of the API endpoints, see the [Hybrid MCP Integration API Playbook](../../walkthroughs/hybrid_mcp_rag_protocol.md).

</div>
