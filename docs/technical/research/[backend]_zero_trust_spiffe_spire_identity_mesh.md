# Zero Trust SPIFFE/SPIRE Identity Mesh for AI Agents

## Problem Statement
Currently, OneHumanCorp (OHC) agents interact with various internal services (Ledger, CRM, Core API) using implicit trust or static API keys. As the platform scales and we introduce third-party integrations (e.g. custom tools via MCP), this model becomes a security risk. A compromised agent could theoretically impersonate another or access unauthorized tenant data. We need a cryptographic, identity-based Zero Trust architecture to securely authenticate and authorize every agent-to-service and agent-to-agent interaction.

## Research Report
Traditional API keys are static, easily leaked, and hard to rotate. The industry standard for zero-trust microservice identity is SPIFFE (Secure Production Identity Framework for Everyone) and its implementation, SPIRE.
- **SPIFFE/SPIRE**: Automatically issues short-lived, rotatable cryptographic identity documents (SVIDs) based on runtime attestation (e.g. Kubernetes pod properties, AWS instance roles).
- **Competitors**: Enterprise platforms like Shopify use internal service mesh identity (e.g., Istio mTLS) but OHC's unique challenge is extending this identity dynamically to ephemeral, potentially third-party AI agents via the MCP protocol.

## Design Doc

### Architecture Diagram (Mental Model)
*   **SPIRE Server**: Deployed centrally in Kubernetes. Defines the trust domain (e.g., `trust.ohc.dev`).
*   **SPIRE Agents**: Run as DaemonSets on every node. Attest workloads (AI Agents, Rust Services).
*   **Workload API**: AI Agents connect locally to the Workload API (Unix Domain Socket) to receive their X.509 SVIDs (SPIFFE Verifiable Identity Documents).
*   **gRPC / MCP Interceptors**: All Rust gRPC servers and the MCP Hub implement mTLS using SVIDs. They parse the SPIFFE ID (e.g. `spiffe://trust.ohc.dev/tenant/123/agent/marketing`) to enforce fine-grained, tenant-isolated authorization.

### Mobile UX Flow
This is a purely backend infrastructure change and is completely invisible to the user. The mobile app continues to use standard JWTs for user authentication, which the API gateway exchanges for a backend SVID context when making internal calls.

### AI Agent Integration
*   Agents no longer need hardcoded API keys. They natively support SPIFFE SVIDs for mTLS connections.
*   Cross-agent coordination via the Mesh uses SVIDs to guarantee message origin.

## Implementation Prompt
Deploy SPIRE into the OHC Kubernetes environment. Configure node and workload attestation. Implement a Rust library wrapping the SPIFFE Workload API to automatically fetch and rotate X.509 SVIDs. Update the `server_lib` gRPC configuration to mandate mTLS using these SVIDs. Implement an authorization interceptor that maps SPIFFE IDs to tenant contexts and enforces access control.

**Priority**: P1
**Estimated Scope**: Medium
