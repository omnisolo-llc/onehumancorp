issue_title: "Implement Unified Multi-Tenant Subscription & Recurring Billing Engine"
issue_description: |
  # [Architecture] Unified Multi-Tenant Subscription & Recurring Billing Engine

  ## Problem Statement
  Service providers and creators on OneHumanCorp — like Leo the music tutor offering monthly lesson packages, or Priya the boutique owner selling quarterly clothing boxes — have no built-in way to automatically charge customers on a recurring basis. Currently, they have to manually send payment links each month, which leads to late payments, awkward follow-ups, and churn. They need a "set it and forget it" subscription engine that seamlessly handles recurring charges, proration, failed payment retries (dunning), and auto-generates invoices, all manageable directly from their phone.

  ## Research Report
  - **Competitor Systems Audit:**
    - **Stripe Billing / Subscriptions:** Industry standard, highly robust, but too complex for non-technical users to configure directly. Requires API knowledge or navigating a complex developer dashboard.
    - **Shopify Subscriptions:** Good ecosystem integration, but heavily skewed towards physical product replenishment. Weak on service-based time-block subscriptions.
    - **Wix Pricing Plans:** Easy to use for members, but lacks flexible cross-channel capabilities (e.g., selling a subscription natively via an Instagram DM).
  - **Gaps Identified in OHC:** We lack a native, Zero-Trust multi-tenant billing engine that unifies physical, digital, and service-based subscriptions. Furthermore, there is no AI integration to proactively handle churn (e.g., the CS Agent reaching out via SMS when a card fails).
  - **Opportunity:** Build a highly abstracted billing engine backed by K8s and LangGraph. Hide the complexity (proration, webhook handling) from the business owner. Expose a simple "Subscribe" toggle on any product or service listing.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Mobile User (375px Viewport)
          UI[Subscription Dashboard Cards]
          T[Translucent Glass Setting Toggles]
      end

      subgraph OHC Control Plane
          API[Billing API Gateway / Zero-Trust SPIFFE]
          SubMgr[Subscription Manager CRD]
      end

      subgraph AI Swarm
          CS_Agent[Customer Service Agent]
          Fin_Agent[Finance & Legal Agent]
      end

      subgraph Infrastructure
          DB[(Multi-Tenant Ledger / Postgres)]
          Stripe[Stripe/Payment Processor MCP]
      end

      UI --> API
      API --> SubMgr
      SubMgr --> DB
      SubMgr --> Stripe
      Stripe -- "Webhook (Payment Failed)" --> Fin_Agent
      Fin_Agent -- "Triggers Recovery" --> CS_Agent
      CS_Agent -- "SMS: Card Expired" --> UI
  ```

  ### Mobile-First UX Flow & Wireframes (375px)
  - **Design Language:** macOS-style Translucent Glass materials, Ubiquiti UniFi modular dashboard cards. Clean, grandmother-friendly typography.
  - **Screen 1: Product/Service Setup:**
    - A simple card layout for the product/service.
    - A prominent toggle switch: **"Offer as Subscription"**.
    - When toggled ON, a glass-morphic bottom sheet slides up:
      - **Frequency:** Dropdown (Weekly, Monthly, Yearly).
      - **Discount:** Optional percentage off for subscribers.
  - **Screen 2: Subscriber Management:**
    - A list view of active subscribers with green/red status indicators.
    - Tapping a subscriber reveals a detailed card: Next billing date, MRR, and a one-tap button: **"Pause Subscription"**.

  ### AI Agent Integration Points
  - **Finance Department:** Automatically calculates prorations when a customer upgrades or downgrades. Reconciles ledger entries.
  - **CS Department (Dunning):** When a payment fails, the CS agent automatically drafts and sends a polite text/DM to the customer with a secure 1-click link to update their payment method, mimicking the business owner's tone.

  ### Key Design Decisions
  - **Abstracted Complexity:** No mention of webhooks, proration logic, or API keys in the UI. Everything is abstracted into human terms ("Monthly," "Pause").
  - **Zero-Trust Multi-Tenancy:** Each tenant's billing data and Stripe customer mappings are cryptographically isolated via SPIFFE identities.
  - **LangGraph for Dunning:** The failed payment recovery process is modeled as a stateful LangGraph workflow, ensuring the system can pause, wait for user action (updating a card), and automatically retry charging without dropping state.

  ## Implementation Prompt
  Implement the backend architecture and frontend components for the Unified Subscription & Recurring Billing Engine. Create the necessary database models for plans, subscriptions, and multi-tenant billing ledgers. Build the API endpoints to create, pause, and cancel subscriptions. On the frontend, build the 375px-optimized translucent glass UI for toggling a product into a subscription and viewing active subscribers. Finally, implement the webhook handler that listens for failed payments and triggers the LangGraph-based CS Agent to start the automated dunning process.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []