issue_title: "Unified Billing Meter & Customer Acceptance Flow"
issue_description: |
  ## Title
  Unified Billing Meter & Customer Acceptance Flow

  ## Problem Statement
  Currently, OneHumanCorp (OHC) owners like Nora (agency principal) or Maya (baker) lack a single, reliable view of task execution tied directly to real-world financial settlement. The system often reports a workflow step as "completed" based on an AI simulation, approval path, or unknown provider outcome, leading to false completion states (F12). Similarly, "checkout-looking URLs" are generated without persisting real pending states, tracking idempotent retries, or reconciling payments against actual usage (F08). An owner cannot confidently know if a job was actually delivered and paid for, or just "marked done" by an agent. They need a grandmother-test simple way to see exactly what work was verified, what cost was incurred (whether OHC-funded or BYOK), and that money was actually collected.

  ## Research Report
  - **Findings**: The `RESEARCH.md` audit identifies severe gaps in state and financial reconciliation:
    - **F08**: Code fabricates checkout-looking URLs without persisting provider IDs, validating money, or supporting idempotent retries.
    - **F12**: Simulation and unknown provider outcomes masquerade as completion; actual authority and reconciliation checks are missing.
    - **F04 & F05**: Model paths disagree on usage; proxy forwards streams without producing an invoice-grade meter (payer/auth attribution, tenant reads, durable deduplication).
  - **Competitive Analysis**: Shopify and Stripe provide immutable order and receipt states with clear pending/failed/paid indicators. Wix and Squarespace tie booking explicitly to deposit collection. Currently, OHC's disjointed usage events (via `hub.rs` and `auditor.rs`) do not form a reliable ledger compared to these platforms.
  - **Target Segment**: OHC-03–08 (Booking/deposit, delivery/review, final payment).
  - **Baseline Metric**: 0% verified end-to-end payment reconciliation for AI-delivered artifacts. Target: 100% of generated invoices have a matching verifiable provider receipt and customer acceptance cryptographic hash.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      actor Customer
      participant App as Mobile/Desktop UI
      participant API as OHC API Gateway (Zero Trust)
      participant Coordinator as AI Orchestrator
      participant Ledger as ohc_universal_ledger
      participant Provider as External Gateway (Stripe/API)

      Customer->>App: Approve Draft & Pay Deposit
      App->>API: Submit Approval + Payment Token (SPIFFE identity)
      API->>Ledger: Reserve Funds (Pending State)
      API->>Provider: Authorize Payment
      Provider-->>API: Provider Receipt ID
      API->>Ledger: Commit Receipt & Lock Budget
      API->>Coordinator: Trigger Fulfillment Hand-off
      Coordinator->>App: Deliver Artifact
      Customer->>App: Accept Delivery
      App->>API: Sign Acceptance Payload
      API->>Ledger: Settle Final Usage & Complete Job
  ```

  ### UI Wireframes & Mobile UX Flow
  - **375px Mobile-First Flow**:
    1. **Feed View**: A card labeled "Review & Pay" appears with translucent Apple/UniFi styling. Light mode uses `background: rgba(255, 255, 255, 0.65)`, `backdrop-filter: blur(30px) saturate(210%)`.
    2. **Detail View**: Tapping the card expands it. It shows a simple itemized list: "Design Mockups - $200", "Estimated AI Compute - $5".
    3. **Action Button**: A large, thumb-friendly, high-contrast button says "Approve & Pay". No technical jargon is visible; terms like "SPIRE identity" or "Token classes" are hidden behind an "Advanced Details" toggle.
    4. **Success State**: The card morphs into a green-tinted success receipt showing a confirmed Provider ID.
  - **Zero Trust & Security**: All endpoints enforce tenant isolation using the SPIFFE/SPIRE context. The frontend never receives raw provider API keys; it only receives ephemeral payment intent tokens scoped to the specific `ohc_universal_ledger` transaction ID.

  ## Implementation Prompt
  Implement the "Unified Billing Meter & Customer Acceptance Flow".
  - **User-facing outcome**: The business owner sees a unified card on their mobile dashboard that accurately links an AI agent's delivered work to a confirmed, reconciled payment receipt.
  - **Critical User Journey (CUJ)**:
    1. An AI agent completes a draft proposal/artifact.
    2. The system calculates the estimated resource cost and presents a payment request to the customer.
    3. The customer approves and pays.
    4. The system securely records the provider receipt, updates the immutable `ohc_universal_ledger`, and releases the final artifact, marking the job definitively "Complete" (not simulated).
  - **Acceptance Criteria**:
    - Replaces fabricated checkout URLs with a real payment integration loop that records external provider IDs.
    - Prevents jobs from being marked "Complete" unless a corresponding delivery acceptance and payment reconciliation exist in the database.
    - Follows OHC Premium Design Standards (Translucent Glass materials).
    - Requires 100% test coverage for the payment failure, retry, and success paths without relying on fake global-success handlers.
    - Use existing battle-tested Rust crates (e.g., `axum`, `tokio`, `serde`) for the backend.

  ## Priority
  P0

  ## Estimated Scope
  Large

  ## Strategy Admission
  - Target ID: OHC-08 (Delivery/review, final payment, durable exceptions).
  - Segment: Nora (agency principal) / Maya (baker) - Service delivery with required deposits.
  - Evidence level: Documented / implemented code gap (F08, F12).
  - Metric: 100% agreement between ledger invoice balance and provider receipt.
  - Dependencies/reuse: Existing Rust server `axum` routing, Next.js frontend components, and Stripe provider integration.
  - Non-goals: Building a generic ERP system or implementing new complex billing models (e.g., usage-based subscriptions).
  - Authority class: Strict owner financial approval for external spend.
  - Cost plan: Utilize OHC-funded inference strictly within reserved local caps.
  - Acceptance checks: Happy path (payment clears, artifact delivered), failure path (payment declines, job pauses securely).
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
