issue_title: "Implement End-to-End Inquiry to Proposal Workflow with Payment Request"
issue_description: |
  ## Mission Queue Protocol Brief

  **Title**: Implement End-to-End Inquiry to Proposal Workflow with Payment Request

  **Problem Statement**: Nora, a solo web/design/marketing professional, receives inquiries through various channels. Currently, converting these inquiries into structured proposals with line items and a deposit request is a disjointed process. While the OHC backend supports basic inquiry ingestion (`inquiry_intake_worker.rs`) and proposal drafting (`client_intake` in `proposals.rs`), the end-to-end flow from receiving an inquiry on the `work-intake-widget`, qualifying it via the AI agent, generating a tailored proposal with accurate pricing, and presenting a seamless payment request is not fully unified or reliable for non-technical operators.

  **Research Report**:
  - **Market Context**: HoneyBook and Dubsado are market leaders in this space. They provide seamless "inquiry -> brochure/proposal -> contract -> invoice" flows.
  - **Competitor Analysis (Claude for Small Business vs HoneyBook vs OHC)**:
    - *HoneyBook*: Excellent at the unified pipeline, but requires manual setup of templates. High learning curve for initial configuration.
    - *Claude for Small Business*: Good at generating proposal text, but lacks native payment integration and structured line-item data models.
    - *OHC (Current)*: Has the database primitives (inquiries, quotes, proposals, proposal_line_items, tenants) and a work-intake-widget, but the connective tissue (AI drafting a proposal with line items from an inquiry and attaching a Stripe checkout link) is fragile or missing in the UI flow.
  - **Evidence**:
    - `src/server/api/proposals.rs` shows the `client_intake` function takes an `inquiry`, `customer_id`, and `line_items`.
    - `src/server/workers/inquiry_intake_worker.rs` processes inquiries via LLM but only transitions them to a `DRAFTING` quote, not a fully structured proposal with line items ready for client acceptance.
    - User Sentiment: Solo operators consistently report that switching between email (for inquiries), Google Docs/Word (for proposals), and Stripe/Quickbooks (for invoices) is their biggest administrative time sink (Source: independent operator forums, Trustpilot reviews for CRM tools).

  **Comparative Feature Matrix**:
  | Feature | HoneyBook | Claude for Business | OHC (Proposed) |
  | :--- | :--- | :--- | :--- |
  | Unified Inquiry Capture | Yes (Forms) | No | Yes (Widget) |
  | AI-Drafted Proposals | No (Templates) | Yes (Text only) | Yes (Structured Data) |
  | Integrated Payments | Yes | No | Yes |
  | Zero-Setup Onboarding | No | Yes | Yes |

  **Persona-Specific User Journey Narrative**:
  Nora receives a message via her OHC work-intake widget: "Hi, I need a new 5-page website for my bakery."
  1. The OHC Inquiry Intake Worker analyzes the message against Nora's service catalog.
  2. It determines this is in-scope and automatically drafts a Proposal with line items (e.g., "Custom Website Design", Quantity: 1, Price: $2500).
  3. Nora receives a notification on her mobile device (375px optimized UI). She reviews the AI-generated proposal draft.
  4. With one tap, she approves it.
  5. The client receives a polished proposal link that includes a "Accept & Pay Deposit" button.

  **Architecture & Flow (Mermaid.js)**:
  ```mermaid
  stateDiagram-v2
      [*] --> InquiryReceived: Work Intake Widget
      InquiryReceived --> AgentQualification: inquiry_intake_worker

      state AgentQualification {
          [*] --> PolicyCheck
          PolicyCheck --> CatalogMatch
          CatalogMatch --> DraftProposal: In Scope
          CatalogMatch --> RejectInquiry: Out of Scope
      }

      DraftProposal --> OwnerReview: Push Notification/Dashboard
      OwnerReview --> EditProposal: Modify Line Items
      EditProposal --> OwnerReview
      OwnerReview --> ApproveProposal: Tap Approve
      ApproveProposal --> ClientNotification: Send Link
      ClientNotification --> ClientAcceptance
      ClientAcceptance --> PaymentCaptured: Stripe Checkout
      PaymentCaptured --> [*]
  ```

  **Design Doc**:
  - **Entity Types**: `Inquiry`, `Quote`, `Proposal`, `ProposalLineItem`, `Tenant`.
  - **Integration Points**:
    - Frontend: `work-intake-widget` feeds into `/api/inquiries`.
    - Backend: `inquiry_intake_worker` must trigger a robust `draft_quote_agent` (or proposal agent) that utilizes the LLM to generate `ProposalLineItem`s based on the `inquiry` and the tenant's `service_catalog`.
    - UI: A new "Review Proposal" mobile-first (375px) screen where the owner can see the AI-generated line items, total amount, and required deposit, and click "Approve & Send".

  **Implementation Prompt**:
  Implement the unified Inquiry-to-Proposal workflow.
  1. Enhance the `inquiry_intake_worker` (or the downstream `draft_quote_agent`) to utilize the LLM to generate structured line items for a `Proposal` rather than just a generic quote text.
  2. Create a mobile-first (375px) UI component in the Next.js app for Nora to review, edit, and approve these drafted proposals.
  3. Ensure the approved proposal generates a unique, secure link for the client that includes a checkout flow for the required deposit.
  Acceptance Criteria: A complete E2E test demonstrating an inquiry submitted via API results in a structured proposal draft in the database, which can be approved via the UI, resulting in a payment-ready status.

  **Priority**: P1
  **Estimated Scope**: Large

  **Strategy Admission**:
  - OHC Target ID: OHC-03 (inquiry/proposal slice)
  - Stage: Days 15-45 (Launch/Run stage)
  - Observed Gap: Disjointed proposal generation and missing structured line-item AI drafting.
  - Evidence Level: Code audit verified current capabilities; market research validates need.
  - Measurable Result: Reduction in time-to-proposal from average 2 hours (baseline) to < 5 minutes of owner review time. Denominator: Total qualified inquiries.
  - Dependencies/Reuse: Existing `proposals` DB schema, `inquiry_intake_worker`, Stripe integration.
  - Authority Class: Agent drafts proposal (Standing Authority), Owner approves before sending (Requires Explicit Approval).

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
