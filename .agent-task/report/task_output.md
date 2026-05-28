issue_title: "[architecture]_autonomous_dispute_and_chargeback_defense_engine"
issue_description: |
  # Title: Autonomous Dispute & Chargeback Defense Engine

  ## Problem Statement
  Small business owners like Maya (baker) and Priya (boutique owner) operate on thin margins. When a customer files a fraudulent chargeback or dispute (e.g., "item not received" or "unauthorized transaction"), the merchant is burdened with a stressful, manual, and time-sensitive process to prove the transaction was legitimate. They must scramble to collect evidence—chat logs, delivery photos, signed invoices, and tracking numbers—across disconnected systems to submit to Stripe or PayPal. Many SMBs lose these disputes simply because they lack the time or organizational capacity to respond properly, resulting in lost revenue and penalty fees. They need an invisible, proactive AI defense system that automatically intercepts disputes, compiles a comprehensive evidence packet from the unified business memory, and submits the defense on their behalf.

  ## Research Report
  - **Current Architecture Limits:** OHC captures vast amounts of conversational context (Omnichannel Unified Inbox), operational data (Inventory Ledger), and fulfillment details (Shipping/Local Pickup), but this data is not automatically synthesized into a structured legal defense format for payment gateways.
  - **Competitor Analysis:**
    - *Shopify/Wix:* Offer basic dispute dashboards that notify the merchant, but the merchant must still manually gather and upload screenshots and tracking links.
    - *Stripe Chargeback Protection:* Charges a premium percentage fee on all transactions, which cuts into already thin SMB margins.
    - *Chargehound/Midigator:* Enterprise-focused APIs that are too complex and expensive for a non-technical SMB to integrate.
  - **Discovery:** OHC has a unique advantage: it owns the entire customer journey (chat logs, order history, inventory, tap-to-pay location, shipping labels). We can deploy an Autonomous Legal/Finance Agent that listens for dispute webhook events from payment providers, automatically queries the OHC unified database for all related transaction evidence, generates a highly persuasive, standardized rebuttal document, and submits it via API—all without the business owner lifting a finger.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      PAYMENT-GATEWAY ||--o{ OHC-WEBHOOK-INGRESS : "Fires Dispute Event"
      OHC-WEBHOOK-INGRESS ||--o{ AI-FINANCE-DEPT : "Routes to Defense Agent"
      AI-FINANCE-DEPT ||--o{ UNIFIED-INBOX : "Extracts Chat/Agreement Logs"
      AI-FINANCE-DEPT ||--o{ INVENTORY-LEDGER : "Extracts Fulfillment Status"
      AI-FINANCE-DEPT ||--o{ SHIPPING-ENGINE : "Extracts Tracking/Proof of Delivery"

      AI-FINANCE-DEPT ||--o{ EVIDENCE-PACKET : "Compiles"
      EVIDENCE-PACKET ||--o{ PAYMENT-GATEWAY : "Submits via API"

      AI-FINANCE-DEPT ||--o{ ACTION-FEED : "Notifies Merchant of Victory/Pending Status"
  ```

  ### UI Wireframes / Screen Flow (375px Viewport)
  1. **Dispute Alert Card (Inbox):** A high-priority card appears in the unified inbox. "Dispute Filed: $120 order by John Doe. Don't worry, your AI Teammate is handling it."
  2. **Evidence Compilation View (Translucent Glass):** Tapping the card shows a real-time progress indicator:
     - [x] Retrieving Instagram DM agreement
     - [x] Fetching USPS Delivery Photo
     - [x] Drafting legal response
  3. **Auto-Submit & Success:** The card transitions to a "Defense Submitted" state with a button to "View Submitted Packet". A subsequent notification arrives days later: "Dispute Won. $120 released back to your account."

  ### Mobile UX Flow
  - **Action:** A malicious customer files a chargeback via their bank.
  - **Behind the Scenes:** Stripe sends a webhook to OHC. The AI Finance Agent instantly wakes up, gathers Maya's IG chat where the customer said "The cake is perfect!", grabs the delivery photo, formats it into Stripe's required evidence schema, and submits it.
  - **Resolution:** Maya checks her app during a break and sees a notification that a dispute was handled and won automatically. Zero friction, zero panic.

  ### AI Agent Integration Points
  - **Finance & Legal Department (Agent):** Triggered by incoming dispute webhooks. Uses specific tools to query cross-domain data (`get_chat_history`, `get_order_details`, `get_shipping_proof`).
  - **LLM Synthesis:** The agent uses an LLM to draft a professional, concise explanation for the bank, referencing the extracted evidence and matching the specific dispute reason code (e.g., "Product Not Acceptable" vs "Fraudulent").

  ### Key Design Decisions
  - **Zero Merchant Effort:** The default behavior is 100% autonomous submission based on OHC's internal data advantage. Merchants can toggle "Require Manual Review Before Submit" in Advanced Settings.
  - **Immutable Evidence:** Chat logs and delivery proofs must be cryptographically hashed or stored immutably to ensure banks accept them as valid evidence.
  - **Zero Trust Multi-Tenancy:** The defense agent is strictly scoped via SPIFFE/SPIRE identity to only query evidence belonging to the specific tenant (`organization_id`) associated with the disputed charge.

  ## Implementation Prompt
  **To Implementer:** Implement the "Autonomous Dispute Defense Engine".
  Create a webhook ingress service to receive dispute events from payment gateways (e.g., Stripe). When an event is received, trigger an internal background job that spawns an AI Agent from the Finance/Legal Department. Equip this agent with tools to fetch the related order, associated customer communications from the unified inbox, and fulfillment/shipping status. The agent must synthesize this data into a formatted evidence packet (JSON or PDF) and post it back to the payment gateway's dispute evidence API. Ensure all data access respects strict multi-tenant isolation by verifying `organization_id`. Surface the status of this process as simple, non-technical cards in the mobile user's Activity Feed. Do not expose API complexity to the user.

  **Acceptance Criteria:**
  - Webhook ingestion securely authenticates and parses a simulated dispute event.
  - The AI Agent successfully retrieves related cross-domain data (chat, order, fulfillment) for the correct tenant.
  - The Agent generates a coherent, structured evidence response suitable for a bank review.
  - The mobile UI displays the dispute status in a clear, reassuring manner.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []