issue_title: "[Platform Enhancement] Agentic Subscription Replenishment Architecture & UI"
issue_description: |
  # [Platform Enhancement] Agentic Subscription Replenishment Architecture & UI

  ## Problem Statement
  Small business owners like Maya the baker or Leo the music tutor struggle to maintain recurring revenue because managing subscriptions is manual and time-consuming. Traditional platforms (Shopify, Wix) offer subscriptions but require owners to manually send reminders, manage failed payments, and handle pause/resume requests. Customers often cancel simply because it's too difficult to adjust their upcoming order (e.g., "I'm out of town this week, can we skip?").

  Owners need an agentic, zero-touch system that proactively manages subscription health. The system should anticipate fulfillment, automatically reach out to customers to confirm or adjust upcoming deliveries/lessons via their preferred channel, and handle the backend adjustments—all while keeping the owner informed via a simple mobile feed.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify Subscriptions:** Robust backend, but customer interaction is mostly limited to transactional emails. Adjustments require the customer to log into a portal, causing friction.
  - **Recharge (Shopify App):** Offers SMS management (e.g., "Reply SKIP to skip this month"), but it's rigid and rule-based, not conversational AI.
  - **Wix Pricing Plans:** Good for digital/services, but lacks proactive, conversational engagement for physical goods replenishment.
  - **OHC Opportunity:** Leverage our Agentic workflow. The "Retention Agent" monitors upcoming subscription cycles. 5 days before fulfillment, it proactively messages the customer (via SMS/WhatsApp): "Hi Sarah, your vegan cake box is set for this Friday. Want to add an extra slice or skip this week?" The agent understands natural language replies, adjusts the backend order/subscription state, and simply updates the owner's daily manifest.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Subscription Cron/Scheduler] -->|Upcoming Cycle Event| B(Event Mesh)
      B --> C[Retention Agent]
      C -->|Query Subscription Context| D[PostgreSQL/Tenant DB]
      C -->|Draft Proactive Outreach| E[Omnichannel Dispatcher]
      E -->|SMS/WhatsApp| F[Customer]
      F -->|Reply: "Skip this week"| E
      E --> G[Retention Agent]
      G -->|Analyze Intent| H{Update Subscription State}
      H -->|Success| I[Tenant Daily Manifest/Feed]
      H -->|Update DB| D
  ```

  ### Mobile UX Flow (375px First)
  - **Owner View (Mobile Feed):**
    - The feed shows a summary card: "Subscription Update: 3 upcoming orders confirmed, 1 skipped."
    - Tapping the card shows details: "Sarah skipped her Friday cake box (Out of town). Next delivery scheduled for next Friday."
  - **Customer View (Conversational):**
    - No app needed. The customer interacts entirely via their preferred chat channel (SMS/WhatsApp). The AI handles the natural language processing.

  ### AI Agent Integration Points
  - **Retention Agent:** Wakes up on cron schedules tied to subscription cycles. Generates context-aware, personalized messages based on past order history and current inventory. Handles natural language intents (Skip, Pause, Add-on, Cancel) and maps them to API actions.

  ### Key Design Decisions
  - **Conversational UI for Customers:** Removing the "login to manage subscription" barrier drastically reduces churn.
  - **Owner Abstraction:** The owner doesn't need to see every successful ping. They only see exceptions or a consolidated summary in their feed.

  ## Implementation Prompt
  **User-Facing Outcome:** Maya opens her OHC app and sees her upcoming subscription deliveries. She notices the AI automatically handled a skip request from a customer who is traveling, without her having to lift a finger.

  **CUJ & Acceptance Criteria:**
  1. Implement a cron-like scheduler (or simulated event trigger) that identifies subscriptions due for fulfillment within X days.
  2. Implement the Retention Agent logic to draft a proactive message and send it via a mock omnichannel dispatcher.
  3. Implement a webhook handler that receives a simulated customer reply (e.g., "Skip this week please").
  4. The Retention Agent must parse the intent and correctly update the subscription status in the database to 'skipped' for that cycle.
  5. The UI must render a feed item on a 375px viewport summarizing the action taken by the AI.
  6. Provide Playwright E2E tests validating the owner's view of the updated subscription state after the simulated customer interaction.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
