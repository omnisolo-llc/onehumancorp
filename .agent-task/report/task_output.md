issue_title: "[Architecture] Dynamic AI Service Quoting & Proposal Generator Engine"
issue_description: |
  ## 1. Problem Statement
  Service-based owners (like Nora the Agency Principal and Carlos the Handyman) spend hours manually estimating jobs, writing proposals, and managing approvals. Often, quoting involves disparate tools—email for back-and-forth communication, Word/Docs for the proposal, and a separate invoicing tool for payments. This disjointed process delays closing deals and requires technical or operational oversight that distracts from core work. They need an integrated solution where the AI assistant interprets customer needs, drafts a professional quote, coordinates approval, and securely manages the transaction, all natively within the OHC platform.

  ## 2. Research Report
  - **Market Context**: Platforms like HubSpot and specialized CRM tools offer robust quoting features but are too complex for the average SMB. Simple solutions like Wix or Square invoicing lack the proactive AI intelligence to draft and follow up on proposals based on conversational context.
  - **The OHC Opportunity**: OHC can differentiate by embedding quoting directly into the "Unified Inbox" and "Agent Feed." The Sales Assistant can read an inbound request (e.g., an Instagram DM asking, "How much to paint a 3-bedroom house?"), cross-reference the owner’s standard pricing, and instantly draft a proposal for the owner to review and send.
  - **Competitor Gaps**:
    - *Shopify/Square*: Excellent payment processing, but poor handling of custom, negotiated services without heavy plugin reliance.
    - *HubSpot*: Highly customizable but demands significant setup and active administration—violating OHC's "no technical manual" rule.
    - *Notion AI*: Good for drafting documents but lacks native payment integration and structured multi-tenant workflow state tracking.

  ## 3. Design Doc
  ### Data Model (PostgreSQL)
  - `Proposal`: Represents a specific quote for a client (linked to `tenant_id`, `customer_id`, `total_amount`, `status: [draft, sent, accepted, rejected, paid]`). Uses Row-Level Security for isolation.
  - `LineItem`: Specific services or products within the proposal (linked to `proposal_id`, `description`, `quantity`, `unit_price`).
  - `ProposalVersion`: Immutable snapshots of the proposal to track negotiations and changes over time.

  ### AI Integration
  - **Sales & Revenue Assistant**: Triggers upon detecting a quoting opportunity in the unified inbox. Uses Gemini Pro to parse the request, extract parameters (size, scope, timeline), and draft a proposal based on the owner's pricing memory.
  - **Customer Relationship Assistant**: Automatically drafts a polite, context-aware email or message to accompany the proposal link. Monitors for client questions and drafts responses for the owner.
  - **Operations Assistant**: Once accepted, automatically creates a project/task list and triggers the Finance Assistant to send the deposit invoice.

  ### Mobile UX Flow (375px)
  1. **Triage Feed**: Owner sees a card: "Carlos asked for a quote for a 3-bedroom paint job. Draft ready."
  2. **Review Screen**: Tapping the card opens a clean, translucent glass-styled screen showing the AI-drafted line items and total.
  3. **Edit & Send**: Owner can adjust prices using large touch targets. A single "Approve & Send" button generates a unique, edge-cached web link for the client and sends it via the original communication channel (e.g., WhatsApp).
  4. **Client View**: Client opens a responsive web view (without needing an app), reviews the proposal, and taps "Accept & Pay Deposit" (Stripe integration).

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner as Owner (Triage Feed)
      participant SalesAgent as Sales & Revenue Agent
      participant OHC as OHC API / DB
      participant Client as Client (Web View)
      participant Stripe as Stripe (Payments)

      Owner->>SalesAgent: Receives inquiry via Unified Inbox
      SalesAgent->>OHC: Extracts parameters & standard pricing
      SalesAgent-->>Owner: Drafts proposal & LineItems (Status: Draft)
      Owner->>Owner: Edits prices / Approves
      Owner->>OHC: Transitions Proposal to 'Sent'
      OHC-->>Client: Sends edge-cached Proposal link
      Client->>OHC: Reviews Proposal & accepts
      OHC->>Stripe: Initiates Checkout Session / Payment Intent
      Stripe-->>Client: Completes deposit payment
      Stripe-->>OHC: Webhook confirms payment
      OHC->>Owner: Updates Status to 'Paid' & notifies Owner
  ```

  ## 4. Implementation Prompt
  **Feature Name**: Dynamic AI Service Quoting Engine
  **Target Persona**: Nora (Agency Principal) & Carlos (Handyman)
  **Outcome**: Owners can convert conversational inquiries into paid deposits with one tap. The AI handles the drafting, formatting, and follow-up, keeping the owner in control of final pricing and approval.

  **Next Actions**:
  1. Define the PostgreSQL schema for `Proposal`, `LineItem`, and `ProposalVersion`, ensuring strict `tenant_id` RLS.
  2. Build the API endpoints (REST/gRPC) to support CRUD operations and state transitions for proposals.
  3. Integrate the Sales Agent to generate `LineItem` drafts from unstructured text using the `visual_workflow` or standard LLM tools.
  4. Develop the Flutter/PWA UI components for the "Proposal Review" and "Client Accept" screens, strictly adhering to the OHC Premium Token library (translucent materials, 44x44px touch targets).
  5. Connect the acceptance flow to Stripe for deposit collection.

  **Acceptance Criteria**:
  - Full mobile parity (usable on 375px screens).
  - 100% unit test coverage for the new backend services.
  - Playwright E2E test verifying the flow from AI draft creation to client acceptance and payment stubbing.
  - Zero mock data in the final UI code; all state must originate from the backend.

  **Priority**: P1
  **Estimated Scope**: Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []