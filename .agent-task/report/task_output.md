issue_title: "Design Autonomous Dispute and Chargeback Resolution Engine"
issue_description: |
  # Autonomous Dispute and Chargeback Resolution Engine

  ## Problem Statement
  Small business owners—whether they are service providers like Carlos (handyman) or sellers of physical goods like Maya (baker)—inevitably face payment disputes and chargebacks. A chargeback is not just a loss of revenue; the process of gathering evidence, communicating with the payment processor, and responding within strict deadlines is highly technical, deeply stressful, and consumes hours of time that should be spent running the business. Traditional platforms (like Stripe or Square) provide tools to upload evidence, but the burden of proof and execution falls entirely on the non-technical merchant. OneHumanCorp (OHC) needs an invisible, automated system that proactively intercepts, manages, and resolves payment disputes on behalf of the merchant, ensuring they never have to think about the complexities of chargeback representment.

  ## Research Report
  **Competitive Analysis:**
  - **Stripe:** Offers Stripe Chargeback Protection (for a fee) and detailed APIs for dispute evidence submission, but requires manual intervention from the merchant to assemble receipts, logs, and communication history unless using expensive third-party tools (like Chargehound).
  - **Shopify:** Provides a dashboard for handling chargebacks, but merchants still manually upload tracking numbers, emails, and order details.
  - **Square:** Similar to Shopify, offering a Risk Manager and dispute dashboard, but requires human review and action.

  **Market Gap:** There is no platform that fully automates the dispute resolution process end-to-end for small businesses. A complete solution must autonomously collect evidence (e.g., chat logs from the Omnichannel AI Inbox, delivery receipts, GPS logs from the Mobile-First Inventory Scanner, signed digital contracts) and submit a heavily structured, compelling representment package to the payment processor without the business owner ever needing to intervene, except to be notified of the outcome.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Payment Processor (Stripe/Adyen)
          Webhook[Dispute Webhook] --> OHC_API[OHC API Gateway];
      end

      subgraph OHC Backend
          OHC_API --> ActionController[Dispute Event Router];
          ActionController --> LegalAgent[AI Legal/Dispute Agent];

          LegalAgent -->|Fetches Order Data| OrderLedger[(Order Ledger DB)];
          LegalAgent -->|Fetches Communication| CommInbox[(Omnichannel Comm Logs)];
          LegalAgent -->|Fetches Tracking| ShippingEngine[(Shipping/Fulfillment Service)];
          LegalAgent -->|Fetches Contracts| ContractDB[(Signed Contracts/Policies)];

          LegalAgent -->|Generates Evidence Package| PackageBuilder[Evidence Document Generator];
          PackageBuilder --> OHC_API_Out[OHC Processor API Client];
      end

      OHC_API_Out -->|Submits Evidence| ProcessorAPI[Payment Processor API];

      subgraph Merchant Notification
          LegalAgent -->|Generates Brief| NotificationEngine[Push/SMS Notification];
          NotificationEngine --> MobileClient[OHC Mobile App];
      end
  ```

  ### Business Journey Mapping
  1. **Trigger:** A customer initiates a chargeback with their bank (e.g., claiming "Product Not Received").
  2. **Ingestion:** The payment processor sends a webhook to OHC alerting of the dispute.
  3. **Autonomous Investigation:** The AI Legal/Dispute Agent is triggered. It queries the order ledger for the transaction, pulls tracking data from the fulfillment service, retrieves chat logs from the unified inbox (e.g., customer confirming address), and pulls the store's refund policy.
  4. **Evidence Generation:** The agent synthesizes this data into a formatted, compelling evidence document optimized for the specific processor's requirements.
  5. **Submission:** The package is submitted automatically to the processor before the deadline.
  6. **Notification:** Maya receives a simple notification on her OHC mobile app: "A $50 chargeback was filed for Order #102. Our AI Dispute Agent automatically gathered the tracking info and submitted the evidence to fight it. You don't need to do anything."

  ### Mobile-First UX Flow (375px)
  - **Zero-Touch Default:** The primary UX is a push notification summarizing the action taken, keeping the merchant informed without requiring action.
  - **Dispute Detail Card:** If tapped, the user sees a clean, UniFi-style card detailing:
    - **Status:** "Fighting Dispute" (with a clear visual progress bar: Received -> Evidence Submitted -> Under Bank Review -> Won/Lost)
    - **Amount:** "$50.00"
    - **Reason:** "Fraudulent Transaction"
    - **Evidence Used:** A simple bulleted list of what the AI found (e.g., "✔️ Signature on delivery", "✔️ Chat history confirming order").
    - **Advanced Toggle:** For developers or highly technical users to view the raw JSON webhook or generated PDF evidence package.

  ### Key Design Decisions
  - **Invisible Execution:** The system must default to automatic submission if the AI confidence in the evidence is high (e.g., tracking shows 'Delivered'). Manual review is only requested if critical evidence is missing.
  - **Multi-Tenant Data Isolation:** Ensure that when the AI agent pulls communication logs or order histories, strict multi-tenant boundaries (Zero Trust) are maintained so evidence from one merchant cannot leak into another's representment package.
  - **Extensible Evidence Extractors:** The architecture must define standard interfaces for evidence gathering so new modules (e.g., video recordings of unboxing, or digital signature captures) can be easily integrated later.

  ## Implementation Prompt
  **For Implementer Agent:**
  Implement the core `Dispute Event Router` and `AI Legal/Dispute Agent` workflows. The system must listen for simulated webhook events representing disputes (e.g., `chargeback.created`). When triggered, the AI agent must orchestrate data retrieval across multiple mock services (Orders, Communications, Fulfillment) to compile a representment package. Ensure the output is formatted as a structured payload ready for API submission. Acceptance criteria: A test suite demonstrating a simulated "Product Not Received" dispute successfully triggering the automated retrieval of shipping data and customer chat logs, culminating in a generated evidence payload, while emitting a simple notification payload for the mobile client. Do not tightly couple to a specific payment provider's SDK; use abstract interfaces.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []