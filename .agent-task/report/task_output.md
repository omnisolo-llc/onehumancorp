issue_title: "Implement Multi-Tenant Mobile-First Agentic Point-of-Sale (POS) & Tap-to-Pay Integration"
issue_description: |
  # Research Report: Agentic Point-of-Sale & In-Person Commerce

  ## 1. Problem Statement
  SMB operators like Priya (Boutique Operator) and Fatima (Food Cart Operator) must manage in-person operations simultaneously with digital channels. Today, in-person Point-of-Sale (POS) systems are entirely decoupled from e-commerce backends and AI assistants. An operator taking an in-person payment via Stripe Terminal or Tap-to-Pay must manually reconcile inventory, capture customer details, and switch between separate dashboards. There is no AI agent capable of bridging offline physical transactions with online customer retention campaigns or inventory sync.

  ## 2. Research Report
  - **Market Context**: Square currently dominates simple POS. Shopify POS is robust but requires an expensive tier and significant hardware setup for tap-to-pay. Neither provides a proactive AI agent that operates at the point of sale.
  - **The OHC Opportunity**: Integrating Stripe Terminal (and Tap-to-Pay on iPhone/Android) directly into the OHC Flutter app. This turns the owner's phone into the POS terminal. When a transaction happens, the unified AI backend (Sales & Operations Agents) can instantly deduct inventory, create a customer profile (if they provide an email/phone for a receipt), and queue follow-up campaigns.
  - **Competitor Gaps**:
    - *Square*: Excellent hardware, but isolated from advanced conversational AI and broader digital workflows.
    - *Shopify POS*: Expensive, high barrier to entry for micro-merchants like Fatima.
    - *Wix/Squarespace*: Their in-person POS solutions are bolted on and non-native to the mobile management app experience.

  ## 3. Design Doc
  ### Data Model (PostgreSQL)
  - `TerminalSession`: Represents an active Tap-to-Pay session on a specific device.
  - `OfflineTransaction`: Represents an in-person payment, linked to `Customer` (optional), `Order`, and `Tenant`.
  - `DeviceRegistry`: Tracks authorized mobile devices for tap-to-pay.

  ### AI Integration
  - **Operations Agent**: Automatically detects low inventory spikes driven by rapid in-person sales and alerts the owner or drafts a supplier reorder email.
  - **Sales Agent**: If a new customer provides their email for a receipt at the POS, the Sales agent drafts a personalized "Welcome to the shop" email with an online discount code to bridge the offline-to-online gap.

  ### Mobile UX Flow (375px)
  1. **POS Dashboard (Owner View)**: A clean, high-contrast, large-button interface on the Flutter mobile app showing quick-add products or manual amount entry.
  2. **Tap-to-Pay Flow**: The owner taps "Charge", and the app natively invokes Stripe Tap-to-Pay (or Terminal SDK). The screen shows the NFC prompt.
  3. **Receipt & Capture**: Post-payment, a single-screen prompt asks for the customer's email/phone to send a receipt, instantly feeding the Sales Agent.

  ## 4. Implementation Prompt
  **Feature Name**: OHC Native Agentic POS & Tap-to-Pay
  **Target Persona**: Priya (Boutique Operator)
  **Outcome**: Priya can take an in-person payment by letting the customer tap their card on her phone. The inventory is automatically synced with her online store, and the OHC Sales Agent automatically emails the customer their receipt and a 10% discount for their next online purchase.

  **Next Actions**:
  1. Extend the data model to support `OfflineTransaction` and `TerminalSession` with strict Row-Level Security for multi-tenancy.
  2. Implement the backend API endpoints to generate Stripe Terminal connection tokens for the mobile client.
  3. Build the Flutter POS UI (mobile-first, 375px target, high-contrast touch targets >44px) that integrates the Stripe Tap-to-Pay SDK.
  4. Wire the transaction webhook/event to the AI Job Queue so the Operations Agent can adjust inventory and the Sales Agent can process receipt follow-ups.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
