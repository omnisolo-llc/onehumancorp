issue_title: "Automate inquiry capture to proposal generation for solo professionals"
issue_description: |
  **Title**: Automate Inquiry to Proposal for Solo Web/Design Professionals

  **Problem Statement**: Solo professionals (like Nora) spend too much unbillable time moving data from inquiry forms into CRMs, scheduling calls, and drafting proposals. This manual work causes delays, lost leads, and context switching away from core delivery work.

  **Research Report**:
  - *Context*: Investigating OHC-04 (inquiry capture and approved proposal).
  - *Persona*: Nora, solo web/design/marketing professional.
  - *Current Alternatives*: HoneyBook, Dubsado, manual email/spreadsheet tracking.
  - *Friction Points*: Setup complexity, lack of phone-first mobile support, manual data entry from inquiry to proposal.
  - *Sources*:
    1. [HoneyBook Pricing & Features](https://www.honeybook.com/pricing): Identifies tiers and multi-step onboarding, confirming setup complexity for solo users.
    2. [Dubsado Feature List](https://www.dubsado.com/features): Highlights heavy customization which operators report as a steep learning curve.
    3. [G2 HoneyBook Reviews](https://www.g2.com/products/honeybook/reviews): Users frequently mention that while powerful, initial setup takes hours and mobile app lacks full proposal editing.
    4. [Capterra Dubsado Reviews](https://www.capterra.com/p/153289/Dubsado/reviews/): Solo operators report feeling overwhelmed by the "blank canvas" approach.
    5. [Superpowers repository context](https://github.com/obra/superpowers/blob/main/skills/using-superpowers/SKILL.md) (Loaded from revision 5bf4e78011075bcfc0dc295f0724994cd123ee71).
    6. [Claude for Work/Small Business](https://www.anthropic.com/claude-for-work): Demonstrates capabilities of AI text generation but lacks native CRM integration.
    7. [Stripe Invoicing Docs](https://stripe.com/docs/invoicing): Confirms capability to programmatically generate payment links.

  **Comparative Feature Matrix**:
  | Feature | OHC Proposed Workflow | HoneyBook | Dubsado | Manual (Email + Stripe) |
  | :--- | :--- | :--- | :--- | :--- |
  | Setup Complexity | Low (Agent configured) | High (Multi-step) | High (Blank canvas) | Low (No setup) |
  | AI Proposal Drafting | Native, context-aware | Basic AI replies | None / Limited | None |
  | Mobile-first Review | Yes (375px optimized UI) | Partial (app limitations) | No | Yes |
  | Workflow Automation | End-to-end (Inquiry to Payment) | Yes, but rigid | Yes, highly customizable | No |

  **Design Doc**:
  - *Workflow*:
    1. Inquiry received (e.g., via a connected email or web form).
    2. OHC AI agent extracts key details (client name, budget, project scope).
    3. OHC AI agent drafts a proposal using a pre-approved template for the owner's specific service.
    4. Owner reviews and approves the proposal via a simple mobile-friendly UI (375px optimized).
    5. Approved proposal sent to client with payment link.

  ```mermaid
  stateDiagram-v2
      [*] --> InquiryReceived: Client submits form/email
      InquiryReceived --> DataExtracted: OHC Agent parses details
      DataExtracted --> ProposalDrafted: OHC Agent generates draft
      ProposalDrafted --> OwnerReview: Draft presented in 375px UI
      OwnerReview --> Approved: Owner taps approve
      OwnerReview --> Edited: Owner modifies details
      Edited --> Approved: Owner finalizes
      Approved --> SentToClient: Proposal sent with Stripe link
      SentToClient --> [*]
  ```

  - *Key Entities*: Inquiry, Client, Proposal, ServiceTemplate.
  - *Integration Points*: Email/Form ingestion, Stripe for payment links.

  **Implementation Prompt**: Implement an AI-driven workflow that takes a raw inquiry, extracts the structured data, and generates a draft proposal for owner review. Ensure the review UI is optimized for mobile (375px) and requires minimal taps to approve or edit. The system must not send the proposal without explicit owner approval (respecting authority boundaries).

  **Priority**: P1
  **Estimated Scope**: Medium

  **Strategy Admission**:
  - *Target ID*: OHC-04 (Inquiry capture to approved proposal)
  - *Stage*: Launch/Run
  - *Gap*: Automated, context-aware proposal generation from inquiry without manual data entry.
  - *Evidence Level*: Hypothesis based on persona research and public reviews.
  - *Baseline*: Manual drafting takes ~30-60 mins per lead.
  - *Result*: Reduce draft time to < 5 mins (review only).
  - *Dependencies*: LLM integration for extraction/drafting, basic CRM entity structure, Stripe integration.
  - *Non-goals*: Complex multi-stage negotiations, custom contract redlining, building a full email client.
  - *Authority Class*: Owner must explicitly approve before sending (Requires Approval).
  - *Cost Plan*: Track LLM token usage per proposal generated.
  - *Acceptance Checks*:
    - Happy path: Inquiry ingested -> Draft generated -> Owner approves -> Sent.
    - Failure path: Missing data -> Agent asks owner for missing info -> Owner provides -> Draft generated.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
