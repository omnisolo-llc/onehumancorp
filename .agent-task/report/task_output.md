issue_title: "Architecture: Agentic Estimate & Proposal Generation"
issue_description: |
  # Mission Queue Protocol: Agentic Estimate & Proposal Generation

  ## Title
  Implement Agentic Estimate & Proposal Generation Architecture

  ## Problem Statement
  For non-technical owners like Carlos (Handyman) and Nora (Agency Principal), turning a casual customer inquiry into a professional, bookable, and payable quote is a highly manual, multi-tool friction point. Currently, they have to read a message, mentally calculate costs, switch to a document or invoice tool, create a quote, generate a payment link, and send it back to the customer. We need an integrated flow where the AI assistant reads the work intake, automatically drafts a line-item estimate, attaches a deposit request, and presents it to the owner for a one-tap approval directly on their phone.

  ## Research Report
  - **Shopify/Wix/Squarespace**: These platforms are heavily optimized for self-serve cart checkouts (B2C products). They offer basic "Draft Orders" or "Invoices," but the process is manual and assumes the owner is sitting at a desktop adding line items.
  - **Service-Based Tools (HoneyBook, Jobber, Housecall Pro)**: These tools excel at quoting and invoicing. HoneyBook's "Smart Files" combine proposals, contracts, and payments, which is a great pattern. However, these platforms require steep learning curves and manual setup.
  - **The OHC Opportunity**: Unlike competitors, OHC can leverage its "Work Triage" and AI capabilities to automatically convert unstructured conversations (e.g., an Instagram DM or SMS) into structured proposals. The "Sales & Revenue Assistant" can proactively draft the quote based on historical pricing and available calendar slots, turning a 10-minute administrative chore into a 10-second approval swipe for the owner.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant Inbox (Work Triage)
      participant SalesAgent (The Dealmaker)
      participant FinanceAgent
      participant Owner (OHC App)

      Customer->>Inbox: "Need my kitchen sink fixed on Tuesday."
      Inbox->>SalesAgent: Trigger Intent Classification (Lead/Quote)
      SalesAgent->>SalesAgent: Extract requirements & draft line items
      SalesAgent->>FinanceAgent: Request deposit/payment link configuration
      SalesAgent->>Owner: Push "Draft Proposal" card to Agent Feed
      Owner->>Owner: Review on 375px mobile screen
      Owner->>SalesAgent: Tap "Approve & Send"
      SalesAgent->>Customer: Send proposal link via original channel (SMS/DM)
  ```

  ### UI Wireframes & Screen Flow (375px)
  1. **Agent Feed**: A priority card appears: "Draft Quote Ready for Carlos: Kitchen Sink Repair".
  2. **Proposal Review Screen**:
     - **Header**: Customer Name & Context ("Requested via SMS 10 mins ago").
     - **Body**: Translucent Glass styling card displaying Line Items (e.g., "Labor - 2hrs", "Parts - Standard Sink trap").
     - **Footer Sticky Action Bar**: Big touch targets (44x44px min). Primary: "Approve & Send ($150)". Secondary: "Edit Items".
  3. **Edit Mode (Optional)**: Native mobile numeric keyboard for adjusting prices. Swipe-to-delete line items.

  ### Mobile UX Flow
  - Designed exclusively for a 375px vertical constraint.
  - All critical calls-to-action are thumb-reachable at the bottom.
  - Loading states use skeleton UI, not spinners, to feel instantly responsive.
  - Optimistic UI on "Approve & Send" so the owner can immediately move on, while the backend finalizes the Stripe intent.

  ### AI Agent Integration Points
  - **Work Triage (Ingestion)**: Listens to incoming webhook messages and routes to the Sales Assistant when intent is classified as "Request for Quote".
  - **Sales & Revenue Assistant**: Responsible for parsing text to structured line items using RAG (matching text against the owner's service catalog).
  - **Finance & Decision Assistant**: Responsible for managing Stripe integration, generating idempotency keys, and creating payment intents for the proposal's deposit.

  ### Key Design Decisions
  - **Decoupled Drafting**: Agents create proposals in a `draft` state without exposing them to customers until explicit owner approval.
  - **Immutability After Send**: Once a proposal is approved and sent, the payload is locked to prevent discrepancies between what the customer sees and what the owner tracks.
  - **Tenant Isolation**: Row-level security (RLS) is strictly enforced on all Proposal entities to ensure no cross-tenant data leaks.

  ## Implementation Prompt
  **Role**: OHC Product Implementer
  **Task**: Implement the Agentic Estimate & Proposal Generation flow.

  **Outcome & CUJ**:
  - An owner receives a customer message requesting a service.
  - The AI backend automatically drafts a proposal and pushes a card to the owner's Agent Feed.
  - The owner taps the card on their mobile device (375px), reviews the auto-generated line items, and taps "Approve & Send".
  - The system transitions the proposal state and dispatches it.

  **Acceptance Criteria**:
  - Implement the UI flow for reviewing and approving a drafted proposal, adhering to the premium macOS Translucent Glass / UniFi dashboard design tokens. Ensure full responsiveness down to 375px.
  - Create the necessary backend data entities and API surface for Proposals, Line Items, and state transitions, enforcing multi-tenant isolation via `tenant_id`.
  - Provide complete E2E tests (Playwright) demonstrating the CUJ: an owner logging in, finding a drafted quote, viewing the details, and approving it. NO MOCK DATA in the UI.
  - Ensure 100% unit test coverage for new backend logic.
  - All tests (`bazel test //...`) must pass cleanly.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
