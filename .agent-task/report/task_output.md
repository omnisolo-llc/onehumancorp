issue_title: "[Research] OHC Autonomous Lead Generation & Conversion System"
issue_description: |
  # Research Report: Autonomous Lead Generation & Conversion System

  ## 1. Problem Statement
  For service-based SMBs (like Carlos the Handyman or Nora the Agency Principal), capturing and nurturing leads is a highly manual, error-prone process. A customer visits the website, fills out a generic contact form, and waits. If Carlos is on a roof, he misses the notification, the lead gets cold, and he loses the job. Traditional tools require configuring complex CRM pipelines, setting up zapier integrations, and manually drafting follow-ups.

  ## 2. Research Report
  - **Market Context:** Existing platforms like Wix or Squarespace provide static forms. HubSpot offers powerful lead generation but at a high cost and complexity barrier for solopreneurs. Durable captures leads but lacks autonomous follow-up.
  - **The Gap:** SMB owners need a system that doesn't just capture a name and email, but actively engages the lead, qualifies them, provides initial estimates based on business rules, and automatically follows up if they drop off.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
    TENANT ||--o{ LEAD : captures
    LEAD ||--o{ INTERACTION : has
    TENANT ||--o{ ESTIMATE_RULE : defines
    LEAD }|--|| CUSTOMER : matches

    TENANT {
      uuid tenant_id PK
      string name
    }
    LEAD {
      uuid id PK
      uuid tenant_id FK
      string source
      string status
    }
    INTERACTION {
      uuid id PK
      uuid lead_id FK
      uuid tenant_id FK
      string channel
    }
    ESTIMATE_RULE {
      uuid id PK
      uuid tenant_id FK
      string logic
    }
  ```

  ### Data Model Guidelines
  - Data models must adhere strictly to the `tenant_id` pattern. Row-level tenant isolation in PostgreSQL using `tenant_id` on every table must be enforced using `ENABLE ROW LEVEL SECURITY`.

  ### AI Agent Integration
  - **The Ambassador (Customer Success/Sales):** Instantly engages new form submissions via email/SMS. Asks qualifying questions ("Can you describe the leak?"). Uses RAG against estimate rules to provide a rough quote.
  - **The Promoter (Marketing):** Identifies stalled leads and automatically drafts a re-engagement email with a small incentive.

  ### Mobile UX Flow (375px)
  1. **Owner View (Lead Inbox):** A Tinder-like swipe interface or clean list view for new leads.
  2. **Detail Card:** Shows lead info, AI-summarized context ("Wants sink fixed, quoted $200"), and AI-drafted next message ("Approve & Send: Hi, I can come tomorrow at 2 PM.").

  ## 4. Implementation Prompt
  **Feature Name:** OHC Autonomous Lead Generation & Conversion System
  **Target Persona:** Carlos the Handyman
  **Outcome:** A native form component that feeds directly into an Agentic Lead Pipeline. The AI automatically replies to inquiries, provides rough estimates based on Carlos's pricing rules, and surfaces actionable cards to his mobile app.

  **Next Actions:**
  1. Implement the data models for the lead, interactions, and estimate rules with RLS for multi-tenant isolation.
  2. Create a "Smart Form" block for the website builder that connects to the lead pipeline.
  3. Extend The Ambassador agent to process new leads, generate estimates, and draft responses.
  4. Build the mobile Lead Inbox UI with swipe/approve actions.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
