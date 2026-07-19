issue_title: "Implement Autonomous Quote-to-Cash Agent for Service Businesses"
issue_description: |
  # Research Report: Autonomous Quote-to-Cash & The Sales Assistant

  ## Problem Statement
  Service-based small business owners (like Carlos the Handyman or Nora the Agency Principal) spend an enormous amount of unbillable time translating customer inquiries into formal estimates, sending them back and forth, and chasing deposit payments. Traditional platforms (like Shopify or Wix) are built for static product checkout, not dynamic, conversational quoting. Existing CRM/invoicing tools (like Jobber or QuickBooks) are passive databases requiring heavy manual data entry.

  ## Research Report
  - **Market Gap:** The service sector (home repair, tutoring, design agencies) is deeply underserved by standard e-commerce flows. E-commerce is "click-to-buy"; services are "inquire-to-quote-to-book-to-pay".
  - **Competitor Analysis:**
    - *Jobber/Housecall Pro:* Excellent for field services, but require the owner to manually build quotes from item lists. They don't autonomously draft the quote from a customer DM.
    - *Shopify/Wix:* Attempt to shoehorn services into "products with variations." Highly rigid.
    - *Stripe Invoicing:* Great backend, but lacks the frontend conversational AI to generate the invoice automatically from a WhatsApp message.
  - **OHC Opportunity:** The "Sales Assistant" agent can bridge the gap between a messy customer inquiry ("I need my gutters cleaned on my 2-story house in Austin") and a formal, payable quote. The agent parses the request, queries the owner's pricing model, drafts the formal estimate, and pushes a 1-tap approval card to the owner's feed.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Inquiry via WhatsApp/DM] -->|Webhook| B(Omnichannel Gateway)
      B --> C{The Sales Assistant Agent}
      C -->|Query| D[Tenant Pricing Schema/Knowledge Base]
      C -->|Draft| E[Quote Data Model]
      E --> F[Action Required: Mobile Feed]
      F -->|Owner Taps 'Approve'| G[Stripe Payment Link Generator]
      G --> H[Dispatch Quote to Customer]
  ```

  ### Mobile UX Flow (375px First)
  1. **Notification:** Carlos receives a push notification: "New Quote Drafted: Gutter Cleaning for 123 Main St."
  2. **Review Screen:** Tapping opens a clean card. Top half shows the customer's raw message. Bottom half shows the structured quote:
     - Line Item: 2-Story Gutter Cleaning ($150)
     - Line Item: Travel Fee ($25)
     - Total: $175.
  3. **Action:** Large primary button "Approve & Send (Stripe Link)". Secondary button "Edit Items".
  4. **Post-Approval:** The card transitions to a "Sent" state and moves to the "Pending Payments" tab.

  ### AI Agent Integration Points
  - **The Sales Assistant (Generative AI):** Triggered by intent classification (e.g., "requesting a price"). Uses RAG against the owner's past quotes and pricing documentation to accurately price the requested service.
  - **Payment Orchestration:** Integrates seamlessly with Stripe to convert the approved internal `Quote` model into an actionable `PaymentLink` or `CheckoutSession` requiring a deposit or full payment.

  ### Key Design Decisions
  - **Conversational to Structured Data:** The core value prop is the LLM's ability to take unstructured text and output a highly structured JSON object representing line items, taxes, and totals.
  - **Owner in the Loop:** Quotes are legally binding and represent revenue. The AI *never* auto-sends a quote without explicit owner approval via the feed.

  ## Implementation Prompt
  **User-Facing Outcome:** When a customer texts Carlos asking for a price on a specific job, Carlos opens the OHC app to find a fully itemized quote already generated. He taps "Approve," and the customer immediately receives a professional link to pay the deposit.

  **CUJ & Acceptance Criteria:**
  1. Define the `Quote` and `QuoteLineItem` PostgreSQL models with strict multi-tenant isolation.
  2. Create a webhook ingestion point simulating a customer inquiry.
  3. Implement the `SalesAssistant` agent protocol that takes the raw text, queries a mocked pricing table, and generates the structured `Quote`.
  4. Build the mobile-first (375px) feed card component displaying the drafted quote.
  5. Wire the "Approve" button to a backend service that generates a Stripe Payment Link and updates the quote status.
  6. Provide Playwright E2E tests verifying the full flow from inquiry ingestion to the owner approving the quote in the UI.

  **Priority**: P1
  **Estimated Scope**: Medium

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []