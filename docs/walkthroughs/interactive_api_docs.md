<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Interactive API Docs Walkthrough

Welcome to the Interactive API Documentation for the OHC Hybrid Agentic OS. This walkthrough demonstrates how to utilize our integrated Swagger/OpenAPI portal to interface with the Swarm directly.

## 1. Accessing the Portal
The interactive API documentation is exposed at `/api/docs` on both Cloud and Standalone installations. It uses real-time WebSockets to reflect live cluster state.

## 2. Authentication
Click "Authorize" and provide your SPIFFE/SPIRE x509-SVID token. In local development mode, you may use the ephemeral `OHC_DEV_TOKEN`.

## 3. Core Capabilities
- **Live Execution:** Test endpoints such as `POST /api/agents/hire` directly.
- **Real-time Monitoring:** The portal connects to the Teammate Mesh, meaning task execution updates are streamed live directly to the Swagger UI via Server-Sent Events (SSE).

</div>
