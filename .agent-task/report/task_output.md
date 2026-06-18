issue_title: "Implement Autonomous Customer Lifecycle Engagement Agent"
issue_description: |
  **Problem Statement**
  Small business owners like Priya (boutique owner) and Leo (music tutor) struggle to consistently engage customers across their lifecycle. They lose potential revenue because they forget to follow up with leads, don't send replenishment reminders, and fail to re-engage dormant customers. Existing tools require manual campaign setup, complex segmentation logic, and constant monitoring—things owners don't have time for. OHC needs a continuous, invisible background agent that tracks customer state and proactively drafts lifecycle communications for owner approval.

  **Research Report**
  Market analysis shows a massive gap in SMB lifecycle marketing:
  - Tools like Klaviyo or Mailchimp offer powerful automation but require technical knowledge to build complex flowcharts.
  - Shopify's abandoned cart recovery is rule-based and lacks contextual understanding.
  - OHC Opportunity: By leveraging the existing `CustomerSuccessWorker` and `ohc_universal_ledger`, we can build an autonomous lifecycle engine. The agent monitors customer activity (last order date, subscription status, interaction history), identifies lifecycle transitions (e.g., Active -> At-Risk), and automatically drafts personalized outreach in the `shared_tasks` table for 1-tap owner approval. This fulfills the "unclear work -> clear next action in minutes" promise.

  **Design Doc**
  - **Architecture diagram (Mermaid.js)**
  ```mermaid
  flowchart TD
      subgraph Events & State
          Ledger[(Universal Ledger)]
          Customers[(Customers Table)]
      end

      subgraph AI Job Queue
          Cron[Daily Lifecycle Cron] --> JobQueue[AI Job Queue]
      end

      subgraph Customer Success Dept
          Worker[Lifecycle Engagement Worker]
          LLM[LLM / Intent Resolution]
      end

      subgraph Owner Feed
          Feed[Shared Tasks / Action Cards]
      end

      JobQueue --> Worker
      Worker <--> Ledger
      Worker <--> Customers
      Worker <--> LLM
      Worker --> Feed
  ```

  - **Mobile UX flow (375px first)**
    1. Owner receives a notification: "The Ambassador drafted 3 check-in messages for dormant clients."
    2. Owner taps notification, opening the Agent Feed.
    3. Feed shows 3 Action Cards (e.g., "Draft to Sarah: It's been 6 months since your last lesson!").
    4. Owner taps "Approve" -> Message sent via preferred channel (Email/SMS).
    5. Owner taps "Edit" -> Modifies text, then sends.

  - **AI agent integration points**
    - The worker queries the DB for customers needing engagement (e.g., no activity in 90 days).
    - It retrieves context from the ledger (past purchases, preferred items).
    - It prompts the LLM to draft a highly personalized, context-aware message.
    - Output is saved to `shared_tasks` with `approval_status = PENDING`.

  - **Key design decisions and why**
    - **Proactive vs Reactive**: Moving from reactive event handling (like DMs) to proactive scheduled scans ensures no customer falls through the cracks.
    - **Draft-for-Review**: Maintains owner control and trust. The agent does the heavy lifting (segmentation, drafting), but the owner has the final say.
    - **Contextual Personalization**: Using ledger history ensures messages feel authentic, not like generic blasts.

  **Implementation Prompt**
  **Task:** Implement the Autonomous Lifecycle Engagement Worker in the Customer Success department.
  **CUJ:** Leo the music tutor hasn't seen student Sarah in 3 months. The nightly Lifecycle cron job runs, identifies Sarah as "dormant", reviews her past lesson history (from the ledger), and uses the LLM to draft a personalized email check-in. The draft appears in Leo's mobile Agent Feed as a pending `shared_task` action card for 1-tap approval.
  **Acceptance Criteria:**
  - Create a new background worker (e.g., `lifecycle_engagement_worker.rs`) scheduled to run daily.
  - Implement logic to query the database for customers meeting specific lifecycle criteria (e.g., dormant, post-purchase follow-up).
  - Integrate with the LLM provider to generate personalized drafts based on customer history.
  - Persist the drafted messages as pending tasks in the `shared_tasks` table.
  - Add comprehensive unit tests and E2E tests verifying the workflow from cron trigger to pending action card visibility.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
