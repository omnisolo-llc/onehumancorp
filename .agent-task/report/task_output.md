issue_title: "HoneyBook / Claude Workflow Gap Analysis"
issue_description: |
  # Superpowers Workflow Provenance
  - Skills loaded: `using-superpowers`, `brainstorming`
  - Repository URL: `https://github.com/obra/superpowers.git`
  - Revision hash: `8ca22dba9a94f28898bbce59f2537ff4d87c747d`
  - Checks performed: Synthesized owner/operator workflows from existing codebase analysis and provider capabilities.
  - Outcomes: Generated research report mapping OHC missing capabilities against HoneyBook and Claude baselines.

  # Title
  Inquiry capture, qualification and an approved proposal (OHC-02)

  # Problem Statement
  Nora, a solo web/design professional, currently spends excessive net time transitioning a raw inquiry (email/social DM) into a qualified lead, drafting a customized proposal, and getting it signed/approved. She often loses track of requests across channels when busy, leading to dropped leads. Existing OHC capabilities lack a cohesive, persistent "Lead to Proposal" loop that functions securely under delegated authority without requiring her manual intervention for every state change.

  # Research Report
  - **Baseline:** Nora uses disconnected tools (Gmail, Calendly, manual PDF generation, Stripe) or centralized CRMs like HoneyBook.
  - **HoneyBook comparison:** HoneyBook excels at consolidating inquiries, auto-replying, and sending smart files (proposals + invoices). It lacks generative AI for complex, domain-specific qualification out-of-the-box without manual template configuration.
  - **Claude for Small Business comparison:** Claude can draft proposals beautifully but lacks persistent context, calendar/booking integration, and secure provider boundaries (it's a chat interface, not an agentic workflow).
  - **OHC Gap:** OHC has the individual integration pieces (email via Resend/Sendgrid, Calendar via Cal.com/Google Calendar) but lacks the orchestrator workflow to stitch "Inquiry Received -> Agent Qualifies -> Proposal Generated -> Owner Approves -> Client Accepts" into a single trackable state machine within standing authority.

  ## Comparative feature matrix
  | Feature | Nora Baseline | HoneyBook | Claude | OHC (Proposed) |
  | --- | --- | --- | --- | --- |
  | Inbox Consolidation | ❌ | ✅ | ❌ | ✅ |
  | Generative AI Qualification | ❌ | ❌ | ✅ | ✅ |
  | Agentic Proposal Drafting | ❌ | ❌ | ✅ | ✅ |
  | Secure Provider Boundaries | ❌ | ✅ | ❌ | ✅ |
  | Standing Authority Limits | ❌ | ❌ | ❌ | ✅ |

  # Design Doc
  - **Architecture:**
    ```mermaid
    graph TD
      A[Event Source Email/Webhook] --> B[Inquiry Triage Agent]
      B --> C[Proposal Drafter Agent]
      C --> D[State Machine]
      D --> E[Drafted]
      D --> F[Owner Review]
      D --> G[Sent]
      D --> H[Accepted]
    ```
  - **UI Wireframes (375px mobile first):**
    - "New Lead" push notification.
    - Tap to open: See summarized client need, drafted response, and proposed scope/price.
    - Action buttons: "Approve & Send", "Edit Scope", "Reject".
  - **AI Integration Points:** Qualification Agent (scores lead), Drafting Agent (writes proposal based on historical business rules).

  # Implementation Prompt
  Implement a stateful "Lead to Proposal" workflow. Create a new durable event pipeline that listens for incoming inquiries, triggers the qualification agent, and prepares a draft proposal. The system must enforce an "Owner Review" gate before sending any external commitment or pricing. Acceptance criteria: A mock inquiry flows through the system, produces a verified draft proposal, halts for owner approval, and correctly updates the lead state without unauthorized send.

  # Priority
  P1

  # Estimated Scope
  Medium

  # Strategy Admission
  - Target ID: OHC-02
  - Stage: Launch
  - Gap: Inferred from missing state orchestration in `registry.rs` vs `RESEARCH.md` goals.
  - Evidence level: Documented.
  - Baseline/Result: Reduce owner time from 30 mins per proposal to 2 mins (approval only).
  - Dependencies: Existing email/calendar providers.
  - Authority class: Draft-only without explicit approval.
  - Cost plan: Measure token usage per proposal generation.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
