issue_title: "Research: Inquiry to Proposal Workflow and Payment Gap"
issue_description: |
  # Research Report: Inquiry to Proposal Workflow

  **Target ID:** OHC-02 (Inquiry capture, qualification and an approved proposal)

  ## Problem Statement
  Small business owners (like Nora, a solo web/design/marketing professional) need an efficient way to turn inbound inquiries into approved proposals and payment requests. Currently, the OHC codebase has gaps in integration readiness, and the exact process for reliably generating an accurate proposal from an inquiry without requiring heavy manual owner supervision or running outside of explicit standing authority needs to be designed. Wait time for owners between inquiry and closing creates lead drop-off.

  ## Research Report
  ### Current OHC Baseline
  Based on `docs/research/native_migration_and_remediation.md`:
  - **F07 (Fabricated proposal terms):** The system previously fabricated $5,000 scope/deposit regardless of inquiry. This was fixed (Closed) to use input/approved-business-rule driven drafts, deterministic validated amounts, and no unauthorized commitments.
  - **F08 (Fabricated checkout links):** The system previously fabricated checkout-looking URLs. This was fixed (Closed). Real provider session or explicit pending/unavailable state is used.
  - **F11 (Smoke tests labeled full journey):** Open. Need actual mutation/state/provider-boundary acceptance tests without live credentials.
  - **F12 (False completion/authority):** Open. Need truthful states/receipts; exact authority, stale approval/revocation and reconciliation checks on affected paths.

  ### Competitor Analysis & Benchmarks
  We reviewed established tools in this space. While live verified usage data across all tools couldn't be obtained, the documented features of leading platforms show a common set of requirements for SMBs:
  - **HoneyBook:** Strong in client flow. Connects inquiry forms directly to a pipeline, allows templates for brochures and proposals, and includes integrated payment processing (Stripe/custom). Users frequently cite the "all-in-one" flow from contact form -> proposal -> contract -> invoice as the main value driver.
  - **Dubsado:** Similar to HoneyBook, but with more complex automation workflows. Allows creating a lead capture form that triggers a workflow sending a proposal, contract, and invoice as a single document/step.
  - **Claude for Small Business (Hypothetical/General AI usage):** Owners using raw AI tools often use them to draft proposal text based on email threads, but must manually copy-paste this into invoicing/contract tools.

  ### User Sentiment & Pain Points (Persona: Nora)
  - **Friction:** Managing the context transfer from "client asks a question via email/form" to "creating a structured, priced proposal" is manual and error-prone.
  - **Pain:** If the owner is busy, leads go cold. If they rush, proposals might have incorrect pricing.
  - **Current Tools:** Traditional CRMs require setting up complex templates and rules.
  - **The "Invisible Agentic" Gap:** The system should securely process the inquiry context, match it against the owner's defined services and pricing rules, draft the proposal, request owner approval (if outside standing authority), and then send a verifiable payment request link (via an established integration like Stripe).

  ## Strategy Admission
  - **OHC Target ID:** OHC-02
  - **Launch/Run stage:** Launch (Initial setup of rules) -> Run (Handling inquiries)
  - **Observed/Inferred Gap:** Inferred gap in full end-to-end reliability for inquiry-to-proposal with correct standing authority enforcement.
  - **Evidence Level:** Documented (competitor benchmarks), Codebase traces (F07, F08 closures).
  - **Baseline:** Manual proposal generation or complex CRM setup.
  - **Measurable Result:** Reduced time-to-proposal (hours -> minutes), measured by the duration between inquiry creation and proposal approval.
  - **Dependencies:** Stripe connection, email/form ingestion, standing authority rules engine.
  - **Non-Goals:** Building a new payment processor, complex multi-party enterprise contracts.
  - **Authority Class:** Drafts require explicit owner approval before sending, unless a specific service has explicit "auto-approve" standing authority defined by the owner.
  - **Cost Plan:** Standard compute + provider LLM costs for drafting.
  - **Acceptance Checks:** Happy path: Inquiry creates accurate draft proposal matching rules, owner approves, secure Stripe link generated. Failure path: Invalid inquiry data creates a flagged task for owner review without generating a bad link.

  ## Design Doc
  ### High-Level Architecture
  1. **Inquiry Ingestion:** Endpoint or webhook receiving customer inquiry (e.g., from a web form or email).
  2. **Context Engine:** Extracts intent, required services, and constraints.
  3. **Pricing & Rules Engine:** Matches extracted needs against owner's defined service catalog and pricing bounds.
  4. **Drafting Agent:** Generates the proposal text and structure.
  5. **Authority Gate:** Checks if the drafted proposal falls within the owner's "auto-send" parameters. If not, it queues for owner approval.
  6. **Payment/Contract Generation:** Upon approval, interfaces with the Stripe integration to generate a real checkout session or invoice link.

  ### Entity Types
  - `Inquiry`
  - `ServiceRule` (Pricing, scope)
  - `ProposalDraft`
  - `ApprovalRequest`
  - `PaymentLink`

  ### Integration Points
  - Stripe API (for checkout/invoice creation)
  - Email/Notification provider (to alert owner or send to client)

  ### Mobile UX Flow (375px first)
  1. **Notification:** "New Inquiry from [Client Name]"
  2. **Review Screen:** Shows original inquiry text and AI-generated proposal draft.
  3. **Action Bar:** "Approve & Send", "Edit", "Reject".
  4. **Edit Screen:** Simple tap-to-edit for price fields or text.
  5. **Confirmation:** "Proposal sent via [Channel]. Waiting for payment."

  ## Implementation Prompt
  Implement the backend workflow for the Inquiry-to-Proposal journey. The system must accept a structured inquiry, use the owner's predefined service pricing rules to draft a proposal, and hold the draft in a pending state for owner approval. Once approved, the system should generate a real Stripe checkout link (reusing the existing fixed Stripe integration).

  **Critical User Journey:**
  1. System receives inquiry.
  2. System drafts a proposal using deterministic pricing rules (not hallucinated prices).
  3. System flags the draft for owner review.
  4. Owner approves the draft.
  5. System creates a verifiable payment request link.

  **Acceptance Criteria:**
  - The generated proposal must strictly adhere to the owner's defined pricing constraints.
  - No proposal is sent to a client without explicit owner approval (respecting the Authority Gate).
  - The payment link generated must be a valid integration link, not a fabricated placeholder URL.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
