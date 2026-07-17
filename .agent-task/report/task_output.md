issue_title: "Design: Offline-First AI Sync Architecture for Mobile Workers"
issue_description: |
  **Title**: Offline-First AI Sync Architecture & Edge Caching for Field Services and Mobile Retail

  **Problem Statement**:
  Our non-technical owners, specifically Carlos (Field Service) and Fatima (Food Cart), frequently operate in environments with flaky, low-bandwidth, or completely disconnected network coverage. Currently, operations like updating service routes, accepting pre-orders, and marking tasks complete rely heavily on continuous server connectivity. When the connection drops, Carlos cannot generate a quote or view route notes, and Fatima's cart cannot acknowledge new local orders or process tap-to-pay offline. This creates immediate revenue loss and severe operational friction, breaking the core "assistant-first" promise of OHC.

  **Research Report**:
  - **Market Context**: Square and Stripe Terminal provide robust offline payment processing (storing authorized transactions and syncing later), but they lack deeply integrated business-logic sync. Shopify's mobile POS supports offline product browsing and cash transactions but struggles with real-time agentic interactions.
  - **Competitor Gaps**: Wix and Squarespace are inherently web-dependent. Field service tools (like Jobber or ServiceTitan) offer some offline caching but don't leverage AI agents to draft quotes or organize tasks while offline.
  - **OHC Opportunity**: By implementing a true Offline-First architecture using local embedded databases (e.g., SQLite/PowerSync for Flutter) combined with our AI agent event mesh, OHC can guarantee that Carlos and Fatima always have a functional workspace. The AI can pre-cache likely needed context (next appointments, popular menu items) when on Wi-Fi.

  **Design Doc**:

  *Architecture Diagram*:
  ```mermaid
  graph TD
      A[Flutter Mobile App] -->|Reads/Writes| B(Local SQLite Cache)
      A -->|Submits Intent| C(Local Event Queue)
      C -.->|Network Restored| D[API Gateway REST/gRPC]
      D --> E[Sync Engine / PowerSync]
      E --> F[(Central PostgreSQL)]
      E --> G[Agent Event Bus]
      G --> H[Operations Agent]
      G --> I[Sales Agent]
      H -->|Action/Draft| F
  ```

  *Mobile UX Flow (375px)*:
  1. **Connectivity Indicator**: A subtle, translucent pill at the top of the UI gracefully transitions from green (Online) to amber (Offline/Syncing) without obstructing the workflow.
  2. **Offline Quoting (Carlos)**: Carlos opens a service request. The UI, powered by locally cached catalog prices, allows him to tap items and generate a quote. The quote is saved locally with a "Pending Sync" status.
  3. **Order Management (Fatima)**: Fatima can toggle menu items to "Sold Out" while offline. The change immediately reflects on her screen.
  4. **Background Sync**: Once the device regains connection, the Local Event Queue flushes automatically. The amber pill spins gently and turns green.

  *AI Agent Integration Points*:
  - **Operations Agent**: Monitors the sync engine. When a batch of offline actions comes in (e.g., Carlos completes 3 jobs), it summarizes them and updates the central dashboard for the day.
  - **Sales Agent**: If Carlos generates a quote offline, the Sales Agent queues up an automated follow-up email/SMS to send to the client as soon as the sync occurs.

  *Key Design Decisions*:
  - **Optimistic UI Updates**: All write actions immediately update the local SQLite store and reflect in the UI, ensuring zero perceived latency for the owner.
  - **CRDTs / Sync Engine**: Use a robust sync solution (e.g., PowerSync) to handle conflict resolution when multiple devices are used.
  - **Pre-fetching Context**: Agents will proactively push the next 48 hours of schedule, customer notes, and catalog updates to the mobile client when a high-speed connection is detected.

  **Implementation Prompt**:
  *For the Implementer Agent*: Build the foundational Offline-First synchronization layer for the Flutter mobile client. Implement a local SQLite database that seamlessly syncs with the central PostgreSQL database using an event-driven sync engine. Ensure that the core Critical User Journey (CUJ) of viewing tasks, generating a simple quote, and marking an item "Sold Out" works flawlessly without an active network connection. The UI must optimistically update and queue these intents, flushing them to the backend when connectivity is restored. Do not prescribe specific library names or table schemas—design the data structures necessary to fulfill this CUJ efficiently. Ensure 100% test coverage for the local queue and sync conflict resolution logic.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
