<div style="font-family: 'Outfit', 'Inter', sans-serif; padding: 2rem;">

# Interactive Edge LLM Offloading Protocol API Playbook

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem; box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.37);">
  <h2 style="margin-top: 0;">Welcome to the Edge LLM Offloading Protocol</h2>
  <p>The OHC Hybrid Agentic OS uses the Edge LLM Offloading Protocol to intelligently transfer context and inference tasks from lightweight standalone desktops to heavy-duty cloud orchestration.</p>
</div>

## Offloading Endpoint

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem;">
  <p>Standalone clients initiate offloading by hitting the cloud gateway API via mutually authenticated TLS.</p>

  <h3 style="color: #66ccff;">POST /api/v1/mcp/llm/offload</h3>
  <p><strong>Authentication:</strong> Bearer Token (`Authorization: Bearer <YOUR_JWT_TOKEN>`)</p>

  <h4>Request Payload Example</h4>
  <pre style="background: rgba(0,0,0,0.5); padding: 1rem; border-radius: 8px; color: #e6e6e6;"><code>{
  "task_id": "req-987abc",
  "local_context_size_bytes": 1048576,
  "model_preference": "kairos-heavy-v2",
  "payload": {
    "prompt": "Synthesize Q3 financial reports with current user session data...",
    "embedded_context": "base64-encoded-local-sqlite-rag-state"
  }
}
</code></pre>

  <h4>Response Example</h4>
  <pre style="background: rgba(0,0,0,0.5); padding: 1rem; border-radius: 8px; color: #e6e6e6;"><code>{
  "status": "accepted",
  "cloud_job_id": "job-cloud-456",
  "estimated_completion_ms": 1200,
  "streaming_endpoint": "wss://gateway.onehumancorp.com/ws/llm/stream/job-cloud-456"
}
</code></pre>
</div>

## Connection Instructions

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem;">
  <ol style="line-height: 1.6;">
    <li>Retrieve the target Cloud Gateway URI from your SPIFFE/SPIRE identity bundle.</li>
    <li>Construct the offloading payload, ensuring sensitive local RAG state is encrypted and correctly encoded.</li>
    <li>Execute the POST request. If accepted, connect to the returned <code>streaming_endpoint</code> via WebSockets to receive real-time inference tokens.</li>
    <li>Fallback to local execution if the gateway returns a <code>503 Service Unavailable</code> due to cloud swarm saturation.</li>
  </ol>
</div>

</div>
