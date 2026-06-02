issue_title: "[architecture] Conversational Checkout & Instant Deposit Engine"
issue_description: |
  # Research Report: Conversational Checkout & Instant Deposit Engine

  ## Architectural Gap Discovery
  Through deep codebase research (`docs/research/`, `docs/technical/`), it is evident that OHC has established robust primitives: the Omnichannel Unified Inbox, the Universal Capacity Ledger, and Localized Invoicing. However, a critical architectural gap exists for our primary personas (Maya the Baker, Carlos the Handyman).
  These users conduct 80% of their business via social media DMs. Currently, there is no system that allows the AI Sales Agent to autonomously bridge a conversational intent (e.g., "I want to book Tuesday") directly into a zero-click, secure checkout session that instantly locks inventory and processes deposits (via Stripe or Mercado Pago).

  ## System Design Deep Dive
  **Business Journey:** Customer requests a booking in IG/WhatsApp -> AI Sales Agent detects intent -> AI requests a 15-minute soft lock from the Capacity Mesh -> AI generates a Conversational Checkout Session -> Customer receives a dynamic interactive card in DM -> Customer pays deposit via Apple Pay/Pix -> Webhook commits the inventory lock -> Operations Agent notifies Maya.

  **Data Model:** Introduce `ConversationalCheckoutSession` holding state, tenant_id, customer_id, deposit amount, and a TTL-based `inventory_lock_id`.

  **AI Department Coordination:**
  - *Sales & Acquisition:* Generates the session.
  - *Operations:* Grants soft-locks on inventory/calendar.
  - *Finance & Payments:* Reconciles the webhook and updates the Ledger.

  ## Technical Integrity & Mobile-First
  - The checkout UI must render as a half-sheet modal on a 375px viewport, requiring zero keyboard entry (utilizing native OS payment sheets or Pix QR codes).
  - Strict tenant isolation using SPIFFE SVIDs for webhook processing.

  ## Proposed Action
  Implement the Conversational Checkout engine to allow AI agents to generate instant deposit links within DM threads, bridging the gap between chat and localized payment processing.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
