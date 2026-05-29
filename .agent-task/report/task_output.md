issue_title: "[architecture] Instant Localized Invoicing with AI Operations"
issue_description: |
  # [Instant Localized Invoicing with AI Operations] OneHumanCorp (OHC)

  ## Problem Statement
  Small business owners, especially those offering services (like Carlos the handyman) or custom orders (like Maya the baker), spend a disproportionate amount of time manually drafting quotes, converting them to invoices, tracking deposit payments, and ensuring compliance with local invoicing laws. These non-technical users often rely on disjointed tools (e.g., Word docs for quotes, Venmo for deposits, Excel for tracking) which introduces friction, delays payment, and feels unprofessional. They need a system that autonomously translates a simple request (e.g., "Quote $500 for the plumbing job, 50% upfront") into a fully compliant, locally tailored invoice with built-in payment collection, all without leaving their phone.

  ## Research Report
  - **Market Gap:** While Square and Stripe offer robust invoicing, their interfaces are desktop-first and require manual data entry for every line item. For micro-businesses, the cognitive load of navigating "Items," "Taxes," and "Terms" is high.
  - **Competitor Analysis:** Shopify's draft orders are e-commerce focused. QuickBooks is too complex for our target personas.
  - **OHC Advantage:** By leveraging the AI Finance Department, OHC can automatically draft invoices based on natural language inputs or conversation history (e.g., parsing an Instagram DM where a price was agreed upon). The AI acts as the "Accountant," managing the transition from Quote -> Deposit -> Final Invoice seamlessly.
  - **Localization Need:** In regions outside the US (e.g., LATAM, EU), invoices must adhere to specific formatting and tax registration requirements (e.g., e-invoicing mandates). The system must dynamically adapt to these local regulations.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  flowchart TD
      subgraph Trigger
          UI[Mobile App 375px UI]
          Chat[Agent Inbox / DM]
      end

      subgraph OHC Departments
          Ops["Operations Agent (Context)"]
          Fin["Finance Agent (The Accountant)"]
      end

      subgraph Core Systems
          Ledger[(Ledger DB)]
          Invoices[(Invoices DB)]
          Config[(Tenant Localization Config)]
      end

      subgraph External
          PaymentGateway[Payment Processor]
          Notification[SMS / Email System]
      end

      UI --> Fin
      Chat --> Ops
      Ops -. "Passes deal context" .-> Fin

      Fin <--> Config
      Fin --> Invoices
      Fin --> Ledger

      Invoices --> Notification
      Notification --> PaymentGateway
      PaymentGateway --> Ledger
  ```

  ### Mobile UX Flow & UI Wireframes (375px First)
  1. **Creation:** User opens the app and speaks/types: "Send Carlos a quote for fixing the sink, $150."
  2. **Draft Review (Glassmorphism Card):** A translucent, beautifully formatted card appears: "Quote for Sink Repair - $150". It includes an auto-generated description and a pre-calculated local tax line.
  3. **Approval:** A single "Approve & Send" button at the bottom. No complex menus.
  4. **Payment Tracking:** The Home Dashboard displays a clean progress bar for "Awaiting Payment" vs "Paid", updated in real-time.

  ### Key Design Decisions & Integrity
  - **Multi-Tenant Isolation:** All invoice and ledger entries must strictly enforce Row Level Security (RLS) based on `tenant_id`. The AI context window must be strictly partitioned to prevent data leakage between businesses.
  - **Zero-Trust Security:** Communication between the Finance Agent and the Invoices DB must occur over mutually authenticated (mTLS) channels via SPIFFE/SPIRE.
  - **Offline Capability:** Draft invoices can be created offline. The app queues the creation request and syncs with the OHC backend once connectivity is restored.
  - **Dynamic Localization:** The Finance Agent queries the `Config` DB to inject region-specific fields (e.g., VAT ID, local currency symbol) before presenting the draft.

  ## Implementation Prompt
  **Task:** Implement the Instant Localized Invoicing capability driven by the AI Finance Agent.
  **CUJ:** A user (e.g., Carlos the handyman) inputs a natural language request to bill a client. The system must autonomously draft a professional, localized invoice, present it for a single-tap approval on mobile, and handle the subsequent state transitions (Quote -> Unpaid -> Paid).
  **Acceptance Criteria:**
  - Define the data models for Quotes and Invoices with strict multi-tenant isolation.
  - Implement the AI prompt engineering and tool calling for the Finance Agent to generate structured invoice data from unstructured input.
  - Build the API endpoints to manage the invoice lifecycle (Draft, Approve, Pay).
  - Ensure the system dynamically applies basic localization rules (currency, tax labels) based on the tenant's configured region.
  - E2E tests must verify the full flow from natural language input to invoice creation and state update.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
