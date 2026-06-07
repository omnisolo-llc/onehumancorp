issue_title: "Implement the Omnichannel Zero-Touch Quotation & Proposal Agent"
issue_description: |
  ## Title
  Implement the Omnichannel Zero-Touch Quotation & Proposal Agent

  ## Problem Statement
  Service-based small business owners (like Carlos the Handyman or Nora the Agency Principal) lose substantial potential revenue because producing customized quotes and proposals takes hours and gets delayed until they are back at a computer. A customer requests a quote via Instagram DM, WhatsApp, or a web form, but the owner cannot rapidly assemble the scope, estimate materials, and format a professional proposal while on the go. Existing e-commerce setups (e.g., Shopify, Wix) are optimized for fixed-price products, not dynamic, service-based custom quoting. They require bolt-on CRMs or manual invoice drafting.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify / Wix / Squarespace:** Highly optimized for physical/digital goods with fixed prices. Quoting functionality requires complex third-party apps that fracture the user experience and don't natively integrate with omnichannel communications (e.g., DMs).
  - **Invoice Software (Freshbooks, QuickBooks):** Excellent at generating the actual invoice, but disconnected from the top-of-funnel inquiry (DMs). They are tools to be operated, not agents that act.
  - **OHC Opportunity:** Bridge the gap between a customer inquiry and a finalized proposal using our core AI agentic platform. When an inquiry comes in via any channel, the AI interprets the intent, extracts the project requirements, checks the owner's pricing rules/past jobs, and drafts a fully formatted, actionable quote. The owner just clicks "Approve & Send" on their mobile device.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Inquiry: IG, WhatsApp, Form] -->|Webhook| B(Omnichannel Gateway)
      B --> C{The Ambassador Agent - Intake}
      C -->|Extract Scope & Intent| D[Quotation Engine]
      D -->|Query| E[Tenant Pricing & Past Jobs DB]
      D --> F{The Operations Agent - Draft}
      F -->|Generate Proposal Draft| G[Action Required Queue]
      G --> H[Mobile App Feed 375px]
      H -->|1-Tap Edit/Approve| I[Quotation Dispatcher]
      I --> A
      I -->|Sync| J[Ledger / Expected Revenue]
  ```

  ### Mobile UX Flow (375px First)
  1. **Notification:** Carlos receives a push notification: "New Quote Drafted: Kitchen Sink Repair for Sarah".
  2. **Feed Card:** Tapping the notification opens a Glassmorphism card in his OHC feed.
  3. **Context View:** Top half shows Sarah's original Instagram DM ("My kitchen sink is leaking under the cabinet, can you fix it tomorrow?").
  4. **Draft View:** Bottom half shows the AI-generated quote breakdown: "Emergency Callout ($100) + Estimated Labor ($150) + Parts ($50). Total: $300."
  5. **Actions:** "Approve & Send Link", "Edit Quote", "Decline".
  6. **Customer Experience:** Upon approval, Sarah gets a DM with a Stripe Payment Link for a 50% deposit to confirm the booking.

  ### AI Agent Integration Points
  - **The Ambassador (Customer Success):** Parses the incoming unstructured text/images to identify the service requested and urgency.
  - **The Operations Agent (Quoting/Manager):** Takes the extracted scope, cross-references Carlos's base rates and previous similar jobs in the tenant DB, and drafts the line-item quote.

  ### Key Design Decisions
  - **Proactive Execution:** The quote is drafted *before* the owner even opens the app.
  - **Unified Flow:** The quote generation is intrinsically tied to the booking and deposit (Stripe) flow. It is not just a PDF generator; it is a conversion engine.
  - **Zero-Touch Fallback:** If the scope is too ambiguous, the AI drafts a clarifying question back to the customer instead of a full quote.

  ## Implementation Prompt
  **User-Facing Outcome:** As a service business owner, when a potential customer messages me asking "How much to paint a 2-bedroom apartment?", I receive a push notification containing a pre-calculated, professional proposal draft. I can review the line items on my phone, tap "Approve", and the customer instantly receives the quote with a deposit payment link.

  **Critical User Journey & Acceptance Criteria:**
  1. Implement a webhook receiver (Omnichannel Gateway) that accepts incoming customer inquiries (mocked as text for this phase).
  2. Implement `The Ambassador` intent extraction to identify a "quote request" and parse the scope.
  3. Implement the `Quotation Engine` using `The Operations Agent` to generate line items based on tenant pricing data.
  4. Create the mobile-first UI component (React/Flutter/HTML depending on stack) for the "Draft Quote Review" card with Approve/Edit actions.
  5. On approval, generate a verifiable output (e.g., a Stripe Payment Link generation call or a formatted message payload).

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
