issue_title: "AI-Powered Smart Quotes & Deposit Architecture for Field Services"
issue_description: |
  ### Title
  AI-Powered Smart Quotes & Deposit Architecture for Field Services

  ### Problem Statement
  Field service operators like Carlos (Handyman) currently spend hours each evening manually writing quotes, tracking down approvals, and chasing deposits. They rely on disjointed tools (e.g., a notebook for voice notes, Word for quotes, Venmo for deposits). Traditional platforms like Jobber or Housecall Pro are too complex and require extensive data entry. OHC needs a seamless, agent-first architecture where a simple voice note from the field generates a professional quote, a booking slot, and a unified deposit payment link, all orchestrated invisibly.

  ### Research Report
  **Findings & Competitive Analysis:**
  - **Jobber / Housecall Pro:** Industry standards for field service, but they operate as passive databases. The user must manually input line items, calculate taxes, and send the invoice.
  - **Thumbtack:** Great for lead gen but extracts high fees and limits the owner's control over the customer relationship.
  - **Shopify/Wix:** Not optimized for service-based quoting or variable deposit collection based on estimates.
  - **OHC Opportunity:** Leverage our Agent Feed and audio command processing (`src/server/api/audio_command.rs`). Carlos can speak into his Android phone ("Draft a quote for the Smith plumbing job, materials $150, labor $200, need 50% deposit"). The Sales Assistant agent parses the audio, drafts a structured quote, generates a Stripe Payment Link with a 50% deposit requirement, and surfaces an Action Card in the Agent Feed for Carlos to 1-tap approve and send via WhatsApp/SMS.

  ### Design Doc
  **Architecture Diagram:**
  ```mermaid
  graph TD
      A[Mobile Client 375px] -->|Audio Voice Note| B(Audio Command API)
      B --> C[Sales Assistant Agent]
      C -->|Extract Intent & Amounts| D[Quote & Estimate Engine]
      D -->|Create Draft Quote| E[Ledger & PostgreSQL]
      D -->|Request Deposit Link| F[Stripe Integration]
      F -->|Return Payment Link| D
      D --> G[Agent Feed Action Queue]
      G --> H[Action Card: Approve Quote]
      H -->|1-Tap Send| I[Omnichannel Dispatcher SMS/WhatsApp]
  ```

  **Mobile UX Flow:**
  1. **Input:** Floating action button on the mobile UI to record a quick voice note.
  2. **Processing:** A translucent glass loading indicator shows "Agent drafting quote...".
  3. **Review:** An Action Card appears in the Agent Feed: "Drafted Quote for Smith Plumbing: $350 total, $175 deposit. [View Quote] [Approve & Send]".
  4. **Approval:** Carlos taps "Approve & Send". The customer receives a WhatsApp message with the OHC-hosted quote and deposit payment link.

  **AI Agent Integration Points:**
  - **Sales Assistant (`src/server/api/agents/`):** Processes the transcribed audio to extract line items, labor, materials, and deposit terms.
  - **Operations Assistant:** Checks the calendar to suggest a service date alongside the quote.

  **Key Design Decisions:**
  - Use asynchronous processing for audio transcription and LLM parsing to ensure the mobile UI remains responsive.
  - Store quotes with versioning and an explicit `status` state machine (`DRAFT`, `PENDING_APPROVAL`, `APPROVED`, `DEPOSIT_PAID`).
  - Utilize Stripe Payment Intents/Links specifically configured for partial deposit captures.

  ### Implementation Prompt
  **For the Implementer Agent:**
  Implement the backend architecture for AI-Powered Smart Quotes & Deposits.
  1. Create the database schema and PostgreSQL migrations for `quotes` and `quote_line_items` with tenant isolation (`tenant_id`). Include states for the quoting lifecycle.
  2. Build the API endpoints in `src/server/api/quotes.rs` (if existing, enhance them; if not, create) to receive parsed intent data and generate quote records.
  3. Integrate with the Stripe billing module to generate a deposit payment link based on a percentage or fixed amount of the quote total.
  4. Wire this into the Agent Feed so that when a quote is drafted via the `Sales Assistant`, an actionable card is pushed to the owner's feed for 1-tap approval and dispatch.
  5. Ensure 100% unit test coverage and add a Playwright E2E test verifying the flow from quote creation to the appearance of the Agent Feed card.

  ### Priority
  P1

  ### Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
