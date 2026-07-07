issue_title: "AI-Driven Subscription & Membership Lifecycle Management"
issue_description: |
  # AI-Driven Subscription & Membership Lifecycle Management

  ## Problem Statement
  Small business owners such as Leo (music tutor) and Priya (boutique operator) need to convert one-off customers into recurring revenue through subscription packages, memberships, or replenishments. Currently, offering subscription services requires expensive third-party apps (e.g., Recharge on Shopify), which create a fragmented user experience and disjointed customer data. Without native support, owners are stuck tracking subscription renewals, expired cards, and usage limits manually, losing valuable revenue and time.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify:** Relies almost entirely on apps like Recharge or Skio for subscription management. This results in the "App Tax" fatigue, disjointed billing experiences for end users, and complex data syncing.
  - **Wix & Squarespace:** Offer basic recurring billing, but the operational follow-up is manual. There is no intelligent automation handling failed payments gracefully or prompting members for upgrades.
  - **OHC Opportunity:** Implement an AI-native subscription module within the core platform. The Finance Agent handles automated billing and dunning, while the Operations Agent tracks package usage (e.g., 4 lessons per month) and the Customer Success Agent handles personalized communications regarding upcoming renewals, skipped months, or payment issues without any manual intervention from the owner.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Checkout] -->|Creates| B(Subscription Intent)
      B --> C[Unified Customer Graph DB]
      B --> D[Billing Service / Stripe Billing]
      D -->|Webhook - Renewal / Failed| E(Event Mesh)
      E --> F{Finance & Operations Agents}
      F -->|Failed Payment| G[Customer Success Agent drafts email]
      F -->|Successful Renewal| H[Update Entitlements / Quotas]
      G --> I[Owner Approval Feed - 375px]
      I -->|Approve| J[Send Reminder to Customer]
      H --> K[Customer Access / Lesson Booked]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Owner View (Mobile Feed):**
    - **Card:** "Action Required: 3 Subscriptions need payment updates. Agent drafted reminders."
    - **Interaction:** Tapping the card opens a list of the 3 customers. The drafted message uses the customer's preferred channel (e.g., WhatsApp). The owner can tap "Approve All" to send.
  - **Setup Flow (Mobile):**
    - Owner taps "Add Offer" -> selects "Recurring Membership".
    - Enters plain English: "4 guitar lessons a month for $200".
    - System autonomously generates the product model, pricing tiers, and entitlement quota without requiring the owner to understand "billing intervals" or "usage limits".

  ### AI Agent Integration Points
  - **Operations Agent:** Interprets the plain English offer description into strict database constraints (e.g., `max_uses: 4, interval: 30 days`).
  - **Finance Agent:** Listens to billing webhooks. Generates anomaly reports (e.g., "Churn rate is up 5% this month") rather than just showing raw charts.
  - **Customer Success Agent:** Uses RAG against customer history to draft empathetic dunning messages (e.g., "Hi Leo, looks like your card expired! Here is a secure link to update it so you don't miss next week's lesson.")

  ### Key Design Decisions
  - **Entitlements First:** The database must track *entitlements* (what the customer has a right to) separately from *billing*. This allows for flexible pausing, gifting, and usage tracking.
  - **Unified Graph:** The subscription record links directly to the customer entity and product entity with strong multi-tenant row-level security.
  - **Zero App Tax:** All subscription capabilities are native and included, requiring no third-party installations.

  ## Implementation Prompt
  **Target Persona:** Leo the Music Tutor
  **Outcome:** Leo can create a "4 lessons a month" package from his phone by simply describing it. When a student's card is about to expire, the AI drafts a reminder message for Leo to approve with one tap.
  **CUJ & Acceptance Criteria:**
  1. Add a `subscriptions` and `entitlements` table in PostgreSQL with tenant-level RLS.
  2. Implement an API route to parse a natural language string ("4 lessons per month") into structured subscription pricing and quota data via the LLM.
  3. Create an E2E Playwright test where a mock user creates a subscription product from the UI, a test customer "purchases" it, and an entitlement of "4 lessons" is accurately reflected in the database.
  4. Ensure the setup flow operates flawlessly on a 375px viewport.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []