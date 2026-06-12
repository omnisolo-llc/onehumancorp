issue_title: "Agentic Estimate & Proposal System for Service Operators"
issue_description: |
  ## Problem Statement
  Service operators and agency principals (like Carlos the handyman and Nora the agency principal) waste hours turning customer inquiries into professional, actionable quotes or proposals. Existing tools (like Jobber, HoneyBook, or manual Word/PDF docs) are disconnected from the primary inbox, require heavy data entry on small mobile screens, and are disconnected from the platform's native payment flow. They need an AI work assistant that reads a work request, drafts a comprehensive estimate, gets the owner's 1-tap approval, and tracks the customer's acceptance and initial deposit.

  ## Research Report
  - **Market Context**: Service operators lose leads because they take too long to send estimates. Tools like Joist, Jobber, and HoneyBook have mobilized this, but they still require manual line-item entry. Wix and Shopify are primarily catalog-based, making custom quoting awkward.
  - **Competitor Gaps**:
    - *Shopify*: Has B2B draft orders, but it is not built for service estimates or proposals with custom project descriptions.
    - *HoneyBook/Jobber*: Great workflows but are standalone silos. AI features are bolt-on text generators, not autonomous agents that tie into a unified inbox.
  - **The OHC Opportunity**: By integrating the Quoting System directly into the Agent Feed, when a lead messages Carlos saying "Need my fence repaired, it's about 20 ft", the Sales/Operations agent can instantly look up Carlos's standard linear-foot pricing, draft an estimate with a 20% deposit link, and push it to his Agent Feed as an Action Card. Carlos taps "Approve," and the quote is sent.

  ## Design Doc
  ### Data Model (PostgreSQL)
  - `Quote`: Core entity linked to `Customer`, containing `status` (Draft, Pending Approval, Sent, Accepted, Rejected), `valid_until`, and `deposit_requirement_percentage`.
  - `QuoteLineItem`: Belongs to a Quote. Has `description`, `quantity`, `unit_price`, and optional `service_id` for catalog matching.

  ### AI Integration
  - **Sales Agent**: Uses RAG on past accepted quotes and base service prices to draft new line items from natural language inquiries.
  - **Work Triage**: Hooks into the unified Messaging / Agent Feed. Once a quote is drafted, an `ActionCard` is queued for owner review.

  ### Mobile UX Flow (375px)
  1. **Notification/Feed**: Action card appears in the Agent Feed: "Drafted estimate for John's Fence Repair ($450)."
  2. **Review Screen**: Tapping the card opens a clean, mobile-optimized line-item summary. The owner can tap any line to edit `description` or `price`.
  3. **Action Bar**: Sticky bottom bar with "Approve & Send" or "Discard".
  4. **Customer View**: Customer receives a mobile-friendly web link with a clear "Accept & Pay Deposit" button powered by Stripe Checkout.

  ### Key Design Decisions
  - Store quotes natively in PostgreSQL with strict multi-tenant isolation (`tenant_id` on all tables, RLS enabled).
  - Treat quotes as actionable items in the unified Work Triage feed rather than a hidden dashboard tab.
  - Native deposit collection via Stripe automatically converts an accepted Quote into a confirmed state.

  ## Implementation Prompt
  **Feature Name**: Agentic Quoting & Estimate Generation
  **Target Persona**: Carlos the Handyman, Nora the Agency Principal
  **Outcome**: The owner receives AI-drafted estimates directly in their Work Feed based on incoming messages. They can review, modify, and approve the quote with one tap. The customer receives a professional web link to accept and pay the deposit.

  **Next Actions**:
  1. Implement the core Data Models (`quotes`, `quote_line_items`) in PostgreSQL with RLS and multi-tenant isolation.
  2. Create the internal gRPC/REST APIs for managing quotes.
  3. Create the Sales Agent capability to parse inquiries, look up standard pricing, and draft Quote records autonomously.
  4. Develop the Mobile UI for Quote Review and the "Action Card" for the Agent Feed using Flutter/Tauri.
  5. Implement the Customer-facing Quote Acceptance and Deposit Payment flow via Stripe.
  6. Add thorough E2E Playwright tests covering the entire owner quote-approval and customer acceptance flow.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
