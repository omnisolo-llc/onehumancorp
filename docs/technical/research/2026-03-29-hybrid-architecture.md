<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Design Doc: OHC Hybrid Agentic OS & Thin Client Architecture

**Author(s):** Antigravity, Principal Product Architect & Visionary (L7)
**Status:** Approved
**Last Updated:** 2026-03-29

## 1. Overview
The **One Human Corp (OHC) Hybrid Agentic OS** requires a fluid and consistent architecture across completely different operating models. This document solidifies the technical spec for the "Standalone" wrapper (Local First) and "Thin Client" API definitions, ensuring that the OHC "Premium Feel" and robust multi-agent orchestration are indistinguishable regardless of deployment tier.

## 2. Goals & Non-Goals
### 2.1 Goals
- Define the Standalone Desktop Wrapper lifecycle, bridging local Rust server execution with the Slint application shell.
- Specify the API contract for the Thin Client Mode, guaranteeing API stability and offline-resilience strategies.
- Enforce OHC-SIP v2 (Swarm-as-Code) consistency across PostgreSQL (Cloud) and SQLite (Local).
- Mandate the Visual Excellence (Aesthetic) Standard across all Hybrid clients.

### 2.2 Non-Goals
- We are not writing a custom ORM to abstract PostgreSQL vs SQLite. We will use robust standard Rust abstractions (e.g., `sqlx` for async database operations with connection pooling).
- We are not deploying a full Kubernetes cluster within the Standalone wrapper; local resources are managed via Rust async tasks and local process execution.

## 3. Hybrid Deployment Architecture

### 3.1 Structural Blueprint

```mermaid
graph TD
    %% Client Tier
    subgraph "Thin Client Mode (Remote/API-Only)"
        MobileThin[Mobile App UI]
        DesktopThin[Desktop App UI]
        WebThin[Web App UI]
    end

    subgraph "Standalone Mode (Local)"
        DesktopFat[Desktop Wrapper UI]
        LocalRust[Embedded Rust Server]
        LocalDB[(SQLite SIPDB)]
    end

    %% Cloud Tier
    subgraph "Cloud-Native Mode (Multi-Tenant)"
        K8sAPI[K8s Rust API Server]
        Postgres[(PostgreSQL)]
        Redis[(Redis)]
        Hub[Orchestration Hub]
    end

    %% Connections
    DesktopThin -->|HTTP/OAuth| K8sAPI
    WebThin -->|HTTP/OAuth| K8sAPI

    DesktopFat -->|Local gRPC/HTTP| LocalRust
    LocalRust -->|File I/O| LocalDB
    LocalRust -->|Sync| K8sAPI

    K8sAPI --> Postgres
    K8sAPI --> Redis
    K8sAPI --> Hub

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class Hub,DesktopFat,LocalRust,LocalDB,Postgres,WebThin,DesktopThin,K8sAPI,Redis premium;
```

### 3.2 OHC-HA Degradation Model
- **Cloud-Native**: High concurrency, strict tenant isolation, distributed caching (Redis).
- **Standalone Mode**: SQLite fallback, single-user identity (bypassing strict JWT tenant requirements for local owner), gracefully disables distributed cache pathways.
- **Thin Client**: Prioritizes UI responsiveness and API latency. Leverages local state caching before syncing with the Cloud via `/api/sync`.

## 4. Standalone Wrapper Spec

### 4.1 Lifecycle Management
The Slint desktop shell acts as the supervisor for the embedded Rust backend.
1.  **Boot**: App starts -> Checks for `OHC_STANDALONE=true` -> Spawns `ohc-server` child process -> Waits for `/healthz`.
2.  **State**: App points internal HTTP clients to `http://localhost:<dynamic_port>`.
3.  **Teardown**: App closed -> Sends graceful shutdown signal (SIGTERM) to Rust process.

### 4.2 SQLite/PostgreSQL Parity
To guarantee parity, the Rust backend uses a unified `DataStore` trait.
- Local: SQLite `file:///.ohc/runtime/swarm.db`.
- Cloud: PostgreSQL DSN.
The underlying schema must remain 100% compatible. Complex JSONB queries in Postgres are translated to SQLite JSON functions.

## 5. Thin Client API Definitions

### 5.1 Remote Connection Flow
Thin clients connect via standard REST/gRPC.

| Endpoint | Method | Role | Response |
| :--- | :--- | :--- | :--- |
| `/api/auth/handshake` | `POST` | Exchanges credentials/OAuth for JWT. | `{ "token": "...", "tenant_id": "...", "mode": "cloud" }` |
| `/api/sync/state` | `GET` | Fetches delta updates for offline support. | `{ "meetings": [...], "agents": [...] }` |
| `/api/agents/command` | `POST` | Dispatches instructions to the swarm. | `{ "task_id": "123", "status": "processing" }` |

### 5.2 Network Resilience
If the Thin Client loses connection, it buffers actions locally (e.g., in Hive/SharedPreferences) and replays `POST /api/sync/mutations` upon reconnection.

## 6. Aesthetic Excellence Mandate

All UIs, regardless of Standalone or Thin Client mode, strictly adhere to the OHC Premium Feel:

*   **Glassmorphism Container**:
    ```css
    backdrop-filter: blur(20px) saturate(200%);
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid rgba(255, 255, 255, 0.1);
    ```
*   **Typography**: `Outfit`, `Inter`.
*   **Animations**: Staggered list entries and 200ms ease-out transitions for state changes.

## 7. Next Steps & Agentic Delegation

- Hand off database abstraction and SQLite schema generation to the Forge (`backend_dev`) agent.
- Ensure the Slint UI perfectly maps these capabilities across target platforms.

</div>
