issue_title: "Lead & Opportunity Lifecycle Engine: Closing the Intake-to-Revenue Gap"
issue_description: |
  ### Problem Statement
  Currently, OneHumanCorp (OHC) has a powerful **Work Intake** system (Triage/Inbox) and a **Revenue** system (Orders/Payments/Quotes), but the critical middle layer—**Lead & Opportunity Management**—is missing.

  For owners like **Nora (Agency Principal)** and **Carlos (Handyman)**, an inquiry in the DM doesn't immediately become a "Quote" or an "Order." It starts as a "Lead" that needs to be nurtured into an "Opportunity" (a project pipeline). Without a structured way to track these, inquiries fall through the cracks, and agents have no centralized "Pipeline" to coordinate on.

  ### Research Report
  - **Shopify & Wix Analysis**: Shopify Inbox focuses on immediate chat-to-cart, which is great for Maya (Baker) but poor for service-based owners like Carlos or Nora. Wix CRM provides a "Lead Table" and "Deal Pipeline" that allows owners to see the dollar value of their "Work in Progress."
  - **Gap Identified**: OHC lacks a tenant-scoped `Opportunity` entity. Currently, the `SalesAgent` drafts quotes directly from messages, which is too aggressive for high-consideration projects (e.g., design projects for Nora).
  - **Proposed Solution**: Introduce a "Pipeline Card" system where `CustomerSuccess` qualifies a lead, `Sales` builds an opportunity/proposal, and `Finance` tracks the potential revenue—all visible in a translucent, mobile-first dashboard.

  ### Design Doc
  #### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ LEAD : "acquires"
      TENANT ||--o{ OPPORTUNITY : "manages"
      LEAD ||--o| OPPORTUNITY : "converts_to"
      OPPORTUNITY ||--o{ QUOTE : "has"
      OPPORTUNITY {
          string id
          string title
          string stage "Qualified, Proposal, Negotiation, Won, Lost"
          double estimated_value
          string priority "Low, Medium, High"
      }
      LEAD {
          string id
          string source "Instagram, WhatsApp, Web"
          string contact_info
          string context
      }
  ```

  #### Sequence Diagram: Intake to Pipeline
  ```mermaid
  sequenceDiagram
      participant C as Customer
      participant CS as CustomerSuccess Agent
      participant S as Sales Agent
      participant O as Owner (Dashboard)

      C->>CS: "Do you do custom branding?" (WhatsApp)
      CS->>CS: Create Lead
      CS->>S: Notify: "New Branding Lead"
      S->>S: Create Opportunity (Stage: Qualified)
      S->>O: Push "Pipeline Card" to Triage
      O->>S: Approve: "Draft Proposal"
      S->>C: Send Proposal Quote
  ```

  #### Mobile UX Flow (375px)
  1. **Triage Feed**: A new card type "Opportunity: Branding Project" appears.
  2. **Pipeline View**: A horizontal-scrolling card tray showing cards grouped by stage (Qualified, Proposal, etc.).
  3. **Interaction**: Swiping a card moves it to the next stage. Tapping a card opens the "Agent Thread" for that specific deal.

  ### Implementation Prompt
  **Outcome**: Implement the `Lead` and `Opportunity` data structures and a "Pipeline" dashboard view.

  **Acceptance Criteria**:
  1. **Core API**: Create CRUD for `leads` and `opportunities` with strict RLS (multi-tenant).
  2. **Agent Integration**: Update `CustomerSuccessAgent` to automatically create a `Lead` when a high-intent message is received.
  3. **Agent Integration**: Update `SalesAgent` to transition an `Opportunity` stage when a quote is drafted/approved.
  4. **Frontend**: Add a "Pipeline" tab to the Dashboard. On mobile (375px), this should render as a series of premium translucent cards that can be swiped between stages.
  5. **Verification**: E2E test where a message from a "New Customer" results in a visible "Pipeline Card" in the UI.

  ### Priority & Scope
  - **Priority**: P1 (High)
  - **Estimated Scope**: Large (Full stack + Agent logic)

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
