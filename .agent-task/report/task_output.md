issue_title: "[Architecture] Offline-First Resilient Sync Engine"
issue_description: |
  ## Problem Statement
  Small business owners like Fatima (operating a food cart in areas with patchy cellular service) and Carlos (working as a handyman in basements or remote areas) frequently lose network connectivity. Traditional cloud-dependent systems prevent them from processing orders, updating inventory, or accessing customer details when offline, leading to lost revenue and frustration. They need a system that feels instantaneous and reliable, regardless of network conditions, and seamlessly syncs in the background once connectivity is restored.

  ## Research Report
  Current market solutions handle offline capabilities poorly:
  - **Shopify POS:** Offers offline capabilities primarily for cash transactions but lacks robust offline-first architecture for complex inventory syncs or customer profile updates without connectivity.
  - **Square:** Provides "Offline Mode" for payments, but with strict limitations (e.g., declined cards when back online, missing dynamic pricing).
  - **GoDaddy/Wix:** Completely web-dependent; essentially non-functional in offline scenarios.

  **Opportunity for OHC:**
  By treating offline operation as the default rather than an edge case, OHC can capture the mobile-first micro-business market. Using local-first database technologies (like SQLite/PowerSync on mobile) coupled with CRDTs (Conflict-free Replicated Data Types) or logical clocks, OHC can ensure that operations (e.g., marking an item as sold out, drafting an invoice) happen instantly with zero loading spinners.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Mobile Device (375px)
          UI[Flutter UI - Optimistic Updates]
          LocalDB[(Local SQLite)]
          SyncQueue[Background Sync Queue]
      end

      subgraph OHC Cloud (Backend)
          API[gRPC / REST Gateway]
          Ledger[(PostgreSQL - Tenant Isolated)]
          ConflictResolver[Conflict Resolution Engine]
      end

      UI -->|Reads/Writes| LocalDB
      UI -->|Enqueues| SyncQueue
      SyncQueue -->|Network Available| API
      API --> Ledger
      Ledger --> ConflictResolver
      ConflictResolver -->|Resolves| Ledger
      API -.->|Server-Sent Events / Push| LocalDB
  ```

  ### Mobile UX Flow (375px first)
  1. **Action:** Fatima taps "Sold Out" on her signature Halal Chicken platter.
  2. **Immediate Feedback:** The UI updates instantly. A subtle "Saved offline" icon (e.g., a cloud with a slash) appears in the corner.
  3. **Background Process:** The action is saved to the local SQLite database and added to the SyncQueue.
  4. **Reconnection:** When the device regains a connection, the SyncQueue pushes the update to the OHC backend. The "offline" icon fades out smoothly.

  ### Key Design Decisions
  - **Local-First Reads/Writes:** All UI reads and writes hit the local database first. This guarantees sub-50ms latency for interactions.
  - **Optimistic UI:** Assume all local writes will succeed globally.
  - **Background Syncing:** Use native OS background sync mechanisms to drain the queue when connectivity is restored.
  - **Conflict Resolution:** Use "Last Write Wins" (LWW) based on timestamps for simple entities (e.g., product descriptions), and CRDT-like strategies for complex ones (e.g., inventory counts).

  ## Implementation Prompt

  **User-Facing Outcome:**
  Users can perform core business operations (create an order, update inventory, draft an invoice) without any network connection. The application will respond instantly without loading spinners. Once back online, the app will automatically synchronize these changes with the cloud.

  **Critical User Journeys (CUJ):**
  1. User logs in (requires network).
  2. User loses network connection.
  3. User navigates to their catalog and updates the price of an item. The UI reflects the change immediately.
  4. User creates a new draft order for a customer.
  5. User regains network connection.
  6. The app silently syncs the price update and the new order in the background. No data is lost.

  **Acceptance Criteria:**
  - The app must read from and write to a local persistent store (e.g., SQLite) as the primary source of truth for the UI.
  - All network interactions for data mutation must be handled via a background queue that retries upon network failure.
  - Implement a basic conflict resolution strategy on the backend to handle conflicting updates from multiple devices owned by the same tenant.
  - Ensure the UI clearly but unobtrusively indicates offline status.
  - Zero usage of `Future.delayed` or artificial loading states for local mutations.
  - Write E2E Playwright tests simulating offline mode (using Playwright's network interception to block requests) to verify local writes and subsequent sync.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
