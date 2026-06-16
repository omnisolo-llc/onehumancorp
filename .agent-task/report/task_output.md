issue_title: "Hierarchical Multi-Location Task Coordination & AI Escalation (Jun Persona)"
issue_description: |
  # Research Report: Hierarchical Multi-Location Task Coordination & AI Escalation

  ## 1. Executive Summary
  This report details the architectural design for a multi-location coordination and AI-driven issue escalation system in OneHumanCorp (OHC). The primary focus is on empowering the "Jun" persona (Location Manager) who manages day-to-day operations at a specific site but needs seamless coordination with staff and the regional/overall business owner. Current SMB platforms fail to provide structured, agent-assisted escalation paths, often leaving location managers overwhelmed and owners blind to local issues until it's too late.

  ## 2. Market Mapping & Competitor Discovery (Track 1)
  - **Competitor Gaps**: Traditional platforms (Shopify, Wix, Squarespace) are heavily focused on single-entity e-commerce or simple bookings. They lack hierarchical structures that delineate a "Location Manager" from an "Owner."
  - **SaaS Solutions**: Tools like Homebase or 7shifts handle scheduling but lack the AI agentic workflows that OHC provides (e.g., auto-drafting an escalation report when a supply shortage occurs).
  - **The Opportunity**: OHC can differentiate by offering a "Manager Mode" where agents act as intermediaries between local staff, the location manager, and the owner, turning raw operational data into actionable, owner-ready summaries.

  ## 3. OHC Gap & Pain Point Identification (Track 3)
  - **Persona Focus**: Jun (Location Manager, 31). He runs one location of a larger operation.
  - **The Gap**: Jun currently has no structured way within OHC to coordinate local staff tasks or intelligently escalate critical issues (e.g., a spike in customer complaints or a broken espresso machine) to the owner without manually writing reports or making frantic phone calls.
  - **Pain Points**:
    - Staff coordination is manual and disconnected from the OHC agent ecosystem.
    - Escalation to the owner is noisy, lacking context and data.
    - No clear boundary between local (Jun's purview) and global (Owner's purview) settings.

  ## 4. Deep Dive Architecture Design (Track 2 & Track 3)

  ### Data Model & Hierarchy
  - **Tenant & Location Entities**: A `Tenant` can have multiple `Locations`. Users can be assigned roles (`Owner`, `Location Manager`, `Staff`) scoped to a specific `Location_ID`.
  - **Task & Escalation Records**:
    - `Task`: Assigned to `Staff` by `Location Manager` or `Operations Agent`.
    - `Escalation`: A specialized record linking an issue (Task, Customer Feedback, Inventory Alert) to an `Owner` review request, complete with an AI-generated summary.

  ### Entity-Relationship Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ LOCATION : "has"
      USER ||--o{ ROLE_ASSIGNMENT : "has"
      LOCATION ||--o{ ROLE_ASSIGNMENT : "scopes"
      LOCATION ||--o{ TASK : "contains"
      TASK ||--o{ ESCALATION : "may_trigger"
      USER ||--o{ ESCALATION : "reviews"

      TENANT {
          uuid id
          string name
      }
      LOCATION {
          uuid id
          uuid tenant_id
          string name
      }
      USER {
          uuid id
          string name
      }
      ROLE_ASSIGNMENT {
          uuid id
          uuid user_id
          uuid location_id
          string role "Owner | Manager | Staff"
      }
      TASK {
          uuid id
          uuid location_id
          string status
      }
      ESCALATION {
          uuid id
          uuid task_id
          string summary
          string status
      }
  ```

  ### AI Agent Coordination
  - **Operations Agent**: Monitors local task completion and supply levels. If a task is blocked or supplies run low, it alerts Jun.
  - **Customer Success Agent**: Analyzes local customer feedback. If sentiment drops or a specific complaint spikes (e.g., "pickup took 30 mins"), it flags Jun and prepares an escalation draft.
  - **The Escalation Workflow**:
    1. Jun receives an alert from an Agent.
    2. Jun clicks "Escalate to Owner".
    3. The Agent drafts a summary ("Location A is experiencing a 30% increase in wait times due to a broken POS terminal. Recommend authorizing $500 for emergency repair.").
    4. Jun reviews, edits, and sends the summary.
    5. The Owner sees a high-priority "Escalation" card in their Work Triage feed.

  ### Mobile-First UX Flow (375px)
  - **Jun's Dashboard**: Focuses on "Today's Local Tasks," "Staff on Shift," and "Active Alerts."
  - **Escalation UI**: A simple modal. "What's the issue?" -> Agent generates draft -> "Send to Owner".
  - **Owner's View**: A consolidated "Regional Summary" card showing the health of all locations, with red badges for pending escalations.

  ## 5. Implementation Prompt

  **Feature Name:** Location Manager Coordination & Escalation Workflow

  **Target Persona:** Jun (Location Manager) and the Business Owner

  **Outcome:** Jun can view local tasks, receive AI alerts about local issues, and use the Operations Agent to draft and send a contextual escalation report to the Owner's unified feed.

  **Critical User Journey (CUJ):**
  1. Jun logs into the OHC app (scoped to his Location).
  2. The Operations Agent flags a recurring issue: "3 customer complaints regarding slow pickup in the last hour."
  3. Jun taps the alert and selects "Escalate to Owner".
  4. The Agent drafts a concise summary: "Spike in pickup complaints at Location A. Staffing appears adequate, but the kitchen printer is offline. Requesting IT support."
  5. Jun approves the draft.
  6. The Owner logs in and sees the Escalation in their Work Triage feed, with actions to "Approve IT Request" or "Message Jun".

  **Next Actions for Engineering:**
  - **Step 1:** Implement the `Location` scoping in the data model and update role-based access control (RBAC) to support the "Location Manager" role.
  - **Step 2:** Create the `Escalation` entity and the corresponding agentic workflow for drafting and routing these summaries.
  - **Step 3:** Build the mobile-first (375px) UI for Jun's local dashboard and the Owner's escalation feed card.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
