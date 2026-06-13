issue_title: "[architecture] Proactive Autonomous AI Operations & Decision Mesh"
issue_description: |
  # Research Report: Proactive Autonomous AI Operations & Decision Mesh

  ## 1. Executive Summary
  This document details the architectural design for a "Proactive Autonomous AI Operations & Decision Mesh." While OHC currently has agents that *react* to events (e.g., a customer sends a message -> the CS Agent drafts a reply), the core product vision requires an assistant that *proactively* coordinates work, identifies risks, and suggests decisions before the owner has to ask. Small business owners like Jun (Location Manager) or Nora (Agency Principal) lack the time to dig through analytics dashboards. They need a system that constantly analyzes their operations in the background and surfaces a unified, prioritized "Owner Feed" of recommended next actions.

  ## 2. Market Context & Pain Points
  - **Jun (Location Manager):** Needs to know immediately if there's a spike in pickup complaints or if staffing is inadequate for an upcoming busy period. He shouldn't have to check a dashboard; the system should tell him "You have 30% more pre-orders for Friday than usual, do you want me to text your part-time staff to ask if they can cover an extra shift?"
  - **Nora (Agency Principal):** Needs to know which client projects are stalling. The system should tell her "Client X hasn't approved the proposal sent 3 days ago. Would you like me to draft a follow-up email?"
  - **The Gap:** Traditional SMB software (Shopify, Quickbooks, Calendly) relies on passive dashboards and raw notifications. They tell you *what* happened, but not *what to do about it*. The OHC assistant must bridge this gap by turning raw data into proactive, agent-driven workflows.

  ## 3. Architecture Design
  ### 3.1 Conceptual Flow
  ```mermaid
  graph TD;
      subgraph Event Streams
          E1[Orders/Bookings]
          E2[Messages/Reviews]
          E3[Inventory Levels]
          E4[Calendar/Tasks]
      end

      E1 & E2 & E3 & E4 --> BQ[Background AI Job Queue (PostgreSQL SKIP LOCKED)];
      BQ --> DA[Decision Agent / Operations Agent];

      subgraph Proactive AI Mesh
          DA -->|Analyze Trends/Anomalies| TM[Tenant Memory/Context];
          DA -->|Identify Risk/Opportunity| AG[Action Generator];
          AG -->|Generate Next Action Proposal| OF[Owner Feed];
      end

      OF --> UI[Mobile Dashboard 375px];
      UI -->|Owner Approves Action| EX[Execution Engine (e.g., Send SMS, Create Task)];
  ```

  ### 3.2 Key Decisions
  - **Agentic Background Processing:** Instead of just simple crons, we need an asynchronous, LLM-powered background processing loop. The "Decision Agent" runs periodically or triggered by specific thresholds, analyzing the aggregate state of the tenant's business.
  - **The Owner Feed:** The primary UI shifts from a scattered list of menus to a centralized "Work Feed." This feed contains actionable cards (e.g., "Draft Reply," "Review Schedule," "Approve Follow-up").
  - **Tenant Memory Integration:** Proactive suggestions rely heavily on the AI understanding the specific context of the business (e.g., knowing what "normal" Friday volume is for Jun).
  - **Human-in-the-Loop:** Crucially, the AI *suggests* and *drafts* the complex actions, but the owner must *approve* them (especially for external communications or spending money), maintaining trust and control.

  ## 4. Implementation Prompt
  Implement the Proactive Autonomous AI Operations & Decision Mesh.
  - **User-Facing Outcome:** The owner opens the OHC app and sees a prioritized feed of what needs their attention, along with drafted actions they can approve with one tap.
  - **CUJ:** Jun opens his app on a Thursday. He sees a card at the top of his feed: "Unusual Volume Alert: You have 15% more orders scheduled for tomorrow morning than average. Suggestion: Ask Sarah and Mike if they can start 1 hour early." Jun taps "Approve," and the Operations Agent sends an SMS to the staff.
  - **Acceptance Criteria:**
    - Develop the background job worker that triggers proactive analysis based on events or schedules.
    - Create the centralized "Owner Feed" UI component optimized for mobile (375px), replacing or enhancing the current dashboard.
    - Implement a mechanism for AI Agents to generate "Actionable Proposals" that are persisted in the database and surfaced in the feed.
    - Ensure strict multi-tenant isolation in the background processing queues and memory access.

  ## 5. Metadata
  - **Priority:** P0
  - **Estimated Scope:** Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
