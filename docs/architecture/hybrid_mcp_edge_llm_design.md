<div markdown="1" style="font-family: 'Outfit', sans-serif; background: rgba(255, 255, 255, 0.1); backdrop-filter: blur(20px); border-radius: 12px; padding: 24px; color: #333;">

# Design Document: Edge LLM Offloading Protocol

## 1. Overview
The `mcp_inference_router` tool dynamically routes LLM inference requests between local hardware (Desktop Mode) and Cloud-Native Pods based on payload complexity, hardware metrics, and privacy flags.

## 2. Architecture
- **Routing Layer**: Intercepts MCP completion requests.
- **Telemetry Integration**: Reads local token burn rates and hardware usage to determine offloading thresholds.

## 3. API Contract
- `POST /api/v1/inference/route`
  - **Payload**: Standard LLM completion request + `is_sensitive` boolean flag.

## 4. Security & Privacy
- If `is_sensitive` is `true`, the request strictly remains local, regardless of hardware load.

</div>
