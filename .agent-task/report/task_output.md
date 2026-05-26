issue_title: "[Architecture] Autonomous Embedded Capital & Micro-Advance Engine"
issue_description: |
  # Autonomous Embedded Capital & Micro-Advance Engine

  ## Problem Statement
  Small business owners frequently face cash flow crunches—whether it's buying bulk ingredients for a large upcoming order, repairing essential equipment, or covering rent during a slow season. Traditional banks require mountains of paperwork, take weeks to approve, and often reject micro-loans.
  - **Maya (Baker)** receives a sudden $2,000 corporate catering order but needs $600 upfront for premium ingredients and boxes. She doesn't have the cash on hand.
  - **Fatima (Food Cart)** has her refrigerator break down and needs $800 immediately to fix it, otherwise she loses a week's inventory.
  - **Carlos (Handyman)** needs to buy materials for a big job before the client pays the final invoice.

  They need access to capital *instantly*, without filling out forms, uploading tax returns, or waiting for a human underwriter. The capital should be repaid invisibly as a small percentage of their daily sales, removing the stress of fixed monthly loan payments.

  ## Research Report
  ### Market Context & Competitor Analysis
  Embedded capital has proven to be a highly lucrative and deeply retentive feature for SMB platforms.
  - **Shopify Capital**: Offers cash advances based on store performance. Repaid via a fixed percentage of daily sales. However, it relies heavily on historical e-commerce data and takes days to process initial applications.
  - **Square Capital (Block)**: Extremely successful model. Pre-approved loans surface in the dashboard. Repayments are automated from daily card swipes.
  - **Stripe Capital**: API-first capital that platforms can embed. It uses Stripe processing history to underwrite.
  - **Wix & Squarespace**: Offer limited access to third-party lending partners, often resulting in jarring handoffs to external lenders, requiring new accounts and complex onboarding.

  ### The OHC Gap
  While competitors rely solely on past transaction volume, OHC's unique advantage is our **AI Agents**. Because our Operations and Inbox agents interact directly with the business context (e.g., they know Maya just booked a $2,000 future invoice via Instagram DMs), OHC can underwrite based on *future confirmed bookings and invoices*, not just past sales. We can offer proactive, hyper-contextual micro-advances exactly when the business needs them, embedded directly in the conversational UI.

  ## Design Doc

  ### Mobile-First UX Flow (375px Viewport)
  1. **The "Smart Offer" Card**: While Maya is viewing her upcoming $2,000 order in the OHC app, a unified dashboard card (macOS Translucent Glass style) appears natively below the order details.
     - *UI*: "Need supplies for this order? Get $600 instantly. Repay automatically from your sales."
  2. **One-Tap Review**: Maya taps the card. A clean slide-up modal details the terms:
     - Advance Amount: $600
     - Flat Fee: $45 (No compounding interest)
     - Repayment: 10% of daily sales until $645 is paid.
  3. **Instant Activation**: She taps "Accept with FaceID."
  4. **Immediate Availability**: A celebratory green toast appears: "$600 added to your OHC Wallet." The funds are instantly available on her OHC virtual/physical card via Apple/Google Pay.

  ### Key Design Decisions & Why
  - **Zero-Form Underwriting**: Small business owners don't have time for forms. We continuously underwrite in the background using their real-time OHC ledger, inventory velocity, and AI-inbox confirmed bookings. If they see an offer, they are already 100% approved.
  - **Daily Sales Split Repayment**: Fixed monthly loan payments cause stress. Repaying a percentage of daily sales aligns OHC's success with the merchant's success (if they make no sales today, they pay nothing today).
  - **Proactive Contextual Surfacing**: Instead of burying the "Loans" page in a deep menu, the AI Finance Agent surfaces offers contextually—e.g., right when Carlos drafts a large quote that requires material purchasing.

  ### AI Agent Integration Points
  - **Finance Agent**: Continuously monitors the business's Unified Ledger and Booking Engine. It calculates predictive cash flow gaps and determines maximum safe advance limits.
  - **Operations/CS Agent**: If Maya asks her AI assistant, "Can I get an advance to buy a new mixer?", the conversational agent understands the intent, checks eligibility with the Finance Agent, and renders the "Smart Offer" interactive card right in the chat thread.

  ### Architecture Diagram (Mermaid.js)

  ```mermaid
  erDiagram
      MERCHANT ||--o{ LEDGER_ACCOUNT : owns
      MERCHANT ||--o{ CAPITAL_OFFER : receives
      CAPITAL_OFFER ||--o| CAPITAL_ADVANCE : converts_to
      CAPITAL_ADVANCE ||--o{ REPAYMENT_SPLIT : generates

      CAPITAL_OFFER {
          string offer_id
          string merchant_id
          decimal amount
          decimal flat_fee
          decimal repayment_percentage
          string status
          timestamp expires_at
      }

      CAPITAL_ADVANCE {
          string advance_id
          string offer_id
          decimal total_owed
          decimal total_repaid
          string status
      }
  ```

  ```mermaid
  sequenceDiagram
      participant Merchant as Merchant (Mobile App)
      participant Chat UI as AI Inbox UI
      participant Agent as Finance AI Agent
      participant Risk as Underwriting/Risk Engine
      participant Ledger as Unified Ledger

      loop Continuous Background Process
          Ledger-->>Risk: Streams transaction & booking data
          Risk-->>Agent: Updates pre-approved micro-advance limits
      end

      Merchant->>Chat UI: "I need $400 for a broken fridge."
      Chat UI->>Agent: Intent: Capital Request
      Agent->>Risk: Check current pre-approval limit
      Risk-->>Agent: Approved up to $800
      Agent-->>Chat UI: Renders interactive Capital Offer Card ($400)
      Merchant->>Chat UI: Taps "Accept & Fund" (FaceID)
      Chat UI->>Ledger: Execute Instant Transfer & Setup Daily Split
      Ledger-->>Merchant: Notification: "$400 available now"
  ```

  ### Zero Trust & Security
  - **Multi-Tenant Isolation**: Capital offers, advance limits, and repayment ledgers are strictly partitioned by `tenant_id` at the database level.
  - **Secure Identity**: Accepting an advance requires cryptographic proof of identity (FaceID/Biometrics tied to the mobile device's secure enclave) to prevent unauthorized issuance.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Implement the backend data models, the risk-engine event listeners, and the unified UI components for the Autonomous Embedded Capital Engine.
  1. Build the data schema for handling Capital Offers, Active Advances, and Repayment Splits. Ensure strict multi-tenant constraints are applied.
  2. Implement a background job (or NATS subscriber) that listens to ledger events and booking confirmations to update a merchant's `pre_approved_capital_limit`.
  3. Create the conversational UI card and API endpoints necessary for a merchant to accept a pre-approved offer and instantly see the funds reflect in their OHC Ledger balance.
  4. Ensure the repayment mechanism intercepts incoming payments to deduct the specified percentage before depositing the remainder.

  *Remember: Do not prescribe specific database schemas or internal function signatures—design them to best integrate with our existing Unified Ledger.*
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
