issue_title: "[Research] Autonomous Abandoned Cart Recovery Workflow"
issue_description: |
  # Title
  [Research] Autonomous Abandoned Cart Recovery Workflow

  ## Problem Statement
  For our core persona (e.g., Priya the boutique owner or Maya the baker), capturing a lead or a custom order in the DMs/storefront often drops off before payment. They lack the time and tooling to manually track down abandoned carts. Existing platforms (Shopify) require complicated third-party plugins (Klaviyo) and manual email configuration, which is fundamentally misaligned with OHC’s "invisible autonomous agents" vision. We need an integrated, zero-click Cart Recovery Agent that operates invisibly in the background to recover lost revenue via WhatsApp/Email/SMS.

  ## Research Report
  - **Market Gap:** According to our `ohc_smb_market_report.md` research, "Automated Cart Recovery" is one of the Top 5 Pillar Automations that SMBs desperately need but find too complex to configure. "Customer Follow-up (10%)" is a top 5 pain point.
  - **Competitor Landscape:**
    - Shopify: Requires apps like Klaviyo. Costs $20-$45/mo extra. Requires template building.
    - Wix: Basic auto-email, but not agentic. Doesn't handle two-way conversational negotiation (e.g., customer replies "is shipping free?").
  - **OHC Advantage:** We can use our Unified Agent Feed and Omnichannel architecture to deploy a `Sales & Revenue Assistant` that not only sends a reminder, but *drafts* it in the owner's tone, and *negotiates* or answers questions if the customer replies.

  ## Design Doc
  - **Architecture Diagram:**
    ```mermaid
    graph TD
        Checkout[Customer drops off Checkout] -->|Event Trigger| JobQueue[Postgres SKIP LOCKED Queue]
        JobQueue -->|Dequeue| CartRecoveryAgent[Sales & Revenue Assistant]
        CartRecoveryAgent -->|Context Fetch| TenantDB[Tenant Orders & Memory]
        CartRecoveryAgent -->|Draft Message| LLM[Gemini Pro]
        LLM -->|Generate Text + Link| ChannelAdapter[Email / SMS / WA Adapter]
        ChannelAdapter -->|Send| Customer[Customer]
        Customer -->|Reply| Triage[Work Triage Inbox]
    ```
  - **Mobile UX Flow (375px):**
    1. Owner opens OHC app -> Sees a notification in the Work Feed: "Recovered $45 from abandoned cart (Maya's custom cake)."
    2. Owner clicks the feed item -> Sees a transparent glassmorphism card showing the agent's conversation with the customer.
    3. *Advanced settings (hidden by default)*: Owner can toggle the agent "Off" or set a "Discount Limit" (e.g., offer 10% off after 24h).
  - **AI Integration Points:**
    - `system_prompt` for the Cart Recovery skill: Instructs the LLM to write a friendly, concise reminder tailored to the specific product in the cart.
    - Memory: Stores the context of the cart so if the customer replies, the triage agent knows they are replying to an abandoned cart reminder.

  ## Implementation Prompt
  **Goal:** Build the invisible Cart Recovery agent pipeline.
  **CUJ (Critical User Journey):**
  1. A customer adds items to a cart but does not complete checkout (simulated via an API endpoint or DB state).
  2. A scheduled background job picks up the "abandoned" cart.
  3. The Sales Agent drafts a personalized follow-up message containing a link to resume checkout.
  4. The message is dispatched via the configured channel (simulated via a mock adapter in E2E tests).
  5. The owner sees a summary card in their Unified Agent Feed saying "Agent followed up with [Customer Name] about their cart."

  **Acceptance Criteria:**
  - Create the background job logic to detect abandoned carts (e.g., > 1 hour old).
  - Implement the agent prompt and generation step.
  - Add a feed item to the `Unified Agent Feed` so the owner has visibility.
  - Write a comprehensive Playwright E2E test verifying the flow from the owner's perspective (seeing the feed item).
  - Ensure 100% unit test coverage for new backend code.
  - **No UI mocks**: All feed data must come from the actual backend database.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
