issue_title: "Agentic Dynamic Proposal & Contract Lifecycle Management"
issue_description: |
  # Research Report: Agentic Dynamic Proposal & Contract Lifecycle Management

  ## Problem Statement
  Service-based businesses and agency operators (e.g., Nora the Agency Principal) suffer from a heavily fragmented and manual process when securing new clients. The typical flow requires piecing together multiple tools: a form builder for intake, a word processor for proposal drafting, a specialized e-signature tool for contracts, and a separate invoicing system for the deposit. This fragmentation introduces massive friction, delays closing deals, and demands significant administrative overhead from owners who should be focusing on billable work.

  ## Research Report
  - **Market Context**: Platforms like HoneyBook and Dubsado offer "all-in-one" CRM features for freelancers, but they rely heavily on static templates and manual data entry by the user. Enterprise tools like PandaDoc or DocuSign are powerful for contracts but disconnected from the initial lead intake and final invoicing steps unless complex Zapier integrations are maintained.
  - **The OHC Opportunity**: OHC can eliminate this multi-tool chaos by introducing an end-to-end, agent-driven proposal and contract lifecycle. Instead of the owner writing a proposal from scratch, the **Sales Assistant Agent** reads the intake form or initial client DM, drafts a customized proposal, automatically generates the contract with dynamic variables, and embeds the deposit payment link—all seamlessly integrated into the OHC feed.
  - **Competitor Gaps**:
    - *HoneyBook/Dubsado*: Template-heavy, requires manual customization for each lead. Not truly agent-driven.
    - *Shopify/Wix*: Built for products/standardized services, lack robust custom proposal and contract capabilities.
    - *DocuSign/PandaDoc*: Purely for execution, lacking context of the project scope and disconnected from the payment/operations workflow.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Client Intake Form / DM] -->|Webhook| B(Omnichannel Gateway)
      B --> C[Sales Assistant Agent]
      C -->|Query Scope & Pricing| D[Tenant Knowledge Base / RAG]
      C -->|Draft Document| E[Proposal & Contract Engine]
      E --> F[Stripe Integration - Deposit Link]
      E --> G[Action Required Queue]
      G --> H[Mobile App Feed 375px]
      H -->|1-Tap Approve| I[Client Facing Portal]
      I -->|E-Sign & Pay| J[Operations Assistant Agent]
      J --> K[Project Setup & Invoice Scheduled]
  ```

  ### Mobile UX Flow (375px First)
  - **Owner Feed**: A card appears: "New lead from Acme Corp. Proposal drafted."
  - **Interaction**: The owner taps the card to review the AI-generated proposal summary, scope, and auto-calculated price.
  - **Action**: The owner can tap "Edit Scope", "Adjust Price", or the primary button "Approve & Send".
  - **Client Experience**: The client receives a single, mobile-optimized link containing the beautiful proposal, the legally binding contract (e-signature), and the Stripe checkout for the initial deposit, all on one continuous page.

  ### Key Design Decisions
  - **Dynamic Generation over Static Templates**: The Sales Agent uses the tenant's past successful proposals and pricing rules (via RAG) to generate a bespoke document, reducing owner editing time to near zero.
  - **Unified Client Action**: Proposal acceptance, contract signing, and deposit payment are consolidated into a single client-facing transaction to maximize conversion rates.

  ### AI Agent Integration Points
  - **Sales Assistant**: Triggers upon lead intake. Generates scope, pricing, and contract terms based on context.
  - **Operations Assistant**: Triggers upon client signature and payment. Automatically creates project tasks, notifies the team, and schedules future invoice reminders.

  ## Implementation Prompt
  **User-Facing Outcome**: When Nora receives a project inquiry, she opens OHC to find a complete proposal, contract, and deposit invoice already drafted. She taps "Approve", and the client receives a unified link to sign and pay in one step. Once paid, the project automatically kicks off in her operations feed.

  **CUJ & Acceptance Criteria**:
  1. An API endpoint simulates a new client inquiry with project requirements.
  2. The Sales Assistant Agent parses the requirements, drafts a proposal document entity, attaches standard contract terms, and generates a Stripe deposit link.
  3. The drafted proposal appears as an action card in the Owner's mobile feed (tested at 375px width).
  4. The owner clicks "Approve & Send".
  5. The system generates a public, shareable URL for the client.
  6. E2E tests verify the full flow: Inquiry ingestion -> AI Draft -> Owner Approval -> Client Link Generation.

  **Priority**: P1
  **Estimated Scope**: Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
