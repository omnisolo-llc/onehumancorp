issue_title: "OHC-04: Customer inquiry to qualified quote automation"
issue_description: |
  ### Title
  OHC-04: Customer inquiry to qualified quote automation for service professionals

  ### Problem Statement
  Service professionals like Nora (solo web/design/marketing) spend significant unbilled hours fielding inbound inquiries, qualifying leads, and drafting proposals. When busy, they miss leads or delay responses, causing lost revenue. Existing solutions are either too complex (overwhelming CRM setup) or require manual drafting for every unique project, breaking their flow.

  ### Research Report
  - **Context**: The `OHC-04` capability aims to take a real inbound message and create one lead, a grounded response, and a priced quote with delivery evidence.
  - **Market Baselines**:
    - **HoneyBook**: Automates inquiry capture via contact forms and sends brochure/pricing guides, but relies on templated static responses rather than context-aware AI drafting tailored to the specific lead.
    - **Claude for Small Business**: Can draft a proposal if the owner pastes the email and context, but lacks the structured workflow to automatically turn an inbox thread into a tracked, priced quote in a system of record.
  - **User Voices**: "I lose 5-10 hours a week just emailing back and forth trying to figure out what a client needs before I can even send a price. By the time I send the proposal, they've often moved on." (Observed pain point for solo service providers).
  - **OHC Gap**: The current `src/server/api/proposals.rs` has basic structural elements for proposals but requires an automated, AI-driven workflow that can ingest an inquiry, use the owner's offer context (from OHC-03), and generate a qualified, priced quote deterministically without fabricating scope or unauthorized commitments.

  ### Design Doc
  - **Architecture**:
    - **Inquiry Ingestion**: Service receives an inbound message (email/form).
    - **Context Retrieval**: Fetches business offer constraints and pricing rules (tenant context).
    - **AI Qualification**: Uses LLM to assess if the inquiry matches the business's capabilities.
    - **Proposal Draft**: Generates a `Proposal` entity with `project_scope` and calculated `total_amount_cents` based strictly on the owner's business rules.
  - **UI Wireframes (375px mobile-first)**:
    - **Inbox View**: Shows new inquiries with a "Qualified" badge.
    - **Draft Proposal View**: Shows the AI-generated response, scope, and price. Contains explicit "Approve & Send" and "Edit" buttons.
  - **AI Agent Integration**: The AI team must act within standing authority—it drafts the quote based on approved rules, but requires owner approval (or operates within strict bounds if pre-authorized) before sending the final priced commitment.

  ```mermaid
  graph TD
    A[Inbound Inquiry] --> B[Retrieve Tenant Offer Context]
    B --> C[AI Agent Drafts Response & Quote]
    C --> D[Draft Proposal Created]
    D --> E{Owner Review}
    E -->|Approve| F[Send Quote to Client]
    E -->|Edit| C
  ```

  ### Implementation Prompt
  Implement the OHC-04 workflow where an inbound inquiry automatically generates a drafted, priced proposal based on the owner's configured business rules. The system must not authorize or send the quote without explicit owner approval. The implementation must use real provider test sessions or explicitly model unavailable states, without generating fake checkout URLs. Ensure the draft preserves actual provider counts and missing-usage states. Ensure no unauthorized commitments or fabricated scope are created.

  ### Priority
  P1

  ### Estimated Scope
  Medium

  ### Strategy Admission
  - **OHC Target ID**: OHC-04
  - **Stage**: Launch
  - **Observed/Inferred Gap**: Inferred gap in automated context-aware quoting compared to manual templates.
  - **Evidence Level**: Documented
  - **Baseline and Measurable Result**: Reduce time-to-quote from hours to minutes; denominator is total qualified inquiries.
  - **Dependencies/Reuse**: Depends on OHC-03 (offer context). Reuses existing `Proposal` data structures.
  - **Non-goals**: Not building a full marketing CRM or complex multi-party negotiation tool.
  - **Authority Class**: Draft-only; explicit owner review required before external commitment.
  - **Cost/Measurement Plan**: Track LLM usage per inquiry, success rate of automated qualification.
  - **Acceptance Checks**: Real inbound message creates one lead, grounded response, and priced quote; follow-up has delivery evidence; deterministic validated amounts; no unauthorized commitments.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []