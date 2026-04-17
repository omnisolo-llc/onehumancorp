<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Edge LLM Offloading Protocol: Visual Walkthrough

This guide illustrates the end-to-end handoff flow between the local Standalone instance and the K8s Cloud Pod when dynamically offloading heavy LLM inference requests.

## Architecture Flow

The system evaluates token counts, local resources, and privacy flags (`is_sensitive`) before execution. The `mcp_inference_router` acts as the traffic controller.

```mermaid
sequenceDiagram
    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%),font-family:\'Outfit\'\, \'Inter\'\, sans-serif;

    participant Standalone as Local Standalone
    participant Router as MCP Inference Router
    participant Cloud as K8s Cloud Pod

    Standalone->>Router: POST /api/v1/inference/route (is_sensitive=false, tokens=5000)

    activate Router
    Router->>Router: Evaluate Token Count
    Router->>Router: Check Resource Utilization

    alt is_sensitive == true OR resources == OK
        Router->>Standalone: Execute Inference Locally
    else heavy load OR heavy prompt
        Router->>Cloud: Offload Request via Edge Protocol
        activate Cloud
        Cloud-->>Router: Cloud-Assisted Inference Response
        deactivate Cloud
    end

    Router-->>Standalone: Return Routed Completion
    deactivate Router

```

## Desktop UI Indicators

The Desktop mode features an elegant UI indicator showing the current execution mode:

- **Inference: Local** - Used when `is_sensitive: true` or the local machine handles the request comfortably.
- **Inference: Cloud Assisted** - Used when the task is offloaded to the K8s backend for rapid execution.

</div>
