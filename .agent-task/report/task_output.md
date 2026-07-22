issue_title: "AI-Driven Intelligent Quoting & Deposit Workflow for Service Operators"
issue_description: |
  # Issue Brief: AI-Driven Intelligent Quoting & Deposit Workflow for Service Operators

  ## Problem Statement
  Service operators like Carlos (handyman) lose countless hours going back and forth with customers to provide quotes and secure deposits. Existing tools like Jobber or Housecall Pro are too complex, while generic platforms (Wix, Shopify) are not built for service-based workflows requiring dynamic quoting, calendar coordination, and upfront deposits. Carlos needs an AI assistant that can ingest a raw customer request via SMS or WhatsApp, instantly draft an accurate quote based on his historical pricing, coordinate his availability, and send a deposit link—all from his Android phone with zero manual data entry.

  ## Research Report
  - **Shopify/Wix:** Built for e-commerce, not service quoting. Requires clunky workarounds (e.g., selling a "Deposit" product) which confuses customers.
  - **Jobber/Housecall Pro:** Powerful but heavy. Requires significant setup, manual entry of items, and acts as a software suite rather than an active assistant.
  - **OHC Opportunity:** By leveraging the Operations and Sales AI Agents, OHC can turn a simple text message ("I need my sink fixed next Tuesday") into a fully structured, actionable Quote Card on Carlos's 375px mobile feed. The AI cross-references past sink repair jobs to suggest a price, checks the calendar for next Tuesday, and generates a Stripe Payment Link for the deposit. Carlos just taps "Approve."

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Request SMS/WhatsApp] -->|Webhook| B(Omnichannel Gateway)
      B --> C[Intent Classifier AI]
      C -->|Service Request| D[Operations Agent - Calendar]
      C -->|Quote Needed| E[Sales Agent - Pricing]
      D --> F[Unified Quote Engine]
      E --> F
      F -->|Historical Pricing Lookup| G[(Central Ledger DB)]
      F --> H[Draft Quote Card Generated]
      H --> I[Mobile Feed - 375px]
      I -->|Carlos Approves| J[Stripe Deposit Link Created]
      J --> K[Quote & Link Sent to Customer]
  ```

  ### Mobile UX Flow (375px First)
  - **Home Feed:** Carlos sees a priority card: "Draft Quote: Sink Repair for Maria".
  - **Card Expansion:** Tapping reveals AI context: "Suggested price: $150 (based on 3 similar past jobs). Suggested time: Tuesday 2 PM. Deposit required: $50."
  - **Interaction:** Carlos can edit the price or time via native keyboard, or simply tap a large, prominent "Approve & Send" button.
  - **Customer View:** The customer receives a clean, mobile-optimized webpage (Apple/Ubiquiti-style clean aesthetics) outlining the service, time, and an Apple Pay/Google Pay enabled Stripe checkout for the deposit.

  ### AI Agent Integration Points
  - **Sales Agent:** Analyzes the request text against historical invoices/jobs to predict the correct price and deposit amount.
  - **Operations Agent:** Cross-references the predicted service duration with Carlos's calendar to suggest a time slot.

  ### Key Design Decisions
  - **Zero-Setup AI Pricing:** Rely on historical data via RAG instead of forcing the owner to manually build a complex price book upfront.
  - **Action-Oriented Cards:** The owner only reviews completed drafts, never starts from a blank form.
  - **Frictionless Deposits:** The quote implicitly includes a Stripe Checkout intent to secure the booking, eliminating the "agreed but unpaid" limbo state.

  ## Implementation Prompt
  **Implementer Agent Task:** Build the end-to-end flow for AI-driven quoting and deposits.

  **User-Facing Outcome:** The user receives a draft quote card in their feed when a service request arrives, can review/edit the price and time, and send it to the customer. The customer can view the quote and pay a deposit via a public link.

  **CUJ:**
  1. A mock incoming SMS request for a service job is triggered.
  2. The system generates a draft quote card in the OHC mobile feed.
  3. The owner (Carlos persona) clicks "Approve" on the draft.
  4. The system generates a public URL for the quote containing a payment flow.

  **Acceptance Criteria:**
  - Full backend ingestion to feed card generation flow is functional.
  - The UI accurately renders the draft quote card on a 375px layout.
  - The public quote page allows for successful (test mode) payment processing.
  - Playwright E2E tests are implemented to verify the full CUJ from incoming request to owner approval to public page rendering.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
