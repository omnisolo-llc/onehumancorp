issue_title: "Agentic Mobile-First Proposals & Milestone Invoicing Architecture"
issue_description: |
  ## Problem Statement
  Service-based owners like Carlos (handyman) and Nora (agency principal) currently struggle with fragmented workflows. They receive leads in chat, manually create estimates in disconnected tools, ask for deposits via separate payment links, and track milestone payments in spreadsheets. This lack of a unified, intelligent system creates friction, delays revenue collection, and requires them to learn multiple complex softwares that are not designed for mobile-first operation.

  ## Research Report
  **Market Findings & Competitive Analysis:**
  - **Joist / Invoice2go / QuickBooks Self-Employed:** Good at simple invoices but lack proactive AI agents to draft proposals based on chat history. They are reactive record-keeping tools rather than intelligent assistants.
  - **HoneyBook / Dubsado:** Excellent feature sets for proposals and contracts but notoriously complex to set up. Their mobile experiences are often just responsive web views that are difficult to navigate on a 375px screen.
  - **Shopify / Stripe Invoicing:** Strong payment rails but lacking native service-oriented proposal structures, milestone billing (e.g., 50% deposit, 50% upon completion), and AI-driven client negotiation.
  - **OHC Opportunity:** Implement an "Agentic Proposal & Milestone Billing" system. The AI Assistant (e.g., The Sales & Revenue Agent) intercepts a service request in chat, drafts a line-item proposal based on past similar jobs, structures a milestone payment plan (deposit + final), and presents it to the owner in the Agent Feed as a 1-tap approval card. Once approved, the agent handles client follow-ups and deposit collection autonomously.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Request via Chat/Email] --> B(Work Triage Gateway)
      B --> C{Intent Resolution Engine}
      C -->|Service Inquiry| D[Sales & Revenue Agent]
      D -->|Query Past Jobs| E[(Unified Customer Graph DB)]
      E --> D
      D -->|Generate| F[Proposal & Milestone Draft]
      F --> G[Action Required Queue]
      G --> H[Mobile App Feed 375px]
      H -->|Owner 1-Tap Approve| I[Proposal Dispatcher]
      I --> J[Customer Receives Mobile-Optimized Quote]
      J -->|Customer Accepts| K[Stripe Payment Intent - Deposit]
      K -->|Webhook| L[Ledger & Job State Update]
      L --> M[Operations Assistant Creates Tasks]
  ```

  ### Mobile UX Flow (375px)
  1. **Agent Feed (Home):** Owner sees an Action Card: "Draft Proposal for Kitchen Repair (Carlos)".
  2. **Proposal Review Screen:** Tap card -> See a clean, translucent glass summary of line items (e.g., Materials $200, Labor $300), and a Milestone block (50% Deposit, 50% Final).
  3. **Edit Mode (Optional):** Owner taps a line item to adjust price using a native mobile number pad.
  4. **Approve & Send:** One primary bottom-sheet button "Send Proposal".
  5. **Client View:** Customer receives a link to a lightweight, fast-loading PWA showing the quote and a "Pay Deposit" Apple Pay/Google Pay button.

  ### AI Agent Integration Points
  - **Sales & Revenue Agent:** Automatically triggered when Work Triage identifies a service request. Uses context from the conversation to draft the proposal.
  - **Finance Assistant:** Monitors the Stripe webhook for the deposit payment. Once paid, it alerts the Operations Assistant.
  - **Operations Assistant:** Automatically creates the project tasks on the owner's calendar once the deposit is secured.

  ### Security & Multi-Tenant Isolation
  - **Database Level:** All proposal, invoice, and milestone records must use `tenant_id` with PostgreSQL Row-Level Security (RLS).
  - **Agent Memory:** The RAG system must strictly filter context (past jobs, pricing) by the requesting `tenant_id`.

  ## Implementation Prompt
  **Goal:** Implement the backend domain models, GraphQL/REST APIs, and the primary 375px mobile UI for the Agentic Proposals & Milestone Invoicing system.
  **CUJ (Critical User Journey):**
  1. As Carlos, I log into OHC and see an AI-drafted proposal in my feed for a recent customer inquiry.
  2. I tap the proposal, review the 50/50 milestone split, and tap "Approve".
  3. The system generates a public, edge-cached link for the proposal and records the pending state in the database.
  **Acceptance Criteria:**
  - Create the multi-tenant PostgreSQL schema for Proposals, Line Items, and Milestones.
  - Implement the "Sales & Revenue Agent" handler that listens to a mock Work Triage event and generates a proposal draft.
  - Build the Mobile-First (375px) Flutter or PWA UI for reviewing and approving the proposal, utilizing the OHC Translucent Glass design tokens.
  - Must include comprehensive unit tests (100% coverage on new backend code) and a Playwright E2E test verifying the approval flow from the Agent Feed.
  - Ensure zero mock data in the final UI presentation; all data must flow from the backend DB.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
