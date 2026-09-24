issue_title: "OHC-04: Inquiry capture, qualification and an approved proposal"
issue_description: |
  # Inquiry to Proposal Workflow Research

  ## Title
  Automated Inquiry Capture, Qualification, and Proposal Generation (OHC-04)

  ## Problem Statement
  For solo web, design, and marketing professionals (like Nora), turning a prospective client's initial inquiry into a qualified, approved proposal is a manual, time-consuming process. Owners often lose leads because they are busy delivering work and cannot respond to inquiries quickly. When they do respond, they spend hours manually qualifying the prospect, assessing requirements, and drafting custom proposals. This friction results in dropped leads, inconsistent pricing, and lost revenue opportunities.

  ## Research Report
  ### Findings & Market Analysis
  Current workflows force owners to string together separate tools: a contact form (e.g., Typeform, native website form), email (Gmail), and a proposal/invoicing tool (HoneyBook, PandaDoc, or manual PDFs).
  - **HoneyBook**: Offers robust proposal and invoicing but requires significant upfront setup and manual intervention to draft custom scopes based on initial inquiries. Its mobile experience for deep proposal editing is limited.
  - **Claude for Small Business / ChatGPT**: Can draft proposal text if the owner manually copies and pastes the prospect's email context and provides context on services. However, it lacks native integration with payment or scheduling gateways, meaning the owner still must copy the output into a proposal tool.
  - **Operator Pain Points (Verified Sources)**:
    - *Friction in Lead Response*: A common theme in Reddit's `r/freelance` and `r/smallbusiness` (Source: Reddit Discussions, 2026-09) is that freelancers take 24-48 hours to respond to leads, by which time the prospect may have moved on.
    - *Qualification Burden*: "I get a lot of leads, but 50% can't afford my minimums. Sifting through them wastes hours." (Source: Anonymous Owner Anecdote O1, RESEARCH.md).
    - *Proposal Generation*: Translating a messy email thread into a professional scope of work and pricing table is cited as the highest-friction administrative task.

  ### Competitive Matrix
  | Feature | HoneyBook | Claude / GenAI (Standalone) | OHC Proposed Solution |
  |---------|-----------|-----------------------------|-----------------------|
  | **Inquiry Capture** | Form embeds | None | Email / Form integration |
  | **Auto-Qualification**| Basic logic jumps | High (if prompted) | Agent-driven evaluation against owner criteria |
  | **Proposal Drafting** | Templates (Manual edit)| High (Text only) | Agent-drafted scope + pricing + payment link |
  | **Mobile Approval** | Limited editing | Text editing | 375px native approval flow |
  | **Standing Authority**| N/A | N/A | Drafts within bounds; requires owner 1-tap approval |

  ### Rationale for Implementation
  Implementing an AI-assisted inquiry-to-proposal pipeline directly addresses the "Inquiry capture, qualification and an approved proposal" capability gap (OHC-04). By generating a structured draft proposal for the owner to review, we reduce the time-to-quote from days to minutes, directly impacting revenue conversion.

  ## Design Doc

  ### Architecture
  ```mermaid
  graph TD
      A[Inquiry Source: Email/Form] -->|Raw Text| B[Agent: Intake Classifier]
      B --> C{Qualified?}
      C -->|No| D[Draft Polite Rejection]
      C -->|Yes| E[Agent: Proposal Generator]
      E --> F[Draft Proposal]
      F --> G[Owner Mobile App: Review Queue]
      G -->|1-Tap Approve| H[Send to Client]
      G -->|Revise| E
      H --> I[Awaiting Deposit/Booking]
  ```

  ### Entity Types & Integration Points
  - **Inquiry**: The raw lead data (name, email, project description, budget).
  - **Service Playbook**: The owner's predefined offerings, minimum pricing, and availability.
  - **Draft Proposal**: Generated entity containing scope, timeline, and pricing estimates.
  - **Integration Points**: Google Workspace (Gmail) for inbound inquiry monitoring; Stripe (or existing billing service) for payment link attachment.

  ### UI Wireframes & Mobile UX Flow (375px First)
  1. **Push Notification**: "New qualified lead from [Name] for [Project]. Proposal drafted."
  2. **Review Screen (Mobile)**:
     - Header: Prospect Name & Project Type.
     - Section 1: Lead Summary (Budget, Timeline, Fit Score).
     - Section 2: Draft Proposal (Scope bullet points, Price).
     - Sticky Bottom Bar: [Approve & Send] [Edit/Reject].
  3. **Edit Screen**: Simple text area to tweak the AI-generated scope or adjust the price slider before sending.

  ### AI Agent Integration Points
  - **Intake Classifier Agent**: Triggered on new inbound message. Evaluates against the owner's Service Playbook.
  - **Proposal Generator Agent**: Translates the qualified inquiry and owner's playbook into a structured proposal.
  - **Guardrails**: Agents *cannot* send proposals autonomously. They only generate drafts requiring explicit owner approval (Standing Authority limit).

  ## Implementation Prompt
  **User-Facing Outcome**: The owner receives a push notification on their phone for every new inquiry. Instead of a raw email, they see a qualified summary and a pre-drafted proposal based on their service menu. With one tap, they can approve and send the proposal to the client.

  **Critical User Journey**:
  1. A prospective client submits an inquiry via email or a connected web form.
  2. The system ingests the inquiry and an AI agent qualifies it against the owner's minimums.
  3. The AI agent drafts a proposal (scope, price, timeline) and places it in a "Requires Approval" queue.
  4. The owner opens the OHC mobile view (375px), reviews the generated draft, and taps "Approve & Send".
  5. The prospect receives a professional email with a link to accept and pay the deposit.

  **Acceptance Criteria**:
  - The system must capture an inbound inquiry and parse key details (budget, needs).
  - A draft proposal must be deterministically generated using the owner's configured service rules.
  - The UI must render a mobile-optimized (375px) review screen for the draft.
  - The proposal must NOT be sent to the client without explicit owner approval (simulated or real).
  - Failure path: If the AI cannot confidently draft a proposal, it routes the raw inquiry to the owner's manual inbox with a flag.

  ## Priority
  P1

  ## Estimated Scope
  Medium

  ## Strategy Admission
  - **OHC Target ID**: OHC-04
  - **Launch/Run stage**: Launch Stage
  - **Observed/Inferred Gap**: Inferred from owner anecdotes and competitor workflows; current OHC implementation lacks a seamless, agent-drafted proposal queue.
  - **Evidence Level**: Documented operator friction (Reddit, O1 anecdote).
  - **Baseline and Measurable Result**: Reduce owner time spent drafting a proposal from baseline (avg 45 mins) to review time (< 5 mins). Denominator: Number of qualified inquiries received.
  - **Dependencies/Reuse**: Requires existing agent primitive capabilities and notification queue UI. Reuse existing proposals/invoices modules where available.
  - **Non-Goals**: Full automated negotiation (chatbots talking to clients); complex multi-stage custom enterprise bidding.
  - **Authority Class**: Draft-only standing authority. Explicit owner approval required to commit scope/price.
  - **Cost/Measurement Plan**: Track agent compute tokens per inquiry processed. Expected serving cost < $0.50 per qualified lead.
  - **Happy-path and Failure Acceptance Checks**:
    - Happy: Valid inquiry -> Draft generated -> Owner approves -> Sent status.
    - Failure: Ambiguous inquiry -> Handed off to owner without sending incorrect price.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
