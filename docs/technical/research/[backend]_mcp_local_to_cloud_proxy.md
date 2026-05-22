# Research Brief: Local-to-Cloud MCP Proxying

**Title**: [backend] Implement Local-to-Cloud MCP Proxy Reverse-Tunnels
**Priority**: P1
**Estimated Scope**: Large

*Design Note: This document must be rendered using OHC's Premium Aesthetic standards, including glassmorphism tokens, a 20px blur, and Outfit/Inter typography.*

## Problem Statement
Current market leaders and the OHC Hybrid OS currently struggle with a seamless degradation path across pure-cloud and pure-local modalities. A key missing capability is allowing cloud-based swarm agents to securely utilize Model Context Protocol (MCP) tools that are running on a user's Standalone Desktop. Without this, cloud agents cannot leverage secure local filesystem access or local stateful sandboxed execution, limiting their autonomy and problem-solving surface area when interacting with hybrid environments.

## Research Report
Based on the `docs/research/hybrid-agentic-os-audit.md` market audit, OHC's unique Hybrid Architecture (Cloud-Native + Standalone Desktop + Thin Client) provides a strategic advantage over competitors like Claude Code and OpenClaw. The audit identifies an emerging trend synthesis: "OHC's Universal MCP Mesh must be extended to support local-to-cloud proxying, allowing a cloud agent to securely utilize an MCP tool running on the user's Standalone Desktop via reverse-tunnels." This unlocks "Zero-Latency Local-Private RAG with Cloud Sync" and "Elastic Swarm Bursting". This capability integrates with the OHC-SIP (Swarm Intelligence Protocol) and SPIFFE/SPIRE zero-trust native identity.

## Design Doc
We need to introduce a Local Proxy that acts as a bridge.

### Architecture Comparison
```mermaid
graph TD
    subgraph OHC Future State
        D[OHC Cloud Orchestrator] <-->|gRPC/WebSocket| E[OHC Local Proxy]
        E --> F[Local Sandboxed Terminal]
        E --> G[Local SQLite Sync]
        G <-->|PowerSync| H[OHC Cloud Postgres/VectorDB]
    end

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class D,E,F,G,H premium;
```

1.  **Local Proxy Component**: A new component running in the Standalone Desktop (Tauri host/embedded Rust server) that initiates a secure, reverse-tunneled gRPC or WebSocket connection to the OHC Cloud Orchestrator.
2.  **Authentication**: This connection MUST use SPIFFE/SPIRE zero-trust identity, exchanging short-lived JWTs/SVIDs to authenticate the Standalone Desktop instance to the Cloud Orchestrator.
3.  **MCP Integration**: The Local Proxy will expose the local MCP server's JSON-RPC interface over this reverse tunnel. The Cloud Orchestrator's MCP Switchboard will register this reverse-tunneled connection as a dynamic tool provider for the authenticated user's session.
4.  **Security Boundaries**: The Local Proxy must strictly enforce read/write policies using the existing `bwrap` integration and intercept local network requests.

## Implementation Prompt
**Task**: Implement the Local Proxy for the OHC Hybrid Agentic OS.
1.  **Backend Implementation**: Create a new service in `src/server/agents/mcp/proxy` in Rust.
2.  **Reverse Tunnel Server**: Implement the server-side endpoint in the Cloud Orchestrator that accepts incoming gRPC/WebSocket connections from the local proxy. Ensure strict SPIFFE ID validation based on `docs/research/spiffe_mcp_langgraph_sync.md`.
3.  **Local Proxy Client**: Implement the client logic that runs in the Standalone Mode (embedded Rust server). It should connect to the Cloud Orchestrator and multiplex the local MCP JSON-RPC traffic.
4.  **Integration**: Update the MCP Switchboard to dynamically route requests to the reverse-tunneled MCP server when the cloud agent requests a locally-available tool.
5.  **Metrics**: Add OpenTelemetry spans and Prometheus metrics (e.g., `ohc_mcp_proxy_connections_active`, `ohc_mcp_proxy_bytes_transferred`).
6.  **Testing**: Write comprehensive unit tests in Rust ensuring 100% test coverage. Add an E2E test verifying a cloud agent can successfully call a mock tool registered via the local proxy.
