issue_title: "[architecture] Unified Capacity & Inventory Mesh"
issue_description: |
  ## Title
  [architecture] Unified Capacity & Inventory Mesh for Seamless Omnichannel Bookings & Sales

  ## Problem Statement
  Small business owners often juggle physical inventory, digital products, and time-based services simultaneously. For example, Leo (music tutor) sells physical guitar strings, digital lesson PDFs, and books 1:1 online sessions. Priya (boutique owner) sells physical dresses and books in-person styling consultations. Currently, industry standards force these into separate paradigms: an "e-commerce catalog" (Shopify style) vs a "booking calendar" (Calendly style). Managing these separate pools of availability leads to double-booking, stockouts, fragmented revenue reporting, and intense operational fatigue. Our users need a unified backend that treats a 1-hour time slot, a physical item on a shelf, and a digital download as interchangeable units of "capacity", allowing them to manage their entire business from a single mobile view.

  ## Research Report
  ### Market Analysis & Competitor Gaps
  - **Shopify:** Heavily optimized for physical/digital goods. Time-based bookings require complex, expensive third-party apps (e.g., BookThatApp) that patch into the cart via fragile webhooks. This breaks the "grandmother test" and adds Cost Creep.
  - **Wix / Squarespace (Acuity):** Offer both products and services natively, but they exist as entirely separate modules. A customer cannot easily add a physical product and a service booking to the exact same seamless checkout experience.
  - **OHC Opportunity:** By introducing a **Unified Capacity & Inventory Mesh**, we can allow a customer to add a "Guitar Lesson (1hr)", a "Digital Sheet Music PDF", and a "Pack of Guitar Strings" to the *same* cart, checking out seamlessly. The AI Finance Agent can automatically split the ledger, and the AI Operations Agent updates the unified capacity mesh instantly.

  ## Design Doc
  ### Data Model & Architecture Diagram
  ```mermaid
  erDiagram
      TENANT {
          string id
          string name
      }
      CAPACITY_NODE {
          string id
          string tenant_id
          string type "TIME | PHYSICAL | DIGITAL"
          int available_count
          json constraints
      }
      RESERVATION_LOCK {
          string lock_id
          string node_id
          string session_id
          timestamp expires_at
      }

      TENANT ||--o{ CAPACITY_NODE : owns
      CAPACITY_NODE ||--o{ RESERVATION_LOCK : manages_locks
  ```

  ### Mobile-First UX Flow (375px)
  - **Unified Creation Flow:** A single "Add Product/Service" Floating Action Button (FAB) in the mobile dashboard using translucent glassmorphic materials.
  - **Contextual Cards:** A Ubiquiti UniFi style modular card interface where users toggle between "Physical Item", "Time Slot", and "Digital File" without changing screens.
  - **Smart Counters:** A unified "Availability" section that morphs contextually: for Time, it displays a tap-to-select calendar; for Items, it shows a large, thumb-friendly number pad.

  ### AI Agent Integration Points
  - **AI Operations Agent:** Continuously monitors the `CAPACITY_NODE`. Triggers low-stock warnings for physical goods, or automatically blocks off `TIME` nodes if the owner texts the Omnichannel AI Inbox saying, "I'm sick today, cancel my afternoon."
  - **AI Finance Agent:** Reconciles mixed-cart checkouts (e.g., separating taxable physical goods from non-taxable digital goods) invisibly.

  ### Zero Trust & Security
  - Multi-tenant isolation is guaranteed via strict Row-Level Security (RLS) on `CAPACITY_NODE`.
  - Service-to-service calls to allocate capacity require SPIFFE/SPIRE workload identity validation.

  ## Implementation Prompt
  Implement the `CapacityMeshService` to unify physical, digital, and time-based inventory into a single allocatable resource model. The service must expose endpoints for allocating units, placing ephemeral distributed locks during user checkout, and committing final deductions.

  **Acceptance Criteria:**
  - Support mixed-cart reservations (e.g., lock 1 physical item and 1 time slot simultaneously).
  - Ensure 100% mobile-first API compatibility with payload sizes under 50kb to support offline/low-bandwidth operations.
  - Achieve sub-50ms p99 latency for lock acquisitions to prevent race conditions during high-traffic drops.
  - Enforce strict Zero-Trust boundaries so tenant isolation is mathematically guaranteed at the database level.
  - Do not prescribe specific DB schemas; utilize the existing OHC hybrid architecture and multi-tenant scaling strategies.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
