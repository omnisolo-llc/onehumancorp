issue_title: "Research: Mobile-First Agentic Workflows for SMB Operators"
issue_description: |
  # Research Report: Architectural Gaps & Mobile Parity for SMB Operators

  ## Problem Statement
  OneHumanCorp (OHC) is designed to be an AI work assistant for the person responsible for the outcome (owners, operators). However, current platform architecture and documentation do not explicitly outline a highly resilient, offline-tolerant, distributed-sync data layer needed by mobile operators like Carlos (field services) and Fatima (food cart). Mobile operators experience flaky networks, low-end devices, and need instantaneous local reads/writes that sync seamlessly with the backend. Our current GRPC/REST implementations risk creating blocking network dependencies and failing the "375px phone screen offline usability" requirement.

  ## Research Report
  - **Market Dynamics**: Leading solutions like Shopify POS, Square, and Toast implement offline-first data architectures. Square Terminal and Toast handhelds use a robust local replication strategy to allow uninterrupted business operations during internet outages, syncing transactions when connectivity is restored.
  - **Competitive Deficit**: OHC currently lacks a standardized local-first data syncing mechanism for the Flutter frontend, relying heavily on standard network request-response models which are insufficient for mission-critical physical retail, field service, or low-data environments.
  - **Technical Gap**: The current multi-tenant Row-Level Security (RLS) backend (PostgreSQL) is strong, but the bridge to the mobile client lacks an offline-first replication layer (e.g. CRDTs, SQLite + PowerSync, or WatermelonDB equivalents for Flutter). The agentic capabilities (e.g., automated follow-ups) can only function effectively if the source of truth accurately reflects offline-captured state once synced.

  ## Design Doc
  ### Proposed Architecture Additions (High-Level)
  - **Local-First Data Layer**: Introduce an offline-first synchronization protocol between the Flutter Client (using local SQLite) and the Go/Bazel Backend.
  - **Optimistic UI & Event Queuing**: The Flutter shell must adopt an optimistic update strategy for all critical writes (e.g., capturing a lead, starting a service job, taking a pre-order). Mutations should be locally queued and background-synced using exponential backoff.
  - **Agent State Reconciliation**: AI agents triggered by webhooks or state changes must handle delayed or batched event streams gracefully, incorporating idempotency keys (already specified for Payments) across all domain models to prevent duplicate agent actions on sync bursts.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant App as Flutter Client
      participant SQLite as Local Database (SQLite)
      participant Backend as Go Backend (RLS PostgreSQL)
      participant AI as Agent Subsystem

      App->>SQLite: Create local Entity (e.g., Order, Lead)
      App-->>App: Optimistically Update UI
      Note over App,Backend: Network Interrupted

      Note over App,Backend: Network Restored
      App->>Backend: Sync local Mutation Queue (w/ Idempotency Key)
      Backend->>Backend: Process mutation, Insert into DB
      Backend->>AI: Trigger Agents (e.g., Notifications)
      Backend-->>App: Ack Sync Success
      App->>SQLite: Remove from Sync Queue
  ```

  ### UX Flow & Integration
  - **375px UX Impact**: The UI remains instantaneous. A small, non-intrusive sync indicator (e.g. "Working Offline", "Syncing...", "All caught up") becomes a standard top-level OHC Premium Token in the Assistant-First Shell.
  - **Agent Handoff**: If Maya enters a new order while offline, the local app accepts it. Upon reconnection, the data syncs, and the Operations Assistant agent seamlessly picks up the new row to trigger deposit requests without duplicate messages.

  ## Implementation Prompt
  **Goal:** Design and implement a prototype for the Local-First Sync Protocol across the Flutter frontend and a core backend entity (e.g., `Tasks` or `Customer Contacts`).

  **Acceptance Criteria:**
  1. The Flutter UI allows creation of the entity while the device network is fully disabled.
  2. The UI optimistically reflects the created entity instantly.
  3. When network is restored, the local queue syncs the entity to the Go backend.
  4. The backend acknowledges the creation, avoiding duplicates via idempotency keys, and triggers any downstream AI agents safely.
  5. The UI updates to reflect the backend-confirmed state without user disruption.
  6. E2E tests must verify this exact offline-to-online journey using Playwright network interception to simulate the outage.

  ## Priority & Scope
  - **Priority**: P1 (High)
  - **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
