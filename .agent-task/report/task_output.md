issue_title: "Agent-Driven Intelligent Booking & Deposit Handoff"
issue_description: |
  # Agent-Driven Intelligent Booking & Deposit Handoff

  ## Problem Statement
  Service operators like Carlos need a way to seamlessly convert customer inquiries into bookable, quoted appointments with deposits. The process is currently manual, requiring the operator to interpret a message ("My sink is leaking, can you come today?"), generate a quote, determine a deposit amount, check availability, and send a booking link. This friction leads to delayed responses and lost revenue.

  ## Research Report
  - **Competitor Analysis:** Field service and CRM tools (like Housecall Pro, Jobber, and HubSpot) automate booking flows, but they rely on static rule sets or forms. AI-native tools are beginning to parse intent but often struggle with end-to-end execution (quote -> schedule -> payment).
  - **User Pain Point:** "Carlos (handyman, 42): No website, word-of-mouth only. Pain: no booking system, quoting is manual, misses leads when busy."
  - **Proposed Solution:** Implement an "Autonomous Quote" feature where the AI parses the inquiry, drafts a quote with a suggested price and deposit, reserves tentative calendar slots via Redis Redlock to prevent double-booking, and presents this to the operator in their Feed as an Action Card. Upon approval, a Stripe deposit link is generated and sent to the customer.

  ## Design Doc
  - **Backend (Rust):**
    - Create a new endpoint `/simulate-autonomous-booking-quote` in `src/server/api/agents/approvals.rs`.
    - Use `RedisLock` to acquire a temporary hold on the proposed time slot (`booking_slot`).
    - Dispatch an action to the `Sales` department orchestrator with `feature_type: "autonomous_quote"`, including the `suggested_price`, `proposed_slots`, and `deposit_amount_cents`.
  - **Frontend (Next.js):**
    - Update `FeedPage` (`src/ui/next/src/app/feed/page.tsx`) to include a simulation button for testing (`simulateBookingDraft`).
    - The Feed UI handles the action card.
  - **Testing (Playwright):**
    - Add a test `src/e2e/playwright/handyman_flow.spec.ts` that navigates to the feed, triggers the simulation, verifies the generated Action Card appears with the correct text ("Action Required: Approve Estimate"), and clicks "Approve".

  ## Implementation Prompt
  - Ensure the test `handyman_flow.spec.ts` correctly locates and interacts with the generated action card.
  - The card should display "Action Required: Approve Estimate" and "My sink is leaking, can you come today?".
  - The test must verify that the card disappears or transitions to a success state after approval.
  - **Note:** Local testing revealed issues with PostgreSQL `pgvector` extension and database connection pooling under load during E2E tests, which may cause the card generation to time out in CI.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
