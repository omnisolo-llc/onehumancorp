issue_title: "Research: Autonomous Proposal & Milestone Billing Engine"
issue_description: |
  # Research Report: Autonomous Proposal & Milestone Billing Engine

  ## 1. Problem Statement
  Service-based small business owners, agencies, and independent professionals (e.g., Nora the Agency Principal) face significant friction when converting a client's initial inquiry into a scoped project, a formal proposal, and a tracked billing schedule. They currently rely on disjointed tools: email for intake, Google Docs/Word for proposals, Docusign for approvals, and Stripe/QuickBooks for invoicing. This fragmentation causes delayed responses, lost revenue due to forgotten follow-ups, and a lack of integrated project visibility.

  ## 2. Research Report
  - **Market Context**: Platforms like HoneyBook and Dubsado offer workflow automation for freelancers, but they are often complex to set up and lack true AI agency. They require the user to manually build templates and trigger workflows. Traditional e-commerce platforms (Shopify, Wix) treat services as one-off products, failing to support the milestone-based billing and iterative approvals common in service work.
  - **The OHC Opportunity**: By deeply integrating an "Autonomous Proposal & Milestone Billing Engine" within the OHC platform, we can empower our AI assistants (Sales, Operations, Finance) to draft proposals, track project milestones, and issue invoices automatically. This eliminates the "tool tax" and moves OHC from a simple booking system to a comprehensive project management partner.
  - **Competitor Gaps**:
    - *Shopify/Wix*: No native support for multi-step proposals or milestone-based payments.
    - *HoneyBook/Dubsado*: Heavy setup required; passive automation rather than active agentic assistance.
    - *Stripe Billing*: Excellent for subscriptions, but lacks the native project proposal and approval UX required by service providers.

  ## 3. Design Doc
  ### Data Model (PostgreSQL)
  - `Project`: The overarching container linking a Customer to a Service.
  - `Proposal`: The structured document containing scope, terms, and the payment schedule.
  - `Milestone`: Individual deliverables or billing phases within a Project (e.g., "50% Deposit", "Design Approval", "Final Delivery").
  - `Invoice`: The payment request tied to a specific Milestone.

  ### Architecture & Flow Diagram
  ```mermaid
  sequenceDiagram
      actor Client
      actor Nora as Owner (Nora)
      participant SA as Sales Agent
      participant OA as Operations Agent
      participant FA as Finance Agent
      participant OHC as Central Ledger (OHC DB)

      Client->>SA: Submits Intake Inquiry
      SA->>Nora: Drafts Proposal & Milestone Schedule
      Nora->>SA: Edits & Approves Proposal
      SA->>Client: Sends Mobile Proposal Link
      Client->>OHC: Reviews Proposal & Pays Deposit (Stripe)
      OHC-->>OA: Triggers Project Start

      Note over OA, OHC: Milestone 1 Complete
      OA->>FA: Notifies Milestone Complete
      FA->>OHC: Generates & Logs Invoice
      FA->>Client: Sends Milestone Invoice Link
      Client->>OHC: Pays Invoice
  ```

  ### AI Integration
  - **Sales Assistant**: Monitors new client inquiries (e.g., via Work Triage), extracts requirements, and drafts a tailored Proposal. Suggests a milestone billing schedule based on the owner's historical projects or industry standards.
  - **Operations Assistant**: Tracks project progress. Once a milestone is marked complete by the owner (or automatically via integration), it triggers the Finance Assistant.
  - **Finance Assistant**: Automatically generates and sends the Stripe Invoice for the completed Milestone and follows up on unpaid invoices.

  ### Mobile UX Flow (375px)
  1. **Intake & Draft**: The owner sees a new inquiry in their feed. They tap "Draft Proposal." The Sales Assistant presents a generated proposal and milestone schedule. The owner can edit text or amounts using large touch targets.
  2. **Client Approval**: The client receives a mobile-friendly link to view the proposal, accept the terms, and pay the initial deposit via Stripe Checkout.
  3. **Project Tracking**: The owner's dashboard shows active projects. Tapping a project reveals the milestone timeline. Completing a milestone is a single swipe/tap, which visibly queues the next invoice.

  ## 4. Implementation Prompt
  **Feature Name**: OHC Autonomous Proposal & Milestone Billing Engine
  **Target Persona**: Nora the Agency Principal
  **Outcome**: Nora can turn a new client request into a sent proposal in under 2 minutes from her phone. Upon client approval, the system automatically schedules the work tasks and issues milestone invoices as the project progresses, without requiring manual invoice creation.

  **Next Actions**:
  1. Implement the Data Models (`Project`, `Proposal`, `Milestone`, `Invoice`) ensuring strict multi-tenant isolation.
  2. Develop the mobile-first Owner Proposal Drafting Flow and the Client Approval/Deposit Flow (integrating with Stripe).
  3. Enhance the Sales Assistant to draft proposals based on intake context.
  4. Implement the Operations/Finance Agent logic to trigger invoices upon milestone completion.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []