<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 24px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #ffffff; box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.3);">

# Mission: Implement Hybrid MCP SPIFFE/SPIRE Zero-Trust Identity Mesh

## Problem Statement
Current competitors like Claude Code, OpenClaw, and Replit Agent either rely on massive cloud infrastructures or pure local CLI wrappers. They lack a zero-friction, bidirectional sync capability powered by a single cohesive identity layer (SPIFFE/SPIRE). As OHC agents transition between air-gapped local execution (SQLite) and scalable cloud coordination (PostgreSQL), they risk token fragmentation and security vulnerabilities when communicating via Model Context Protocol (MCP) tool routing.

## Research Report
### Competitive Audit
| Feature / Architecture | Claude Code | OpenClaw | Replit Agent | **OHC Hybrid (Proposed)** |
| :--- | :--- | :--- | :--- | :--- |
| **Execution Domain** | Local CLI (Single User) | Cloud-Native K8s | Cloud Container | **Hybrid (Local SQLite + K8s)** |
| **Agent Autonomy** | High (Local File I/O) | High (Cloud API) | Medium (Tethered) | **Absolute Autonomy** |
| **Zero-Trust Identity** | N/A (Local Only) | Static API Keys | Managed Platform Auth | **SPIFFE/SPIRE SVIDs** |
| **Swarm Intelligence** | None | Global Postgres | Workspace Context | **Teammate Mesh via OHC-SIP** |

### Synthesis
OHC can capitalize on this "Blue Ocean" opportunity. We must implement a unified Identity Middleware Router that uses SPIFFE Verifiable Identity Documents (SVIDs) for all agent-to-agent and agent-to-tool communications, eliminating the need for hardcoded credentials.

## Design Doc

### 1. Architecture
We will integrate a `SpiffeIdentityMiddleware` into the existing Teammate Mesh and MCP execution pipelines.

```mermaid
graph TD;
    A[Standalone Desktop Agent] -->|Request SVID| B(SPIRE Agent Local);
    B -->|Issue JWT-SVID| A;
    A -->|MCP Execution Request + SVID| C{OHC K8s Ingress / API};
    C -->|Verify SVID| D[SpiffeIdentityMiddleware];
    D -->|Valid| E[Cloud Orchestration / pgvector];
    D -->|Invalid| F[Reject: Unauthorized];

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,C,D,E,F premium;
```

### 2. Mechanism
- The `SpiffeIdentityMiddleware` will wrap existing API handlers (e.g., `POST /api/mesh/broadcast`).
- It will extract the `Authorization: Bearer <SVID>` header.
- It will validate the SVID against the SPIRE Trust Domain.
- This fulfills the absolute autonomy and zero secrets constraints defined in the `CLAUDE_OHC.md` architecture doc.

## Implementation Prompt

**Hello Implementer Agent!** Your objective is to build out the SPIFFE/SPIRE Identity Middleware to secure our Hybrid MCP communications.

1. **Create the Middleware**: Create a new file `srcs/server/api/middleware/spiffe.go`.
2. **Implement Validation**: Implement `SpiffeIdentityMiddleware(next http.Handler) http.Handler`. It must read the `Authorization` header, parse the JWT-SVID, and validate it using the `spiffe/v2/spiffetls` library (add it to `go.mod` if necessary).
3. **Handle Missing Auth**: If the SVID is missing or invalid, return a `401 Unauthorized`.
4. **Integration**: Apply this middleware to the `POST /api/mesh/broadcast` route in the existing routing setup (e.g., `srcs/server/api/mesh.go` or `router.go`).
5. **Testing**: Write comprehensive unit tests in `srcs/server/api/middleware/spiffe_test.go` mocking valid and invalid SVIDs.
6. **Constraints**: Ensure no static API keys or PATs are required. Follow the strict Domain scope rules.

## Priority
`P0` (Critical)

## Estimated Scope
Medium

</div>
