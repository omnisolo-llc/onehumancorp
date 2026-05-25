issue_title: "[Architecture] Autonomous Chargeback & Fraud Defense Engine"
issue_description: |
  ## Title
  [Architecture] Autonomous Chargeback & Fraud Defense Engine

  ## Problem Statement
  Small business owners—especially those selling physical goods online like Maya (the baker) or Priya (the boutique owner)—face an asymmetric threat from "friendly fraud" and chargebacks. When a customer falsely claims an item was not delivered or not as described, the burden of proof falls entirely on the merchant. Gathering evidence (tracking numbers, communication logs, IP addresses) and compiling it into a format that banks accept is incredibly stressful, manual, and time-consuming. Most SMBs lose these disputes by default because they miss the tight response windows or submit incomplete evidence, directly impacting their bottom line and threatening their merchant accounts. They need an invisible defense system that proactively gathers evidence during every transaction and automatically contests fraudulent chargebacks on their behalf.

  ## Research Report

  ### Competitive Analysis
  | Platform | Chargeback Handling | Key Constraint |
  |---|---|---|
  | Shopify | Shopify Protect (limited to Shop Pay), basic manual evidence submission form. | Relies heavily on the merchant to notice the dispute, gather the specific required evidence, and manually submit it before the deadline. |
  | Stripe | Stripe Radar (fraud prevention), manual dispute resolution dashboard. | Highly developer-centric dashboard; requires understanding of dispute categories and evidence requirements. |
  | Wix | Basic notification of dispute, links to payment provider. | No native evidence compilation or automated defense mechanism. |
  | **OHC (Target)** | **Autonomous, Zero-Touch Defense.** | **Must instantly detect disputes, autonomously compile evidence from all internal mesh services, and submit the response without merchant intervention (unless explicitly requested).** |

  ### Industry Findings
  - **Win Rates:** Merchants who submit comprehensive, correctly categorized evidence win up to 70% of chargebacks. Those who submit partial or poorly formatted evidence win less than 20%.
  - **Friendly Fraud:** Accounts for nearly 75% of all chargebacks. It is often combatted effectively with strong, centralized evidence (e.g., AVS match, CVV match, signed delivery receipts, IP location matches, and customer communication history).
  - **Time Sensitivity:** Dispute windows are strict (often 7-15 days). Missed deadlines result in automatic forfeiture.

  ### The Architectural Gap
  OHC currently lacks a unified, automated system to ingest dispute webhooks from payment gateways (Stripe, PayPal, Adyen) and immediately cross-reference that transaction against the `Customer360` interaction timeline, the shipping ledger, and the unified communication inbox. Without this, OHC merchants are vulnerable to the same manual, stressful process as users on legacy platforms.

  ## Design Doc

  ### Business Journey Mapping (The Maya Persona)
  1.  **Transaction:** A customer purchases a $150 custom vegan cake from Maya's OHC storefront.
  2.  **Delivery:** The cake is delivered via a local courier; the courier marks it delivered with a photo and GPS pin (logged in OHC's shipping mesh).
  3.  **Dispute:** Three weeks later, the customer files a chargeback claiming "Item Not Received."
  4.  **Autonomous Action:**
      - The `Chargeback Defense Engine` receives the webhook from the payment processor.
      - It queries the `Customer360` profile, extracting the original order details, AVS/CVV match status, IP address at checkout, the courier's delivery photo/GPS pin, and any post-purchase Instagram DMs confirming the cake was delicious.
      - The AI Legal/Finance Agent compiles this into a structured PDF evidence packet tailored to the specific dispute reason code.
      - The Agent submits the evidence directly to the payment processor via API.
  5.  **Notification:** Maya receives a simple push notification: "A $150 dispute was filed for Order #102. Don't worry, we've automatically submitted the delivery photo and customer chat logs as evidence to the bank. You don't need to do anything."

  ### Data Model & Invariants

  ```mermaid
  erDiagram
      MERCHANT_TENANT ||--o{ DISPUTE_CASE : "owns"
      DISPUTE_CASE ||--|| TRANSACTION : "references"
      DISPUTE_CASE ||--o{ EVIDENCE_ARTIFACT : "contains"
      TRANSACTION ||--|| SHIPPING_EVENT : "has"
      TRANSACTION ||--o{ COMMUNICATION_LOG : "has"

      DISPUTE_CASE {
          string case_id PK
          string tenant_id FK
          string transaction_id FK
          string status "OPEN, WON, LOST, AUTO_SUBMITTED"
          string reason_code
          float disputed_amount
          string currency
          timestamp deadline
      }

      EVIDENCE_ARTIFACT {
          string artifact_id PK
          string case_id FK
          string artifact_type "IP_LOG, DELIVERY_RECEIPT, CHAT_TRANSCRIPT, AVS_MATCH"
          string s3_uri
          float ai_relevance_score
      }
  ```

  ### Multi-Tenant Isolation & Security
  - **Strict Data Silos:** Evidence compilation must strictly execute within the context of the `tenant_id`. The agent compiling the evidence cannot cross tenant boundaries or access global communication logs.
  - **SPIFFE/SPIRE Identity:** The worker process generating the evidence packet must assume a temporary, tightly scoped SPIFFE identity that grants read-only access to the specific transaction, shipping, and communication records for that single tenant.

  ### Mobile-First UX Flow (375px Viewport)
  - **The Dashboard Card:** Under the "Finance" tab, a simple summary card: `Disputes: 1 Auto-Defended (Tap for details)`.
  - **The Detail View:** A clean, timeline-based view.
      - *Top:* Status pill ("Evidence Submitted"), Amount ($150), Reason ("Not Received").
      - *Middle:* A carousel of the compiled evidence (thumbnail of delivery photo, snippet of chat log).
      - *Bottom:* "We handled this for you on [Date]. Expect a resolution from the bank by [Date]."
  - **Grandmother Test:** Maya should not see terms like "Reason Code 10.4", "AVS Mismatch", or "Issuer Network." She only needs to know the money is protected and the platform has handled the administrative work.

  ### AI Department Coordination
  - **The Legal/Finance Department (The Defender):** Subscribes to the `payment.dispute.created` event on the hybrid NATS mesh.
  - **The Customer Success Department:** Is queried by *The Defender* to pull relevant chat logs (WhatsApp, IG DMs) proving the customer was satisfied or acknowledged receipt.
  - **The Operations Department:** Is queried by *The Defender* to pull shipping and fulfillment logs.

  ## Implementation Prompt
  Implement the Autonomous Chargeback & Fraud Defense Engine to protect OHC merchants from friendly fraud without requiring their manual intervention.

  *   **Acceptance Criteria 1 (Ingestion):** Create a secure webhook handler that listens for dispute events from primary payment gateways (e.g., Stripe) and normalizes the dispute data into the `DISPUTE_CASE` schema.
  *   **Acceptance Criteria 2 (Evidence Orchestration):** Implement the internal agent workflow (*The Defender*) that automatically queries the Shipping Mesh, Communication Inbox, and Transaction Ledger to gather relevant `EVIDENCE_ARTIFACT` records based on the specific dispute reason code.
  *   **Acceptance Criteria 3 (Submission):** The engine must format the gathered evidence into the required gateway-specific format (e.g., Stripe Evidence Object) and automatically submit it before the dispute deadline.
  *   **Acceptance Criteria 4 (Mobile UX Visibility):** Create the mobile-first UI components to display the status of the auto-defended dispute in plain language, abstracting away the complex financial terminology.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []