issue_title: "Automated Cart Recovery via Agents"
issue_description: |
  ## Mission Queue Protocol: Automated Cart Recovery via Agents

  ### Problem Statement
  Users frequently abandon shopping carts, representing significant lost revenue for SMBs. The platform currently lacks a native, automated mechanism to re-engage these users without requiring complex third-party integrations (like Klaviyo), which alienates non-technical users. Small businesses, like Maya the Home Baker or Carlos the Field Service Owner, need a system that monitors abandoned carts and sends personalized follow-ups to recover lost sales.

  ### Research Report
  - **Market Context:** Cart recovery is a proven revenue driver. Existing platforms like Shopify often rely on third-party apps for robust follow-up sequences.
  - **OHC Opportunity:** By integrating cart recovery natively and powering it with AI agents, OHC can provide a seamless, proactive recovery mechanism that doesn't require user configuration or extra monthly fees.
  - **Competitor Gaps:** Most platforms require manual setup of email templates and delays. An agentic approach can personalize the message based on the cart contents and the specific business context.

  ### Design Doc
  #### Architecture Diagram (Mermaid)
  ```mermaid
  erDiagram
      Tenant ||--o{ Customer : has
      Customer ||--o{ Cart : creates
      Cart ||--o{ CartItem : contains
      Tenant ||--o{ RecoveryCampaign : configures
      RecoveryCampaign ||--o{ RecoveryAttempt : triggers
      RecoveryAttempt }o--|| Cart : "recovers"
      RecoveryAttempt }o--|| Customer : "targets"
  ```

  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC Storefront
      participant Job Queue
      participant Sales Agent
      participant OHC App

      Customer->>OHC Storefront: Adds item to cart, leaves
      OHC Storefront->>Job Queue: Schedule cart check event
      Note over Job Queue: Wait 1 hour (configurable)
      Job Queue->>Sales Agent: Trigger recovery check
      Sales Agent->>Database: Evaluate cart & campaign rules
      Sales Agent-->>Sales Agent: Draft personalized recovery message
      Sales Agent->>OHC App: Push message to Agent Feed
      OHC App->>Merchant (Maya): Shows drafted message for approval
      Merchant (Maya)->>OHC App: Approves message
      OHC App->>Customer: Send Recovery Email/SMS
  ```

  #### Data Model (PostgreSQL)
  - `RecoveryCampaign`: Configuration for the recovery process (e.g., delay time, auto-send flag).
  - `RecoveryAttempt`: Tracks individual recovery attempts, linking to the customer, the abandoned cart (source event), and the generated message.

  #### AI Integration
  - **Sales/Marketing Agent:** Monitors cart events, evaluates if a cart is abandoned based on the `RecoveryCampaign` rules, and uses an LLM to generate a personalized recovery message (e.g., "Hey, noticed you were looking at the Vegan Chocolate Cake. Complete your order now and we'll throw in a free cookie!").
  - **Operations Agent (Job Queue):** Handles the delayed execution (e.g., "Check this cart in 1 hour"). The existing `ohc_job_queue` can be utilized by scheduling jobs with a `next_retry_at` timestamp in the future.

  #### Mobile UX Flow (375px)
  - **Merchant View (Dashboard):** The merchant sees a unified feed (Agent Feed) of recovery actions. They can view the drafted messages, approve them (if manual approval is required), and see the resulting revenue from recovered carts. No complex setup screens; just actionable insights and approvals.

  #### Top 5 Things That Do Not Make Sense In The Repo
  1. Use of unstable let_chains features that cause rust clippy failures
  2. Legacy Next.js frontend code is still in the repo while Tauri v2 is the canonical UI
  3. No clear strategy documented for how `ohc_job_queue` items handle tenant-level rate limiting
  4. Missing `.gitignore` rule for `.agent-task` directory
  5. Use of hardcoded paths inside some E2E tests instead of configurable environment variables

  ### Implementation Prompt
  **Feature Name:** OHC Automated Cart Recovery Agent
  **Target Persona:** Maya the Home Baker
  **Outcome:** Maya automatically recovers abandoned cart revenue without configuring email campaigns. The Sales Agent drafts personalized messages for abandoned carts, which Maya can approve via her Agent Feed, or have sent automatically.

  **Next Actions:**
  1. Leverage the existing `ohc_job_queue` to schedule cart check events (e.g., 1 hour after the last cart update).
  2. Implement the Sales Agent capability to evaluate abandoned carts and generate personalized recovery messages using the LLM.
  3. Integrate the generated messages into the `Agent Feed` for merchant review and approval (if `auto_send` is false).
  4. Build the E2E Playwright test to verify the automated cart recovery flow, ensuring no mock data is used and the agent correctly drafts a message after the simulated delay.

  **Priority:** P1
  **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
