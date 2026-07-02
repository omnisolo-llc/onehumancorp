issue_title: "Architectural Gap: Unified Multi-Channel Tap-to-Pay & Zero-Click Generation"
issue_description: |
  # Unified Multi-Channel Tap-to-Pay & Zero-Click Generation Architecture

  ## Problem Statement
  Small business owners such as Priya (boutique operator) and Carlos (field service owner) suffer from fragmented operations. They struggle to synchronize in-person Tap-to-Pay transactions with their online inventory, and they experience significant friction when setting up digital storefronts. Current platforms (Shopify, Wix) demand manual, desktop-first configuration and rely heavily on disparate plugins to bridge online and offline sales. This disconnect results in out-of-sync inventory, double bookings, and a steep learning curve that alienates non-technical owners.

  ## Research Report
  **Market & Competitor Analysis:**
  - **Shopify:** Offers strong POS and Tap-to-Pay capabilities but requires a complex, plugin-heavy setup that is overwhelming for micro-SMBs. The onboarding process is desktop-centric and time-consuming.
  - **Wix & Squarespace:** Provide easier drag-and-drop interfaces but lack the deep, native integration of in-person payments with real-time inventory locking across online and offline channels.
  - **Square:** Excels in POS hardware but lacks comprehensive, agent-driven workflow automation to unify the business seamlessly.
  - **OHC Differentiation:** OHC must deliver an "Invisible AI Automation" experience. This involves a **Zero-Click Generation Workflow** where an owner can set up their entire digital and physical storefront via a single conversational prompt on their phone. Furthermore, it requires a robust, centralized inventory system that supports instant offline-capable Tap-to-Pay via Stripe Terminal, perfectly synchronized with the online catalog.

  ## Design Doc

  ### High-Level Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile App - 375px POS Mode] -->|Stripe Terminal SDK| B(Tap-to-Pay Transaction)
      A -->|Conversational Prompt| C(Zero-Click Generator Agent)
      B --> D{Inventory Locking Service}
      C --> E[PostgreSQL Central Ledger]
      D -->|Redis Redlock| F[Inventory Cache]
      D --> E
      F -->|Sync| E
      G[Operations Agent] -->|Monitor| E
      G -->|Push Notification| H[Owner Feed]
      I[Finance Agent] -->|Reconcile| B
      I --> E
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Zero-Click Onboarding:**
    - **Screen 1:** A simple conversational interface. Maya types, "I sell custom cakes in Austin."
    - **Screen 2:** The AI generates the storefront, DB schema, and product catalog in the background. A loading spinner with translucent glass styling is displayed.
    - **Screen 3:** The fully configured storefront is presented for review with a massive "Approve & Launch" button.
  - **Tap-to-Pay Checkout Flow:**
    - **Screen 1:** POS Mode. Large, touch-friendly product cards (≥ 44x44px targets).
    - **Screen 2:** "Tap to Pay" screen using native device NFC capabilities (Stripe Terminal integration).
    - **Screen 3:** Instant inventory deduction. If the network is flaky, the app logs the transaction locally (Offline-Capable UX) and syncs to the Central Ledger once reconnected.

  ### AI Agent Integration Points
  - **Zero-Click Generator Agent:** Interprets initial owner prompts to autonomously provision the database schema, store layout, and basic inventory without manual forms.
  - **Operations Agent ("The Manager"):** Actively monitors real-time stock levels. If an in-store Tap-to-Pay transaction depletes stock, it immediately updates the online storefront and alerts the owner if a restock is needed.
  - **Finance Agent ("The Accountant"):** Processes the Stripe Terminal transactions, handles splits, and correlates offline POS data with online sales for unified reporting.

  ### Key Design Decisions
  - **Mobile-First POS:** The entire Tap-to-Pay and inventory management interface is strictly designed for a 375px viewport to accommodate operators like Carlos in the field.
  - **Distributed Locks:** Implementation of Redis Redlock (`ohc:lock:{tenant_id}:inventory:{product_id}`) to prevent double-booking during concurrent online browsing and offline purchases.
  - **Offline Resilience:** Eventual consistency model for the POS client. It caches catalog data locally and queues finalized offline sales for asynchronous reconciliation when connectivity returns.

  ## Implementation Prompt
  **User-Facing Outcome:**
  As Priya, I can open the OHC app on my iPhone, process an in-store Tap-to-Pay transaction for the last "Red Dress", and trust that the system will instantly and invisibly mark it "Sold Out" online. If I am setting up a new location, I can simply type a sentence, and the AI will generate the entire digital setup.

  **CUJ & Acceptance Criteria:**
  1. Build a responsive, 375px-optimized POS UI featuring premium translucent glass styling and large touch targets.
  2. Integrate the Stripe Terminal SDK for Tap-to-Pay functionality within the mobile client.
  3. Implement the Redis Redlock mechanism on the backend to reserve inventory instantly upon a Tap-to-Pay action.
  4. Develop an offline-sync queue in the mobile client that locally caches finalized transactions during network drops and syncs them to PostgreSQL upon reconnection.
  5. Coordinate the Operations Agent to trigger a push notification to the owner's feed when an item sells out via POS.
  6. Ensure all new logic is covered by Playwright E2E tests simulating both online and offline POS transactions.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []