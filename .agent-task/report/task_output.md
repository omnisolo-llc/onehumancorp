issue_title: "[architecture] Offline-First Edge Sync & Real-Time Push Architecture"
issue_description: |
  # Offline-First Edge Sync & Real-Time Push Architecture

  ## Problem Statement
  Small business owners often operate in environments with poor or intermittent internet connectivity (e.g., Fatima’s food cart in a crowded plaza, Priya’s boutique in a thick-walled building, or Carlos at a rural job site). Currently, a slow connection causes the OneHumanCorp (OHC) app to hang during critical actions like toggling an item "Sold Out", taking an in-person payment, or uploading menu photos. For non-technical users, a hanging app creates panic and lost revenue. They need the app to feel instantaneous and reliable, regardless of the network state, with loud, immediate notifications when online orders arrive.

  ## Research Report
  ### Competitor Analysis
  - **Shopify POS:** Relies heavily on local caching for product catalogs, allowing in-person sales to proceed offline. Syncs when the network is restored.
  - **Square POS:** Supports "Offline Mode" for swiped/dipped payments (queued locally and processed later) and inventory management.
  - **Linear (Issue Tracker):** Built on a sync engine that makes the UI instantly responsive. All mutations are applied to a local SQLite/IndexedDB store first and synced to the cloud asynchronously.

  ### Gap in OHC
  OHC lacks a structured local-first mutation queue and conflict resolution system. If Fatima toggles "Sold Out" while offline, the HTTP request fails or hangs. If she gets an order while her phone is locked, polling fails to wake the app. We need a robust Edge Sync Engine and a dedicated WebSockets/Push notification pipeline.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Mobile Client (375px Viewport)
          UI[Translucent Glass UI Cards]
          LocalDB[(Local Edge SQLite)]
          MutQueue[Local Mutation Queue]
          SyncEngine[Edge Sync Engine]
          UI -->|Reads/Writes| LocalDB
          UI -->|Queues action| MutQueue
          MutQueue --> SyncEngine
      end

      subgraph OHC Zero-Trust Cloud
          API[OHC API Gateway]
          MultiTenantDB[(Tenant Postgres)]
          ConflictRes[AI Operations Dept]
          PushService[FCM/APNs Push Service]

          SyncEngine --|TLS / SPIFFE|--> API
          API --> MultiTenantDB
          API --> ConflictRes
      end

      API -->|Real-time Order| PushService
      PushService -->|Wake App / Loud Chime| UI
  ```

  ### UI Wireframes & Mobile UX Flow (375px)
  1. **Network Status Indicator:** A subtle, translucent pill at the top of the screen: "Offline - Changes saved locally" (using OHC design tokens, warm amber).
  2. **Instant Action (Fatima):** Fatima taps the "Sold Out" toggle on a Falafel card. The toggle animates instantly to "Sold Out" (optimistic UI update) and a tiny sync icon spins next to it.
  3. **Conflict UI:** If a conflict occurs (e.g., sold offline but ordered online 1 second prior), the UI shows a clean, friendly prompt: "We got an online order just before you marked this sold out. Should we refund the online order?" with a 1-tap "Yes, refund" button (handled by AI Operations).
  4. **Push Notification:** A custom loud notification chime (bypassing silent mode if the user permits, via Critical Alerts) for "New Order".

  ### AI Agent Integration Points
  - **Operations Department (Conflict Resolution):** Listens to the Sync Conflict Ledger. If concurrent offline and online actions clash (e.g., inventory drops below zero), the Operations Agent automatically analyzes the timestamp and drafts a friendly resolution (like an SMS to the online customer offering a free substitute or instant refund).

  ### Key Design Decisions
  - **Local-First Reads/Writes:** The mobile app will ONLY read from and write to the local embedded store (LocalDB). This guarantees 0ms latency for UI interactions.
  - **Background Sync Engine:** A dedicated background service manages the Mutation Queue, attempting to sync with the OHC backend via a secure (SPIFFE-authenticated) channel whenever network conditions permit.
  - **Optimistic Concurrency Control:** Backend uses version vectors to detect conflicts. Multi-tenancy isolation is enforced at the API gateway layer during sync.
  - **No Manual Resolution:** We do not expose merge conflicts to the business owner. All conflicts are resolved invisibly by the Operations AI or presented as a simple "A or B" business decision.

  ## Implementation Prompt
  **Objective:** Implement the Local-First Edge Sync Engine for the mobile client and the corresponding conflict-aware API endpoints in the backend.

  **User Journey (CUJ):**
  1. Fatima opens her app (no internet).
  2. She toggles "Falafel" to "Sold Out". The UI updates instantly.
  3. She regains internet 10 minutes later. The app silently syncs this state to the backend.

  **Acceptance Criteria:**
  - The mobile frontend must write all data changes to a local embedded database and queue a mutation event.
  - The UI must reflect changes instantly without waiting for a network response.
  - The backend must expose a sync endpoint that accepts batches of queued mutations, enforcing multi-tenant isolation.
  - Push notifications must trigger for new orders, waking the mobile app.
  - AI Operations agent must have a hook to process any inventory conflicts arising from delayed syncs.
  - Adhere strictly to the translucent glass UI design system for any status indicators, hiding technical details behind "Advanced Settings".
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
