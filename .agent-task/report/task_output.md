issue_title: "Agentic Quote & Milestone Invoicing System"
issue_description: |
  ## Problem Statement
  Service-based and agency business owners (e.g., Nora the Agency Principal, Carlos the Handyman) struggle with disconnected quoting, project tracking, and invoicing systems. Today, owners manually generate quotes in Word/Docs, email them, await manual approval, perform the work, and then manually generate separate invoices in tools like QuickBooks or Wave, often forgetting to bill for change orders or final milestones. This fragmented workflow leads to delayed revenue, lost context, and a lack of professional presentation to the client.

  ## Research Report
  - **Market Context**: Platforms like Shopify handle discrete products well but fail at complex service workflows requiring negotiation and milestone payments. Wix and Squarespace offer basic invoicing but lack intelligent progression from quote to project to final invoice. Dedicated tools like HoneyBook or Dubsado are powerful but often feel disconnected from the primary business operations and lack proactive AI agents.
  - **The OHC Opportunity**: By integrating quoting and milestone invoicing directly into the OHC platform, powered by Sales and Finance Agents, we can automate the quote-to-cash lifecycle. The AI can draft proposals based on client conversations, convert approved quotes into tracked tasks, and automatically send invoices when milestones are reached.
  - **Competitor Gaps**:
    - *Shopify*: Primarily e-commerce; poor handling of custom service quotes.
    - *Wix/Squarespace*: Basic invoicing; passive systems with no AI progression.
    - *HoneyBook*: Excellent workflow but siloed from broader business operations (e.g., inventory, broader marketing).

  ## Design Doc
  ### Data Model (PostgreSQL)
  - `Quote`: Represents a proposed service or project with line items, total cost, and status (draft, sent, approved, rejected).
  - `QuoteLineItem`: Detailed items within a quote, optionally linked to standard services or products.
  - `Milestone`: Defined phases of a project derived from an approved quote, with associated payment amounts (e.g., 50% deposit, 50% on completion).
  - `Invoice`: Generated from a Quote or Milestone, tracking the actual request for payment, linked to a Stripe Payment Intent.

  ### AI Integration
  - **Sales Agent**: Monitors communications (e.g., DMs, emails) for project inquiries. Can automatically draft a `Quote` based on natural language project descriptions and past similar projects.
  - **Finance Agent**: Tracks `Milestone` completion (triggered manually or by the Operations Agent completing related tasks). Automatically drafts and sends `Invoices` for completed milestones and follows up on overdue payments.

  ### Mobile UX Flow (375px)
  1. **Owner View (Drafting)**: Owner reviews an AI-drafted Quote in a clean, card-based interface. They can easily edit line items or milestone percentages with large touch targets. A prominent "Send Quote" button pushes the quote to the client via email/SMS.
  2. **Client View (Approval)**: Client receives a mobile-optimized web link displaying the quote. They can review line items, digitally sign/approve, and immediately pay the initial deposit via Stripe (Apple Pay/Google Pay integrated).
  3. **Owner View (Tracking)**: The Owner Dashboard shows active projects. When a milestone is marked complete, a one-tap "Generate Invoice" action is presented, pre-filled with the milestone amount.

  ## Implementation Prompt
  **Feature Name**: Agentic Quote & Milestone Invoicing System
  **Target Persona**: Nora the Agency Principal
  **Outcome**: Nora can receive a project request, have the Sales Agent draft a detailed quote with a 50/50 milestone split, send it for client approval, and have the system automatically generate the final invoice when the project tasks are marked complete.

  **Next Actions**:
  1. Implement the core Data Models (`Quote`, `QuoteLineItem`, `Milestone`, `Invoice`) ensuring strict multi-tenant isolation.
  2. Develop the Owner UX for reviewing, editing, and sending Quotes (mobile-first, 375px optimized).
  3. Develop the Client UX for viewing, approving, and paying deposits on Quotes.
  4. Create the Sales Agent capability to draft Quotes from conversational context.
  5. Create the Finance Agent capability to manage Milestones and generate Invoices upon completion.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
