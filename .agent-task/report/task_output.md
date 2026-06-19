issue_title: "Architectural Gap: Offline-First SQLite Sync Engine for Mobile Operations"
issue_description: |
  **Title**: Architectural Gap: Offline-First SQLite Sync Engine for Mobile Operations

  **Product-use evidence**:
  - **Persona**: Carlos (Field Service Owner)
  - **Observed Flow Attempt**: Attempted to start the stack with `docker compose -f deploy/docker-compose.yml up -d --build` to simulate Carlos's daily mobile workflow. The product could not launch due to an external Docker Hub unauthenticated pull rate limit (`Error response from daemon: error from registry: You have reached your unauthenticated pull rate limit. https://www.docker.com/increase-rate-limit`).
  - **Exception applied**: As per the Startup Exception protocol, the live UI pass was blocked. However, based on architectural review and the `Standalone Mode` documented in `README.md`, an offline CQRS sync engine is structurally missing for the Flutter client.

  **Superpowers Skills Loaded**:
  Applied standard systematic discovery and documentation research workflows from `https://github.com/obra/superpowers/`, shaping the codebase audit and the architectural gap identification.

  **Problem Statement**:
  Users like Fatima (Food Cart Operator) and Carlos (Field Service Owner) frequently operate in environments with poor or flaky mobile data (events, basements, remote sites). Currently, OHC's critical writes and state mutations assume a reliable connection to the backend API. If the network drops, they cannot record a pre-order, process a deposit, or update a route note. The owner needs absolute confidence that when they tap "Save" or "Accept Payment," the app captures the action immediately and reconciles it seamlessly in the background when connectivity is restored, without throwing a technical error.

  **Research Report**:
  - **Market Context**: Shopify Point of Sale and Square Terminal both feature robust offline modes that cache transactions and sync later. Linear uses an offline-first sync engine that applies mutations locally and queues them.
  - **Codebase Findings**: We have `powersync` in `deploy/docker-compose.yml` and mentions of "Local SQLite SIPDB" in `README.md` ("Standalone Mode"), but no cohesive, platform-wide offline-first write queue and conflict resolution engine in the Flutter client for mobile operations. The "Hybrid RAG Workflow" in `docs/vision/market_strategy.md` describes SQLite DB sync via "OHC-SIP", but the operational data (bookings, tasks, menu toggles) needs a strict CQRS/Event-Sourcing layer on the Flutter side.

  **Design Doc**:
  - **Architecture Diagram (Mermaid.js)**:
    ```mermaid
    graph TD
      A[Flutter UI (375px)] --> B[Local SQLite Repository]
      B --> C{Action Queue (CQRS)}
      C -->|Offline| D[Pending Mutations Table]
      C -->|Online| E[Sync Engine / OHC-SIP]
      E --> F[API Gateway / KAIROS]
      F --> G[(Postgres Tenant DB)]
    ```
  - **Mobile UX Flow**:
    1. Owner operates normally on a 375px screen.
    2. Connection drops. A subtle, translucent amber indicator appears: "Working offline. Changes saved."
    3. Owner updates a task or completes an order. The UI updates instantly with a local optimistic mutation.
    4. When connection returns, the indicator changes to "Syncing..." and disappears upon completion.
  - **AI Agent Integration**:
    - The `Operations Assistant` can detect long-running sync conflicts (e.g., double-booked slots offline) and proactively draft a resolution message to the customer ("Apologies, that time slot was just taken, how about 2PM?").
  - **Zero Trust & Multi-Tenancy**:
    - Local SQLite databases are strictly partitioned by `tenant_id` and encrypted at rest on the mobile device to prevent data leakage in shared-device scenarios.

  **Implementation Prompt**:
  - Implement a generic offline-first Action Queue in the Flutter frontend. Define an `OfflineAction` interface with optimistic UI updates. When an action is dispatched, write it to a local SQLite `pending_actions` table and update the local read model immediately. Write a background worker that drains the `pending_actions` queue to the backend API when `ConnectivityResult` is positive. Include retry logic with exponential backoff. Do not prescribe specific backend API changes, assume a REST/gRPC endpoint will accept these batched actions. Add E2E Playwright tests simulating offline/online toggles.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
