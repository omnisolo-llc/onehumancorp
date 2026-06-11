issue_title: "Implement High-Performance Multi-Tenant Dynamic Pricing & Instant Quotes Cache"
issue_description: |
  ## Issue Title
  Implement High-Performance Multi-Tenant Dynamic Pricing & Instant Quotes Cache

  ## Problem Statement
  Small business owners such as Carlos the handyman or Maya the baker need to be able to instantly calculate dynamic pricing and issue instant quotes based on rules (e.g. rush fees, seasonal discounts, customer loyalty tier, or specific travel distance for mobile services). Currently, creating a quote is either a manual process or requires a full roundtrip to the backend LLM which adds unpredictable latency. Customers expect instant feedback on pricing when configuring a service request on the booking or storefront UI. If the quoting engine is slow or offline, they abandon the cart or booking flow.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Traditional Builders (Wix/Squarespace)**: Rely on static pricing tables or simple percentage discounts. They do not have conditional, multi-variable logic engines for mobile services (e.g., "$50 base + $2/mile + 20% rush fee if booked for tomorrow").
  - **Shopify**: Requires complex third-party apps for advanced dynamic quoting or custom product builder flows.
  - **GoDaddy**: Very rigid pricing structures.
  - **OHC Opportunity**: Introduce a localized, edge-cached, multi-tenant rules engine. The "Sales Agent" pre-computes or securely syncs the pricing rules to the client (or edge) so that as the customer toggles options (e.g., vegan, delivery, rush), the price updates instantly <50ms without hitting the LLM. The AI only engages to negotiate or draft the final proposal text.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Owner configures Pricing Rules via AI] --> B(Pricing Engine Config Store - Postgres)
      B --> C[Edge/Local Cache Sync - Redis/PowerSync]
      C --> D[Customer Client - Web/Mobile]
      D --> E{Instant Rule Evaluation <50ms}
      E -->|User toggles options| E
      E -->|Finalize| F[Generate Instant Quote]
      F --> G[Sales Agent drafts proposal text]
      G --> H[Present to Customer]
  ```

  ### Mobile UX Flow (375px)
  1. **Customer View**: A clean, modular "Glassmorphism" form where they select service parameters (e.g., Cake Size, Vegan, Delivery Date).
  2. **Instant Feedback**: At the bottom of the screen, a sticky "Estimated Quote" bar instantly updates as they tap options.
  3. **Submit**: Customer taps "Request Final Quote".
  4. **Owner View**: The owner receives an "Action Required" card in their Work Triage feed: "New Quote Request from [Name]. AI has applied all rules. Tap to approve and send."

  ### AI Agent Integration Points
  - **Setup Phase**: The owner tells the AI "I charge $50 an hour, but double on weekends." The AI translates this into deterministic JSON pricing rules.
  - **Execution Phase**: The rules execute deterministically on the edge/client for speed.
  - **Closing Phase**: The Sales Agent generates the polite email/message to deliver the quote and handles follow-ups.

  ### Key Design Decisions
  - **Deterministic Fast-Path**: Pricing calculation must be deterministic and executed on the client/edge using synced rules, not via LLM inference, to guarantee <50ms latency.
  - **Multi-Tenant Isolation**: Pricing rules must be strictly keyed by `tenant_id` to ensure one business's rules never leak to another.
  - **Offline Tolerance**: If a field worker (like Carlos) is generating a quote on-site without cell service, the local app should use cached rules to generate an estimate.

  ## Implementation Prompt
  Implement the Dynamic Pricing & Instant Quotes Engine.
  1. **Backend**: Create the data models and API endpoints to store and retrieve structured, multi-tenant pricing rules (e.g., base price, modifiers based on conditions). Ensure multi-tenant isolation.
  2. **Frontend/Edge**: Implement a high-performance evaluation engine on the client side that consumes these rules. As the user selects options in the booking/checkout flow, it should instantly calculate and display the estimated price.
  3. **Owner UI**: Create a clear "Glassmorphism" card in the dashboard where the owner can view and approve generated quotes.
  4. **Verification**: Write full Playwright E2E tests simulating a customer interacting with the options and verifying the instant price updates without network requests during the interaction. Ensure the owner flow also works on a 375px viewport.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
