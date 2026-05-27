issue_title: "Invisible AI Chargeback Defense & Fraud Shield Engine"
issue_description: |
  # [architecture] Invisible AI Chargeback Defense & Fraud Shield Engine

  ## 1. Title
  Invisible AI Chargeback Defense & Fraud Shield Engine

  ## 2. Problem Statement
  **For the non-technical small business owner (like Maya the baker or Carlos the handyman):**
  "Friendly fraud" and chargebacks on deposits represent a massive vulnerability for small businesses. Currently, defending a chargeback requires manually hunting down contracts, SMS/IG DM logs, booking calendars, and delivery photos to upload to complex, intimidating payment processor portals (like Stripe or PayPal). The process is time-consuming, confusing, and often results in lost revenue simply because the owner didn't submit the right "compelling evidence" in time. Our proposed AI engine will invisibly compile these assets in the background and auto-submit bank-ready dispute packets the moment a chargeback is initiated, requiring zero manual work from the business owner.

  ## 3. Research Report
  ### Current Landscape & Gap Analysis
  *   **Stripe Chargeback Protection:** Requires opt-in and charges an additional fee per transaction. It primarily protects against fraudulent disputes but still requires manual intervention and evidence compilation for other types of disputes.
  *   **Shopify Protect:** Offers protection on eligible Shop Pay transactions, but doesn't extend to all payment methods or custom invoicing scenarios typical of service-based businesses (like Carlos the handyman).
  *   **Square / PayPal:** Both offer dispute resolution centers, but the burden of proof rests heavily on the merchant to gather, format, and upload evidence within a strict timeframe.

  ### The OneHumanCorp Opportunity
  OneHumanCorp has a unique advantage: we are the central nervous system for the business. Because we handle the initial inquiry (via Omnichannel AI Inbox), the quote, the contract (via Invisible Contract Engine), the booking/deposit, and the final fulfillment/communication, we already possess all the necessary "compelling evidence" for a chargeback defense. We can shift this from a reactive, manual burden to a proactive, invisible, AI-driven defense.

  ## 4. Design Doc
  ### 4.1 Architecture Diagram

  ```mermaid
  graph TD
      A[Payment Gateway Webhook: Chargeback Initiated] --> B(Fraud Shield Orchestrator)
      B --> C{Context Gathering}
      C --> D[Omnichannel Inbox: Fetch Comms/DMs/SMS]
      C --> E[Contract Engine: Fetch Signed Agreement & TOS]
      C --> F[Ledger/Booking: Fetch Deposit/Fulfillment Status]
      D --> G(Dispute Packet Compiler AI)
      E --> G
      F --> G
      G --> H[Bank-Ready Dispute Packet Generated]
      H --> I[Payment Gateway API: Auto-Submit Evidence]
      I --> J[Business Owner Notification: 'We fought a chargeback for you']
  ```

  ### 4.2 Mobile UX Flow (375px First)
  The core design philosophy is invisibility. The user should rarely interact with this system unless viewing a success notification.

  **Screen 1: The 'Peace of Mind' Notification (Push/In-App Card)**
  *   **Design Style:** macOS Translucent Glass, Ubiquiti UniFi modular card.
  *   **Content:** "🛡️ Chargeback Defended: A customer disputed a $150 deposit for 'Custom Cake'. We automatically submitted the signed contract and Instagram DM logs confirming delivery. You'll hear back in 5-7 days. No action needed."
  *   **Action:** [View Dispute Details] (Secondary button)

  **Screen 2: Dispute Details (If clicked)**
  *   **Layout:** Clean, stacked cards.
  *   **Header:** Dispute Status (e.g., "Pending Bank Review").
  *   **Evidence Submitted Card:** A clean list of what the AI compiled (e.g., "Signed Terms of Service", "Customer IP Address at Checkout", "Delivery Photo").
  *   **Advanced Settings (Hidden behind toggle):** Access to raw JSON payload or specific processor dispute IDs.

  ### 4.3 AI Agent Integration Points
  *   **Finance/Ops AI Department:** Acts as the primary orchestrator. Listens for dispute webhooks, triggers the compilation process, and interfaces with the payment gateway APIs.
  *   **CS/Comms AI Department:** Provides the context gathering agents with relevant communication logs (DMs, emails) mapped to the specific transaction ID.
  *   **Legal AI Department:** Ensures the compiled evidence meets the specific "compelling evidence" criteria required by the card networks (Visa, Mastercard) for the specific dispute reason code.

  ### 4.4 Key Design Decisions
  *   **Proactive Compilation:** Evidence is continuously linked to the transaction record during the lifecycle of the order, rather than retroactively scraped when a dispute occurs.
  *   **Zero-Touch Submission:** The system defaults to auto-submitting the strongest possible case without requiring the owner's review, maximizing the chance of meeting strict processor deadlines.
  *   **Multi-Tenant Isolation:** Dispute evidence compilation happens strictly within the tenant's bounded context to ensure no cross-contamination of customer data.

  ## 5. Implementation Prompt
  **For the Implementer Agent:**
  Implement the core logic for the 'Fraud Shield Orchestrator'.
  *   **CUJ (Critical User Journey):** A webhook is received from a payment provider indicating a dispute (e.g., `charge.dispute.created`). The orchestrator must automatically query the internal systems (Inbox, Contracts, Ledger) using the associated `transaction_id`, compile a structured 'Evidence Packet', and submit it back to the provider's API. Finally, it should trigger a simple notification to the business owner stating the dispute is being handled.
  *   **Acceptance Criteria:**
      *   System can ingest standard dispute webhooks.
      *   System successfully aggregates linked data (communications, contracts, fulfillment status) based on transaction ID.
      *   System formats the aggregated data into a standardized dispute evidence format suitable for processor APIs.
      *   System operates completely invisibly to the user until the notification stage.
  *   **Note:** Do not prescribe specific database schemas or lower-level API function signatures. Focus on the orchestration logic and the integration between the payment gateway listener and the internal context gatherers.

  ## 6. Priority & Scope
  *   **Priority:** P1 (High - directly impacts user revenue and trust)
  *   **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []