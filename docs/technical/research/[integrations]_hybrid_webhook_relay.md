# Research Brief: Hybrid Webhook Relay for Standalone Mode

**Title**: [integrations] Implement Hybrid Webhook Relay for Local Desktop
**Priority**: P1
**Estimated Scope**: Large

*Design Note: This document must be rendered using OHC's Premium Aesthetic standards, including glassmorphism tokens, a 20px blur, and Outfit/Inter typography.*

## Problem Statement
OHC Standalone Desktop Mode (SQLite) allows users to run agents locally for data privacy and resource efficiency. However, a major limitation of this local-first architecture is the inability to directly receive external webhooks (e.g., from Stripe, GitHub, or Jira) because the local instance is not exposed to the public internet. Without a reliable way to receive these asynchronous events, local agents cannot react to critical external state changes, breaking the "Full-Spectrum Observability" and "Absolute Autonomy" values of the Hybrid OS.

## Research Report
Competitors like Claude Code and Replit Agent do not natively solve the local-webhook routing problem in a unified, multi-tenant capable architecture. Our audit of hybrid OS capabilities indicates that OHC can extend its existing reverse-tunnel infrastructure (used for MCP proxying) to solve this. By introducing a Hybrid Webhook Relay, the OHC Cloud Orchestrator can ingest incoming webhooks and securely forward them to the appropriate Standalone Desktop instance. This provides an "Unfair Advantage" by granting local agents cloud-level connectivity without exposing the local machine to the internet.

## Design Doc
**Architecture:**
We need a Webhook Relay service in the Cloud-Native tier and a matching receiver in the Standalone Desktop tier.

1.  **Cloud Relay (Webhook Ingestion)**: A new endpoint in the Go backend (`/api/webhooks/relay/{tenant_id}`) that accepts raw incoming HTTP payloads.
2.  **Routing & Security**: The Cloud Relay looks up active reverse-tunnel connections (established via gRPC/WebSocket with SPIFFE/SPIRE identity) for the target `tenant_id`.
3.  **Local Receiver**: The embedded Go server in the Standalone Desktop listens on the reverse tunnel. When a webhook payload is received, it pushes the event onto the local Redis Pub/Sub (or direct channel if Redis is absent) for the Teammate Mesh to consume.
4.  **Graceful Degradation**: If the local instance is offline, the Cloud Relay should queue the webhook in PostgreSQL (or Redis) and deliver it when the local instance reconnects.

## Implementation Prompt
**Task**: Implement the Hybrid Webhook Relay in `src/server/lib/integrations/`.
1.  **Cloud Relay**: Implement `RelayServer` in Go that exposes an ingestion endpoint. It must authenticate external webhook signatures (e.g., standard HMAC verification) if configured.
2.  **State Management**: Implement a persistent queue mechanism using PostgreSQL to store webhooks if the target local instance is disconnected.
3.  **Tunnel Delivery**: Implement the delivery mechanism over the existing `Teammate Mesh` reverse-tunnel infrastructure.
4.  **Local Receiver**: Implement `LocalReceiver` in the embedded Go server that processes relayed webhooks and dispatches them to the local agent swarm.
5.  **Tests & Metrics**: Write 100% coverage unit tests. Ensure metrics (e.g., `ohc_webhook_relayed_total`, `ohc_webhook_queue_depth`) are exposed via OpenTelemetry.
