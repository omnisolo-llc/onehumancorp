<div markdown="1" style="font-family: 'Outfit', 'Inter', sans-serif; backdrop-filter: blur(20px); -webkit-backdrop-filter: blur(20px); background: rgba(255, 255, 255, 0.1); border: 1px solid rgba(255, 255, 255, 0.2); border-radius: 12px; padding: 24px;">

# MCP Inference Router Design Doc

## Architecture
An MCP Tool (`mcp_inference_router`) acts as a proxy for inference requests. It evaluates prompt size and privacy flags to decide between local (Standalone SQLite/Edge) execution and Cloud (Postgres/Pod) offloading.

## API Contract
`POST /api/v1/inference/route`
Accepts a standard completion request payload, returns the routed response.

## Security & Privacy
Requests marked `is_sensitive: true` are never offloaded.

</div>
