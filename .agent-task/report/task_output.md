issue_title: "[Architecture] Universal Autonomous Grant and Funding Engine"
issue_description: |
  ## Title
  Architect and Implement Universal Autonomous Grant and Funding Engine

  ## Problem Statement
  Small business owners like Maya (baker), Carlos (handyman), and Priya (boutique owner) constantly struggle with cash flow and scaling capital. There are billions of dollars in local, state, federal, and private small business grants available annually (especially for minority-owned or female-owned businesses). However, finding these grants, checking eligibility, and writing complex proposals is practically a full-time job. Most non-technical SMB owners don't have the time or expertise to apply, leaving free capital on the table. They need an invisible financial assistant that proactively finds free money they qualify for and does the heavy lifting of applying.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Current Platforms (Shopify Capital, Stripe Capital):** These provide revenue-based loans and cash advances, but they are *loans* that must be repaid (often with high fees). They do not help businesses secure *free* grant money.
  - **Grant Aggregators (Grants.gov, HelloAlice):** These require manual searching, manual profile creation, and manual essay writing.
  - **The Gap in OHC:** OHC already possesses the business's entire context—revenue history, location, industry, owner demographics, and growth trajectory. By leveraging this data, the OHC AI Swarm can cross-reference live grant databases, autonomously verify eligibility, and draft highly personalized, compelling grant proposals.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      GRANT_DATABASE ||--o{ FUNDING_ENGINE : "Syncs Live Grants"
      TENANT_PROFILE ||--o{ FUNDING_ENGINE : "Provides Context"
      FUNDING_ENGINE ||--o{ AI_FINANCE_AGENT : "Triggers Evaluation"
      AI_FINANCE_AGENT ||--o{ AI_LEGAL_AGENT : "Drafts Proposal"
      AI_LEGAL_AGENT ||--o{ FUNDING_OPPORTUNITY : "Creates Draft"
      TENANT_PROFILE ||--o{ FUNDING_OPPORTUNITY : "Reviews & Submits"

      FUNDING_OPPORTUNITY {
          string id PK
          string tenant_id FK
          string grant_name
          string grant_url
          string status
          jsonb drafted_proposal
      }
  ```

  ### Mobile UX Flow (375px first)
  1. **Triage Feed Alert:** Maya receives an Action Card on her feed: "✨ Finance Dept: We found a $10,000 local bakery grant you qualify for. Tap to review."
  2. **Review Screen:** A clean, translucent card shows:
     - **Grant Info:** The name and amount of the grant.
     - **Why You Qualify:** A bulleted list of reasons Maya is a match (e.g., "Female-owned", "Operating in NY for 2+ years").
     - **Drafted Proposal:** An AI-generated narrative answering the grant's specific questions, based on her OHC profile.
  3. **Action:** Maya can edit the text or simply tap "Approve & Submit Proposal".
  4. **Background Execution:** The AI Swarm finalizes the formatting and submits the application via external APIs (where supported) or generates a finalized PDF for Maya to email.

  ### AI Agent Integration Points
  - **Finance Agent ("The Accountant"):** Scans new grants and cross-references them against the tenant's financial profile.
  - **Legal Agent ("The Protector"):** Drafts the proposal narrative to ensure a professional tone and alignment with the grant's grading rubric.

  ### Key Design Decisions
  - **Zero Trust Isolation:** Grant data must be isolated per tenant so that Maya's financial history is never exposed to another tenant's proposal.
  - **Proactive Not Reactive:** The user doesn't search for grants; the AI finds them and presents a ready-to-sign proposal.

  ## Implementation Prompt
  **To the Implementer:**
  Implement the Universal Autonomous Grant and Funding Engine. Create the `funding_opportunities` table. Build the background job that simulates scanning a grant database and evaluating tenants for eligibility. Implement the multi-agent collaboration (Finance Agent + Legal Agent) to draft a proposal. Create the "Action Card" UI for the Triage Feed and the Review Screen to allow the owner to approve and "submit" the proposal. Ensure comprehensive Playwright E2E testing of the approval workflow.

  ## Priority
  P2 (Medium)

  ## Estimated Scope
  Large
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
