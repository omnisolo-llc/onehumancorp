issue_title: "[research] Architectural Gap: Real-time Multi-Tenant Sync Infrastructure"
issue_description: |
  # Research Report: Real-time Multi-Tenant Sync Infrastructure

  ## Problem Statement
  Small business owners like Leo (music tutor) and Priya (boutique owner) require their OneHumanCorp (OHC) platform to remain perfectly synchronized across mobile, tablet, desktop, and web views. For example, if Priya updates a product's price from her iPhone while a customer views the website, the web view must reflect the update instantly without page reloads. The existing architecture lacks a unified, horizontally scalable real-time synchronization layer capable of handling tens of thousands of active WebSockets connections while securely maintaining strict multi-tenant isolation.

  ## Research Report
  **Competitive Analysis:**
  - **Shopify:** Utilizes a highly robust, proprietary real-time event system allowing simultaneous updates across merchant tools.
  - **Wix:** Features synchronized data across views, although sometimes reliant on polling.
  - **Supabase/Firebase:** Offer out-of-the-box real-time subscriptions, setting the industry expectation for developers, but OHC requires an integrated, platform-level solution to hide this complexity from the merchant and seamlessly interact with our AI agents.

  **Market Need:**
  The solopreneur expects their business operating system to be real-time. If they accept a booking on their mobile app, their desktop calendar view must update instantly. We need an infrastructure layer that abstracts the complexity of WebSockets, Pub/Sub, and CRDT synchronization, making real-time multi-tenant updates available natively within the OHC platform.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Clients
          Mobile[Mobile App 375px]
          Web[Web Storefront]
      end

      Mobile -- "WebSocket/SSE" --> Gateway[OHC API Gateway]
      Web -- "WebSocket/SSE" --> Gateway

      Gateway --> SyncEngine[Real-time Sync Engine]

      SyncEngine -- "Pub/Sub" --> Redis[(Redis Cluster)]
      SyncEngine --> MainDB[(Cloud Postgres)]

      SyncEngine --> Agents[AI Agent Swarm]

      subgraph Multi-Tenant Isolation
          SyncEngine -- "Verify SVID" --> Spiffe[SPIFFE/SPIRE]
      end
  ```

  ### Mobile UX Flow (375px First)
  1. The user makes an update on their mobile device (e.g., changing an appointment status).
  2. The UI instantly updates locally using optimistic UI updates and CRDTs.
  3. The app establishes a WebSocket/SSE connection to the `Real-time Sync Engine`.
  4. Changes are synchronized to the cloud and broadcasted instantly to any other active sessions associated with the same `tenant_id`.

  ### AI Agent Integration Points
  - **Operations Agent:** Receives real-time state changes and triggers background workflows (e.g., sending appointment reminders) without polling the database.
  - **Sales Agent:** Can instantly push notifications directly to the merchant's mobile device via the sync engine when a high-value lead is identified.

  ### Key Design Decisions
  - **Zero Trust & Security:** All WebSocket connections must be authenticated and authorized using SPIFFE/SPIRE SVIDs to enforce strict multi-tenant isolation. A user must only receive events for their `tenant_id`.
  - **Performance Targets:** The sync engine must target sub-100ms latency for broadcasting events across connected clients.
  - **Protocol Agnostic:** Support both WebSockets and Server-Sent Events (SSE) to ensure compatibility across different network conditions and client capabilities.

  ## Implementation Prompt
  Implement the Real-time Multi-Tenant Sync Infrastructure.
  - **User-Facing Outcome:** Users experience seamless, instant updates across all their devices. If they update inventory on their phone, the web storefront updates instantly without a refresh.
  - **CUJ (Critical User Journey):**
    1. User modifies an entity (e.g., a product or booking) on Device A.
    2. Device A sends the mutation to the backend.
    3. The backend validates the mutation, updates the database, and publishes the event to the Sync Engine.
    4. The Sync Engine broadcasts the event to all authenticated clients for that tenant.
    5. Device B receives the event and updates its UI instantly.
  - **Acceptance Criteria:**
    - Real-time event broadcasting to multiple connected clients.
    - Strict multi-tenant isolation; clients cannot subscribe to or receive events from other tenants.
    - Integration with SPIFFE/SPIRE for secure identity verification.
    - Target sub-100ms broadcast latency.
    - Graceful fallback and reconnection logic for clients with unstable connections.

  ## Priority
  P0

  ## Estimated Scope
  Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, high-impact, architecture]
assignees: []
