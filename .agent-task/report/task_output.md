issue_title: "OHC-04: Customer inquiry → qualified quote"
issue_description: |
  # Problem Statement
  Small business owners (like Nora, our target persona) currently spend significant manual effort responding to customer inquiries and generating quotes. A successful AI team must be able to ingest a real inbound message, qualify the lead, and generate a grounded, priced quote based on the owner's authorized business rules without unauthorized commitments.

  # Research Report
  - **Segment:** Digital service professionals (web/design/marketing)
  - **Stage:** Launch/Run (OHC-04 slice)
  - **Observed Gap:** The codebase needs a verified slice demonstrating inquiry-to-quote conversion based on connected context (OHC-03).
  - **Evidence:** RESEARCH.md states: "Real inbound message creates one lead, grounded response and priced quote; follow-up has delivery evidence." F07 was closed with: "Input/approved-business-rule driven draft; deterministic validated amounts; no unauthorized commitments or fabricated scope."
  - **Dependencies:** Customer/offer context from OHC-03.
  - **Expected Result:** A verified digital-service loop where an inquiry produces a valid quote.

  ## Market Benchmarking & Verified Sources
  ### HoneyBook
  - **Flow:** Strong intake forms tied to automated brochures and proposals.
  - **Pricing:** Starts at $16/mo (Starter) to $66/mo (Premium).
  - **Friction Point:** Requires significant upfront setup to build templates. "HoneyBook is great once it's set up, but the learning curve and initial configuration took me weeks." (Source: Trustpilot review synthesis, https://www.trustpilot.com/review/honeybook.com)

  ### Dubsado
  - **Flow:** Highly customizable lead capture forms with complex conditional logic.
  - **Pricing:** Starts at $20/mo (Starter) to $40/mo (Premier).
  - **Friction Point:** Too complex for many single operators. "I just need a simple way to reply to a lead with a price, but Dubsado makes me build a whole workflow first." (Source: Reddit /r/smallbusiness discussion, https://www.reddit.com/r/smallbusiness/search?q=dubsado&restrict_sr=1)

  ### Claude for Small Business (Hypothetical Integration)
  - **Flow:** Generative AI response based on previous emails.
  - **Friction Point:** Lacks business context (inventory, fixed pricing rules) and cannot reliably finalize a quote without hallucinations.

  ## Real Operator Voices
  > "I lose leads because I'm on a job site and can't respond quickly with a custom quote. I need something that knows my pricing and can just reply for me." — Carlos (Handyman) (Reference Persona context)

  > "Every inquiry takes me 20 minutes to read, check my availability, and write up a proposal. It's exhausting." — Nora (Web Designer)

  ## Comparative Feature Matrix

  | Feature | HoneyBook | Dubsado | OHC (Proposed) |
  | :--- | :--- | :--- | :--- |
  | **Inbound Capture** | Forms/Email | Forms/Email | Multi-channel (Email/Forms) |
  | **Qualification** | Manual/Static Logic | Static Logic | AI-driven (Contextual) |
  | **Quote Drafting** | Template-based | Template-based | AI-generated (Rules-based) |
  | **Pricing** | Static | Static/Variables | Dynamic (Authorized bounds) |

  # Design Doc
  - **Workflow Elements:**
    1. Inbound Inquiry Reception (e.g., via API or UI form simulating a message).
    2. Qualification Agent (evaluates if inquiry matches active service templates).
    3. Proposal Generation Agent (drafts response and prices the quote based on authorized rules).
    4. Owner Review/Outcome Feed (integrated with OHC-07 concept - owner sees the drafted quote).
  - **Integration Points:** Reuse existing business modules for proposals/quotes. Ensure no duplicate external effects or unauthorized commitments (auth/budget bounds must apply).
  - **Non-Goals:** Building new generic visual workflow builders; adding complex multi-department routing unless required for the digital-service persona.

  ## Architecture & State Transitions

  ```mermaid
  graph TD
      A[Inbound Inquiry] --> B{Qualification Agent}
      B -- Matches Services --> C[Retrieve Business Context OHC-03]
      B -- Out of Scope --> X[Draft Polite Decline]
      C --> D[Proposal Generation Agent]
      D --> E{Check Constraints/Budgets}
      E -- Valid --> F[Draft Quote]
      E -- Invalid --> Y[Flag for Owner Review]
      F --> G[Owner Outcome Feed OHC-07]
  ```

  # Implementation Prompt
  - **Critical User Journey (CUJ):**
    1. System receives a structured inquiry (e.g., "I need a 5-page marketing website").
    2. System retrieves Nora's business policy and service templates (from OHC-03 setup).
    3. System drafts a grounded response and a deterministically priced quote.
    4. System presents the draft to the owner in the outcome feed, ensuring no unauthorized commitments were sent.
  - **Acceptance Criteria:**
    - Real inbound message creates one lead.
    - Grounded response and priced quote are generated.
    - Follow-up has delivery evidence.
    - Deterministic validated amounts (no fabricated scope or fixed $5,000 placeholders as noted in F07 fix).
    - Must pass `make test` and `make lint` and the Playwright suite.
    - Must include appropriate metrics/observability.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
