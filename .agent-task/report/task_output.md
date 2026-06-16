issue_title: "[Architecture] Edge-Cached Dynamic Storefronts & POS Offline Sync Engine"
issue_description: |
  # Research Report: Edge-Cached Dynamic Storefronts & POS Offline Sync Engine

  ## Problem Statement
  Small business owners like Priya (Boutique Operator) and Fatima (Food Cart Operator) need a reliable and fast Point-of-Sale (POS) and storefront system that works even in poor network conditions (e.g., crowded festivals, basements, or areas with spotty cell service). Currently, traditional systems fail or block transactions when offline, leading to lost sales and frustrated customers. Furthermore, online storefronts need to load instantly globally to capture fleeting mobile attention, but generating dynamic, personalized content often introduces unacceptable latency. They need a system that caches storefronts at the edge for instant loading, while allowing POS devices to operate offline and sync state (inventory, transactions) deterministically when connectivity is restored, all without the owner needing to understand "sync conflicts" or "edge caching".

  ## Research Report
  - **Competitor Analysis:**
    - **Square / Stripe Terminal:** Offer some offline mode capabilities, but often restrict high-risk transactions or require complex reconciliation if inventory conflicts occur.
    - **Shopify POS:** Has offline capabilities, but the sync process can sometimes result in inventory discrepancies that require manual intervention from the merchant.
    - **Vercel / Cloudflare (Edge Caching):** Industry standard for fast, globally distributed static content, but challenging to integrate with highly dynamic, personalized AI-driven storefronts.
  - **Gaps Identified:**
    - OHC needs a robust, deterministic offline sync protocol (e.g., CRDTs) for the POS mobile app to handle transactions and inventory decrements while completely offline.
    - OHC needs an architecture to aggressively edge-cache dynamic storefronts (Next.js/Vercel style) while using stale-while-revalidate patterns to keep inventory and AI-generated offers fresh.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Edge Network (CDN/Cloudflare)
          EdgeCache[Edge Cache]
          EdgeWorker[Edge Worker - Stale-While-Revalidate]
      end

      subgraph Mobile POS App (375px)
          POSUI[POS Interface]
          LocalDB[(Local CRDT Store)]
          SyncEngine[Offline Sync Engine]
      end

      subgraph OHC Cloud Platform
          APIGateway[API Gateway]
          SyncService[Conflict Resolution Service]
          MainDB[(Cloud Postgres Ledger)]
          Redis[Redis Distributed Locks]
          Agents[AI Agent Swarm]
      end

      CustomerBrowser --> EdgeCache
      EdgeCache --> EdgeWorker
      EdgeWorker -- Background Fetch --> APIGateway

      POSUI <--> LocalDB
      LocalDB <--> SyncEngine
      SyncEngine -- "Intermittent Connection" --> APIGateway

      APIGateway --> SyncService
      SyncService --> MainDB
      SyncService <--> Redis

      SyncService --> Agents
      Agents --> OpsAgent[Ops Agent: Handle Inventory Conflicts]
  ```

  ### Mobile UX Flow (375px First)
  1. **Offline Mode Indicator:** When the POS app loses connection, a subtle, reassuring "Offline Mode - Ready to Transact" badge appears.
  2. **Transacting Offline:** Priya rings up a customer. The transaction is recorded in the `Local CRDT Store`, and the local inventory count is optimistically decremented.
  3. **Background Sync:** Once a connection is re-established, the `Offline Sync Engine` pushes the CRDT deltas to the `Conflict Resolution Service` via the API Gateway.
  4. **Conflict Resolution:** If an online customer bought the last item while the POS was offline, the `Ops Agent` detects the conflict, prioritizes the in-person POS transaction, automatically issues a refund/backorder for the online customer, and drafts an apology email for Priya to approve.

  ### AI Agent Integration Points
  - **Operations Agent:** Monitors the sync process. If inventory conflicts arise between edge-cached online sales and offline POS sales, it automatically resolves them based on business rules (e.g., prioritize in-person) and queues necessary actions (refunds, supplier reorders).

  ### Key Design Decisions
  - **CRDTs for Local State:** Use Conflict-Free Replicated Data Types for the local mobile database to ensure deterministic, mathematically provable eventual consistency without complex manual reconciliation screens for the user.
  - **Stale-While-Revalidate:** The online storefront uses standard SWR caching. If inventory is slightly stale at the edge, the backend handles the race condition during checkout using Redis Redlock.

  ## Implementation Prompt
  Implement the Edge-Cached Dynamic Storefront & POS Offline Sync Engine.
  - **User-Facing Outcome:** The POS app continues to function flawlessly without internet. Storefronts load instantly globally.
  - **CUJ:** Priya loses internet at a pop-up shop. She processes 5 transactions. Her phone reconnects. The transactions seamlessly sync to the backend. The Ops Agent resolves an inventory conflict automatically and notifies her.
  - **Acceptance Criteria:**
    - Implement a local CRDT-based storage engine in the Flutter POS app.
    - Build the backend `Conflict Resolution Service` to ingest and merge CRDT deltas.
    - Implement Redis Redlock for inventory locking during online checkout.
    - Ensure the Ops Agent can detect and automatically propose resolutions for inventory sync conflicts.
    - All POS UI elements must be fully functional and visually clear on a 375px display in both online and offline states.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
