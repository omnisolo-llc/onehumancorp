issue_title: "Implement AI-Automated Service Quote & Proposal Generator for Service Operators"
issue_description: |
  ## Title
  Implement AI-Automated Service Quote & Proposal Generator for Service Operators

  ## Problem Statement
  Service business operators like Carlos (field service) and Nora (agency principal) lose critical momentum because generating custom quotes and proposals is a manual, time-consuming process. When a lead requests a quote (e.g., via DM, email, or a web form), the operator typically has to switch context, manually calculate service costs and materials, draft a document, and generate a payment link for a deposit. This friction leads to delayed responses, lost leads, and administrative exhaustion. There is a missing critical user journey (CUJ) where the AI assistant seamlessly turns inbound demand into actionable, ready-to-approve quotes.

  ## Research Report
  - **Market Context**: Platforms like Jobber, Housecall Pro, and HoneyBook provide quote generation, but they require the user to manually enter data into complex forms. The small business owner is still acting as a data entry clerk.
  - **Competitive Analysis**:
    - *Shopify*: Great for fixed-price products, completely fails for dynamic service quotes.
    - *Wix/Squarespace*: Rely on clunky 3rd-party form plugins that don't natively generate integrated Stripe invoices with deposits.
    - *Jobber/Honeybook*: Form-heavy and desktop-first; they do not proactively draft the quote based on AI understanding of a casual text message or DM.
  - **Findings**: The highest conversion rates for service businesses happen when a quote is sent within 15 minutes of the request. By using the AI Sales & Operations Assistant to instantly draft a quote from natural language and present it for a 1-tap owner approval, OHC can radically differentiate its value proposition for service businesses.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Lead as Customer Lead
      participant Inbox as OHC Triage Inbox
      participant AI as AI Sales/Ops Assistant
      participant DB as Multi-Tenant DB
      participant Owner as Owner (Mobile UI)
      participant Stripe as Payment Gateway

      Lead->>Inbox: Sends message: "Need a quote for fixing 2 doors"
      Inbox->>AI: Trigger intent parsing
      AI->>DB: Fetch Service Catalog & Pricing for Tenant
      AI->>DB: Draft preliminary Quote & Deposit terms
      AI->>Owner: Push Notification: "Quote drafted for 2 doors. Review?"
      Owner->>Owner: Reviews on 375px mobile UI (Translucent Glass card)
      Owner->>AI: Approves Quote (1-tap)
      AI->>Stripe: Generate Payment Link / Checkout Session
      AI->>Lead: Replies with friendly message + Quote + Deposit Link
  ```

  ### Mobile UX Flow & UI Wireframes (375px First)
  - **Screen 1: Triage Feed (Home)**
    - *Layout*: UniFi-style translucent glass card at the top of the feed.
    - *Content*: "New Quote Drafted: 2 Door Repairs for John Doe. Est: $300."
    - *Action*: Large primary action button (44x44px touch target) labeled "Review Quote".
  - **Screen 2: Quote Review Modal**
    - *Layout*: Bottom sheet sliding up over the inbox. Clean, non-technical interface.
    - *Content*:
      - Breakdown of services (e.g., "Labor - 2 Hours: $150", "Materials - $150").
      - Deposit requested: "$100".
      - Generated friendly message to customer.
    - *Actions*: "Approve & Send" (Primary, bold color), "Edit" (Secondary outline), "Reject" (Tertiary).
  - **Screen 3: Confirmation**
    - *Layout*: Minimal toast notification: "Quote sent! We'll notify you when the deposit is paid."

  ### AI Agent Integration Points
  - **Intake Parser**: The AI agent listens to the unified Triage Inbox stream. If it detects a request for an estimate/quote, it extracts the scope of work.
  - **Contextual Pricing Engine**: The agent queries the tenant's predefined service list (`ohc:lock:{tenant_id}:services`) to guess pricing. If pricing is ambiguous, it prepares a range.
  - **Human-in-the-Loop (HITL)**: The agent never sends a quote without the owner's explicit 1-tap approval, maintaining safety and owner control.

  ### Key Design Decisions
  - **1-Tap Approval over Complex Forms**: We optimize for speed. If the AI gets the price wrong, the owner taps "Edit", which trains the model for next time, but the baseline is a fast track.
  - **Deposit-First Integration**: The quote automatically includes a Stripe deposit link by default, as service operators (Carlos) frequently suffer from unpaid time without upfront commitments.
  - **Mobile Native**: The entire approval process is designed to be executed one-handed by an operator in the field.

  ## Implementation Prompt
  Implement the AI-Automated Service Quote Generator CUJ.
  1. Add a Quote entity to the multi-tenant PostgreSQL schema that supports line items, a linked lead/customer, and a deposit amount.
  2. Create the backend gRPC/REST endpoint to allow the AI Agent queue to draft a Quote based on a customer inquiry.
  3. Build the Mobile-First (375px) Frontend UI in the Triage Feed that displays pending drafted quotes. Use the premium OHC Translucent Glass design tokens and ensure touch targets are 44x44px.
  4. Implement the "Approve & Send" action which finalizes the quote, generates a Stripe Payment Link for the deposit, and records the interaction in the ledger.
  5. **Acceptance Criteria**: A complete E2E Playwright test simulating Carlos (the operator) logging in, seeing a drafted quote in his feed, clicking "Approve", and verifying the quote is marked as sent with a generated deposit link. No mock data; route through the real API.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
