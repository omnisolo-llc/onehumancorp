issue_title: "[Architectural Design] Autonomous Client Intake & Proposal Generation"
issue_description: |
  # Architectural Design: Autonomous Client Intake & Proposal Generation

  ## Problem Statement
  Service professionals, freelancers, and small agency owners (e.g., Nora the Agency Principal) lose countless hours managing the friction of client intake. When a new prospect reaches out, the owner must manually gather requirements, draft a custom proposal, estimate costs, request approvals, and handle the back-and-forth negotiation. Existing tools (like HoneyBook or Dubsado) offer templates but still require heavy manual input and lack deep integration with an AI that can understand the context of the business's past work and standard pricing.

  ## Research Report & Competitive Analysis
  - **Market Context**: Platforms like HoneyBook, Dubsado, and Bonsai provide CRM and proposal software for freelancers, but they are passive template engines. They do not autonomously draft content based on conversational intake.
  - **The OHC Opportunity**: OHC can transform intake from a "form filling" exercise to an "Agentic Conversation." By deploying the Sales & Knowledge Agents, OHC can capture intent from an Instagram DM or a short web inquiry, cross-reference past successful projects, and instantly draft a personalized proposal for the owner's review.
  - **Competitor Gaps**:
    - *HoneyBook / Dubsado*: Excellent workflow automation, but the user must still write the proposal and configure complex logic flows.
    - *Shopify*: Not built for service/custom proposal businesses.
    - *Notion AI*: Good for drafting text, but disconnected from invoicing and payment gateways.

  ## Design Doc

  ### 1. Data Model (PostgreSQL with Row Level Security)
  The intake and proposal system requires the following core entities, strictly isolated by `tenant_id`:
  - `IntakeRequest`: Captures the initial lead. Attributes: `client_id`, `source` (Web, DM, Email), `raw_intent`, `structured_requirements` (JSONB).
  - `Proposal`: The generated proposal. Attributes: `intake_request_id`, `status` (draft, sent, approved, rejected), `total_amount_cents`, `content` (Markdown/HTML), `expires_at`.
  - `ProposalLineItem`: Breakdown of the proposal. Attributes: `proposal_id`, `description`, `amount_cents`.
  - `Project`: Created automatically upon proposal approval.

  *Multi-Tenant Invariant*: Every table must have a `tenant_id` column and `ENABLE ROW LEVEL SECURITY`.

  ### 2. AI Department Coordination
  - **Sales Assistant ("The Closer")**:
    - Parses the raw `IntakeRequest` using LLMs to extract budget, timeline, and deliverables.
    - Drafts the `Proposal` and `ProposalLineItem`s by comparing the request against the tenant's historical pricing and services.
  - **Knowledge Assistant ("The Librarian")**:
    - Supplies the Sales Assistant with past successful proposals and standard terms/conditions from the owner's knowledge base to ensure consistency.
  - **Finance Assistant**:
    - Monitors the `Proposal` status and automatically generates the deposit invoice once the client clicks "Approve."

  ### 3. System Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Prospect
      participant OHC_Triage as Work Triage
      participant Sales_Agent as Sales Agent
      participant DB as PostgreSQL (Ledger)
      participant Owner as Nora (Owner)

      Prospect->>OHC_Triage: Submits Inquiry (DM/Form)
      OHC_Triage->>DB: Create `IntakeRequest`
      OHC_Triage->>Sales_Agent: Trigger Proposal Draft
      Sales_Agent->>DB: Fetch past proposals & pricing
      Sales_Agent->>Sales_Agent: Generate `Proposal` draft
      Sales_Agent->>DB: Save `Proposal` (Status: Draft)
      Sales_Agent->>Owner: Agent Feed: "Draft proposal ready for Prospect"
      Owner->>Sales_Agent: Reviews, edits, clicks "Approve & Send"
      Sales_Agent->>Prospect: Emails Proposal link
  ```

  ### 4. Mobile-First UX Flow (375px Target)
  - **Owner View (Agent Feed)**: Nora receives a card in her feed: *"New lead from Acme Corp. I've drafted a $5,000 proposal for the website redesign based on our standard rates. Review?"*
  - **Review UI**: A single touch-friendly screen showing the generated line items. Nora can tap any item to edit the price or description using native mobile inputs.
  - **Client View**: A clean, responsive mobile web view where the prospect can read the proposal, sign digitally, and pay the deposit via Stripe in one continuous flow.

  ## Implementation Prompt

  **Objective**: Implement the backend foundation and core APIs for the Autonomous Client Intake and Proposal Generation system.

  **Persona Outcome**: Nora the Agency Principal needs to receive an inquiry, have the AI automatically draft a proposal with line items, and be able to review/send it from her phone without writing it from scratch.

  **Required Steps**:
  1. Define the Protocol Buffer definitions (`src/proto/`) for `IntakeRequest`, `Proposal`, and `ProposalLineItem`, including RPCs for creation and status updates.
  2. Implement the PostgreSQL database migrations and repository layer with strict RLS (Row Level Security) and `tenant_id` enforcement.
  3. Implement the `Sales Agent` capability to listen for new `IntakeRequest` events and automatically generate a draft `Proposal` using the LLM provider.
  4. Create the gRPC/Axum service handlers for the owner to retrieve, edit, and send the proposal.
  5. **MANDATORY**: Create a Playwright E2E test (`src/e2e/proposal.spec.ts`) that simulates an incoming intake request, verifies the agent drafted a proposal, logs in as the owner, approves the proposal, and verifies the client-facing view.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
