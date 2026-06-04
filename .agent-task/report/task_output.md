issue_title: "[Research] Autonomous CRM & Loyalty Engine Architecture"
issue_description: |
  # Issue Brief: Autonomous CRM & Loyalty Engine Architecture

  ## Problem Statement
  Small business owners currently lack the time and expertise to manually manage Customer Relationship Management (CRM) workflows and loyalty programs. The process of tracking customer interactions across channels, maintaining profiles, evaluating loyalty points, and sending personalized re-engagement campaigns is manual, disjointed, and prone to error. They require an "Autonomous CRM & Loyalty Engine" to act as an invisible backend teammate.

  ## Research Report
  Our research into small business workflows and competitive products (such as Shopify, Wix, GoDaddy) indicates that a major gap exists between complex enterprise CRM tools and the simplistic "Contacts" lists offered by traditional SMB platforms. The "Autonomous Customer Lifecycle & Loyalty Engine Research Report" (see `docs/reports/autonomous_crm_loyalty_engine_research_report.md`) validates the need for an automated background architecture.

  Key Findings:
  - Non-technical business owners are overwhelmed by manual CRM tasks.
  - Integration of disparate communication channels is essential.
  - A proactive, agentic approach to CRM is required to drive revenue and retention without manual user input.

  ## Design Doc
  **High-Level Architecture & Key Components:**
  1. **Customer360 Profile:** A central hub aggregating purchase history, lifetime value (LTV), preferences, tags, and contact info. Real-time updates by AI agents.
  2. **InteractionTimeline:** A unified chronological view of all touchpoints (site visits, support messages, purchases). Acts as context memory for the AI Customer Success Ambassador.
  3. **LoyaltyLedger:** An event-sourced ledger to autonomously track and manage loyalty points, tier statuses, and rewards based on customizable rules.

  **Architecture Diagram (Mermaid):**
  ```mermaid
  graph TD
      A[Customer Actions: Purchase, Support, Web Visit] --> B(InteractionTimeline)
      B --> C[Customer360 Profile Aggregation]
      A --> D(LoyaltyLedger - Event Sourced)
      C --> E[AI Agents: Operations, CS, Sales]
      D --> E
      E --> F[Automated Campaigns & Insights]
  ```

  **Mobile UX Flow:**
  - View Customer Profile: 375px optimized card showing LTV, recent interactions, and AI-generated insights.
  - Approval Interface: The CRM Agent proposes an action (e.g., "Send 10% off to [Customer] who hasn't purchased in 60 days"). User simply taps "Approve".

  ## Implementation Prompt
  **Goal:** Implement the backend architecture for the Autonomous CRM & Loyalty Engine, encompassing `Customer360`, `InteractionTimeline`, and `LoyaltyLedger`.
  **User Persona:** Maya the Baker (non-technical).

  **Acceptance Criteria:**
  - Create the data models/schemas required for `Customer360`, `InteractionTimeline`, and an event-sourced `LoyaltyLedger`. Ensure row-level tenant isolation.
  - Implement a backend service to ingest customer events and update the `InteractionTimeline` and `LoyaltyLedger`.
  - Design the architecture such that the AI Customer Success Agent can query the `Customer360` profile and `InteractionTimeline` for context.
  - Provide an endpoint for the mobile app to fetch the synthesized customer profile and AI-proposed actions.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
