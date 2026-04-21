<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# OHC Hybrid Data Sync Research Report

## Executive Summary
This report details the investigation into providing a robust, scalable, and fully autonomous local-to-cloud database synchronization mechanism for the One Human Corp (OHC) Hybrid Architecture. The core challenge is maintaining seamless state consistency between the Cloud-Native Mode (PostgreSQL) and the Standalone Desktop Mode (SQLite) while supporting offline-first operations and complex conflict resolution.

## Problem Context
The OHC Hybrid Architecture requires that agents and users can operate seamlessly whether connected directly to the cloud or running entirely locally. The Standalone Desktop mode utilizes SQLite for extreme resource efficiency. However, when the user transitions online or when local agents need to share state with cloud-based agents via the OHC Central Database (OHC-SIP), data must be synchronized bi-directionally without manual intervention. Custom-building this synchronization layer, handling Write-Ahead Log (WAL) reading, and managing distributed conflict resolution represents a significant diversion of engineering resources away from core agentic AI development.

## Tool Evaluation

We evaluated three primary contenders for this critical piece of infrastructure:

1.  **ElectricSQL**:
    *   **Pros**: Excellent active-active replication, strong guarantees.
    *   **Cons**: Deployment is complex, requires specific Postgres extensions that may complicate multi-tenant Kubernetes deployments. Flutter support is currently less mature than other options, making it a higher risk for our UI-centric thin clients.
2.  **RxDB**:
    *   **Pros**: Fantastic reactive capabilities, very strong in the JavaScript/Web ecosystem.
    *   **Cons**: It is fundamentally a NoSQL/document-oriented approach. Adapting it to our strictly relational OHC-SIP (PostgreSQL/SQLite) schema would require massive impedance matching. It is also heavy for a native desktop/mobile hybrid environment.
3.  **PowerSync**:
    *   **Pros**: Built specifically for SQLite-to-PostgreSQL replication. It reads logical replication streams directly from Postgres (pgoutput) and syncs to local SQLite databases. It has a native, well-supported Flutter SDK (`powersync_flutter`). It handles offline-first natively and allows for dynamic sync rules, which is crucial for maintaining multi-tenant security when syncing data down to a single-user Standalone instance.
    *   **Cons**: Introduces a new service (PowerSync Service) into the Cloud-Native deployment architecture.

## Decision & Justification
**PowerSync is the clear winner for the OHC Hybrid Architecture.** Its native alignment with our exact database stack (PostgreSQL + SQLite) and our UI framework (Flutter) reduces integration friction dramatically. The ability to define dynamic sync rules ensures we can adhere to our strict tenant isolation requirements in Cloud Mode while empowering the Standalone Mode.

## Proposed Integration Blueprint

### 1. Cloud-Native Infrastructure (Backend)
*   **PostgreSQL Configuration**: Enable logical replication (`wal_level = logical`).
*   **Service Deployment**: Deploy the PowerSync Service container within our Kubernetes orchestration.
*   **Sync Rules Definition**: Create a declarative ruleset (`sync_rules.yaml`) defining which OHC-SIP tables and rows sync to which authenticated clients, based on their SPIFFE/SPIRE identities.

### 2. Standalone Desktop & Thin Client (Frontend)
*   **Dependency Injection**: Add the `powersync_flutter` SDK to the Flutter application.
*   **Data Abstraction Layer**: Refactor the existing local SQLite access patterns (e.g., in `srcs/client/data/database.dart`) to route through the PowerSync SQLite engine.
*   **Auth Integration**: Hook the PowerSync client into our existing SPIFFE/SPIRE authentication flow to request sync tokens.

## Next Steps
An issue brief has been created in `docs/research/[feature]_powersync_hybrid_data_sync.md` to hand off the implementation to an Implementer Agent.

</div>
