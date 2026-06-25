issue_title: "[Architecture] Mobile-First Tap-to-Pay POS & Optimistic Mutation Engine"
issue_description: |
  # Mobile-First Tap-to-Pay POS & Optimistic Mutation Engine

  ## Problem Statement
  Small business owners like Priya (boutique operator) and Fatima (food cart operator) often operate in environments with spotty internet connectivity (e.g., crowded markets, basements, moving food trucks). Legacy Point-of-Sale (POS) systems either require expensive proprietary hardware (Square) or fail completely when offline (Shopify POS web fallbacks). Furthermore, these owners need to capture payments directly on their primary device (their Android or iOS smartphone) without carrying extra dongles, and they need the UI to react instantly even if the network is dropping packets. If a transaction or inventory update hangs on a spinner, the line stops, and the owner loses money.

  ## Research Report
  - **The Market Gap**: Current SMB platforms treat offline support as an afterthought. Stripe Terminal SDK supports Tap-to-Pay on iPhone/Android, but it requires deep integration into a native app shell. Web-only PWAs cannot access the NFC secure element required for Apple/Google Tap-to-Pay.
  - **Competitor Analysis**:
    - *Square*: Excellent POS, but locked to their ecosystem and often requires their hardware for the best experience.
    - *Shopify POS*: Powerful, but the mobile app can be clunky for pure mobile-first operators, and offline mode is limited in functionality (cannot process new cards offline).
    - *Link-in-Bio tools (Stan, Linktree)*: Purely online. Useless for in-person transactions.
  - **The OHC Opportunity**: Deliver a Flutter-based native shell that embeds the Stripe Terminal SDK for Tap-to-Pay, combined with a local SQLite-backed optimistic mutation engine. This allows Fatima to tap a customer's card, instantly see a "Payment Captured" screen, and move to the next customer, while the OHC Agent handles the background sync and retry logic when she gets back to 4G/5G coverage.

  ## Design Doc
  ### Architecture
  1.  **Flutter Native Shell**: The OHC mobile app must be a Flutter application (not just a PWA) to access native NFC/Secure Element APIs via platform channels.
  2.  **Stripe Terminal Integration**: Utilize the `stripe_terminal` Flutter package. The backend provides connection tokens; the mobile app handles the physical tap.
  3.  **Optimistic Mutation Engine (Local-First)**:
      - Integrate a local SQLite database (via `sqflite` or PowerSync) as the primary data source for the UI.
      - When an order is placed or inventory is updated, write to the local DB *first*. The UI updates instantly.
      - A background Sync Daemon (the "Operations Agent" locally) queues the mutation and attempts to sync with the Go/Bazel backend.
  4.  **Multi-Tenant Backend API**: The Go backend receives the synced mutations, validates them, and updates the authoritative PostgreSQL database.

  ### Mobile UX Flow (375px)
  1.  **Home**: Fatima opens the app. The "Work Triage" feed is visible, but a prominent, persistent "New Sale" FAB (Floating Action Button) is always accessible.
  2.  **Cart Builder**: Tapping the FAB opens a fast, visual cart builder. Large tap targets for menu items.
  3.  **Checkout**: Tapping "Charge $15.00" immediately initiates the Tap-to-Pay overlay (native OS UI).
  4.  **Success**: Customer taps card. OS returns success. OHC instantly shows a green checkmark and returns to the empty cart builder. The sync happens invisibly in the background.

  ### AI Agent Integration
  - **Operations Agent (Sync Manager)**: Monitors the local queue. If a sync fails repeatedly, it alerts the owner in the Work Triage feed ("3 transactions pending sync - move to better coverage").
  - **Finance Agent**: Automatically reconciles the Tap-to-Pay batch settlements at the end of the day and provides a plain-language summary to Priya.

  ## Implementation Prompt
  Implement the core architectural foundations for the Mobile-First Tap-to-Pay POS and Optimistic Mutation Engine.
  1.  **Backend (Go)**: Create the necessary API endpoints (`/api/v1/terminal/connection_token`, `/api/v1/pos/sync`) to support the Stripe Terminal connection and the offline sync queue. Ensure these are properly isolated by `tenant_id`.
  2.  **Frontend (Flutter)**: Design the core SQLite schema for the offline queue and implement the optimistic UI pattern for adding an item to a cart and "completing" an order (simulated network delay). The UI must update instantly without waiting for the backend response. (Note: Full Stripe Terminal SDK implementation may require native build steps, so focus on the architecture and optimistic UI flow first).
  3.  **Verification**: Write Playwright E2E tests simulating a spotty network connection to verify that an order can be created, the UI updates, and the sync happens eventually.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
