issue_title: "Implement Autonomous AI Vision Estimator & Quoting Engine for Field Services"
issue_description: |
  ## Problem Statement
  Field service operators like **Carlos (handyman)** spend hours off-site reviewing photos sent by customers via WhatsApp, iMessage, or email to generate quotes. This manual review and estimation process creates a severe bottleneck. Customers expect rapid estimates, and Carlos loses leads if he cannot respond while on another job. OHC currently handles text-based omni-channel intake but lacks a native vision-to-quote pipeline capable of instantly turning customer photos into structured, itemized estimates with deposit links.

  ## Research Report
  - **Codebase & Docs Audit**: The current `omni_channel_quote_to_cash_engine` supports text triage and basic intent classification. However, it lacks an integration with multi-modal LLMs (like Gemini Pro Vision or GPT-4o) to process inbound media attachments, estimate material costs, and cross-reference the OHC inventory/service ledger.
  - **Competitor Analysis**:
    - *Jobber & Housecall Pro*: Require manual photo upload by the operator and manual line-item creation. No AI visual estimation.
    - *Shopify/Wix*: Lack service-oriented quoting workflows entirely.
  - **The Gap**: OHC has an opportunity to offer "Visual Quoting Autonomy." When a customer sends a photo of a broken fence or leaky pipe, the AI should instantly analyze the damage, estimate labor and parts, and draft a quote for Carlos's 1-tap approval.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant Ingress Gateway (WhatsApp/SMS)
      participant KAIROS Orchestrator
      participant Vision AI Agent
      participant Ops & Finance Agent
      participant Mobile UI (375px)

      Customer->>Ingress Gateway: Sends Photo ("How much to fix this?")
      Ingress Gateway->>KAIROS Orchestrator: Ingest Media Event
      KAIROS Orchestrator->>Vision AI Agent: Delegate Media Analysis
      Vision AI Agent-->>Vision AI Agent: Identify Objects, Damage, Scope
      Vision AI Agent->>Ops & Finance Agent: Request Estimate (Labor + Materials)
      Ops & Finance Agent-->>Vision AI Agent: Returns Itemized Quote Draft
      Vision AI Agent->>KAIROS Orchestrator: Draft Quote & Queue for Approval
      KAIROS Orchestrator->>Mobile UI: Push Notification: "Quote Drafted for Approval"
      Mobile UI->>KAIROS Orchestrator: Carlos Approves (1-tap)
      KAIROS Orchestrator->>Ingress Gateway: Send Quote + Deposit Link
      Ingress Gateway->>Customer: Delivers Quote
  ```

  ### Mobile UX Flow (375px)
  1. **Unified Inbox**: Carlos sees a new message from a lead containing a photo, badged with an AI Sparkle ✨ indicating a drafted estimate is ready.
  2. **Quote Review Modal (Bottom Sheet)**: Tapping the message slides up a glassmorphic bottom sheet. It shows the customer's photo alongside an AI-generated itemized list (e.g., "Drywall repair: $150, Labor: 2 hours").
  3. **1-Tap Action**: A prominent primary button allows Carlos to "Approve & Send Quote," or he can tap individual line items to adjust prices using a native numeric keypad.
  4. **Payment Link**: Upon approval, the quote is dispatched with a Stripe Payment Link for the deposit.

  ### AI Agent Integration Points
  - **Vision AI Agent (New)**: Specialized KAIROS agent leveraging multi-modal LLM providers. Responsible for parsing images, identifying service categories, and estimating complexity.
  - **Salesperson/Finance Agent**: Coordinates with the Vision Agent to map identified issues to the tenant's service pricing catalog and generates the Stripe deposit link.

  ### Key Design Decisions
  - **Human-in-the-Loop Required**: Given the physical and financial risks of misquoting field service work, AI-generated visual quotes *must* require the owner's 1-tap approval before sending. Auto-send is disabled for vision quoting.
  - **Tenant-Scoped Memory**: The Vision Agent must learn from Carlos's past accepted quotes (e.g., if he consistently charges more for complex tile work) using strictly isolated tenant vector embeddings.

  ## Implementation Prompt
  **To Implementer Agent:**
  Implement the Vision AI Estimator workflow within the KAIROS Orchestrator.
  1. Extend the Ingress Gateway to handle multipart media payloads (images) from WhatsApp/SMS and persist them to tenant-isolated GCS/MinIO buckets.
  2. Create a `VisionAIAgent` capability in the backend that calls the configured Multi-Modal LLM (Gemini Pro Vision fallback to GPT-4o) to analyze the image.
  3. Implement the logic to map the LLM's visual analysis to the tenant's `ServiceCatalog` and draft a `Quote` record marked as `PENDING_OWNER_APPROVAL`.
  4. Build the mobile UI (375px) `VisualQuoteReviewSheet` allowing the owner to view the photo, edit line items, and approve the quote with one tap.
  5. Include E2E Playwright tests verifying the ingestion of a mock image, quote generation, and owner approval flow.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
