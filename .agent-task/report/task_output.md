issue_title: "Implement Autonomous AI Quoting & Proposal Engine"
issue_description: |
  ## Overview
  This report investigates the architectural gap and proposes a solution for an "Autonomous AI Quoting & Proposal Engine" within the OneHumanCorp platform. This feature addresses the needs of service-based businesses (e.g., Carlos the Handyman, Nora the Agency Principal) who currently face significant friction in turning inquiries into actionable, paid commitments.

  ## Problem Statement
  Service professionals often lose leads because generating a quote or proposal takes too long. Current tools require them to manually compile details, estimate costs, draft a document, and send it to the client. This is entirely reactive. For an owner operating primarily from a mobile device, this process is tedious and delays revenue capture. Competitors like Shopify and Wix lack robust native quoting systems without heavy reliance on third-party integrations, resulting in a fragmented user experience.

  ## Research Report
  - **Shopify:** Primarily product-focused. Generating quotes requires third-party apps, which break the native checkout flow and add monthly costs.
  - **Wix/Squarespace:** Offer basic contact forms, but lack an integrated AI that can parse an inquiry, check calendar/resource availability, and instantly draft a pricing proposal.
  - **OHC Opportunity:** By integrating quoting natively and leveraging the Operations and Sales AI Agents, OHC can instantly transform a customer inquiry into a draft proposal. The agent parses the request, checks predefined service pricing and resource availability, drafts the quote, and pushes it to the owner for one-tap approval.

  ## Design Doc
  ### Data Model (PostgreSQL)
  - `ServiceItem`: Reusable components of a service (e.g., "Hourly Labor", "Materials", "Diagnostic Fee") with base prices.
  - `Quote`: The main entity, linked to a Customer, containing a list of `QuoteLineItem`s, an expiration date, and status (draft, sent, accepted, rejected, paid).
  - `QuoteLineItem`: Specific items or services included in the quote, with calculated prices.

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Inquiry via IG/Web] -->|Webhook| B(Omnichannel Gateway)
      B --> C{Intent Classifier Engine}
      C -->|Quote Intent| D[The Ambassador Agent]
      D -->|Query Catalog| E[ServiceItem DB]
      D -->|Check Availability| F[The Manager Agent]
      F -->|Availability Confirmed| G[Draft Quote Generator]
      G --> H[Action Required Queue]
      H --> I[Mobile App Feed 375px]
      I -->|Owner Approvs| J[Stripe Deposit Link]
      J --> K[Customer SMS/Email]
  ```

  ### Entity-Relationship Diagram
  ```mermaid
  erDiagram
      Customer ||--o{ Quote : requests
      Quote ||--|{ QuoteLineItem : contains
      ServiceItem ||--o{ QuoteLineItem : templates
      Quote {
          uuid id
          uuid customer_id
          string status
          datetime expires_at
      }
      QuoteLineItem {
          uuid id
          uuid quote_id
          uuid service_item_id
          int quantity
          numeric price
      }
      ServiceItem {
          uuid id
          string name
          numeric base_price
      }
  ```

  ### AI Agent Integration
  - **Sales Agent (The Ambassador):** Intercepts incoming inquiries (via unified inbox or intake form), identifies the intent for a quote, and uses RAG against the `ServiceItem` catalog and past similar quotes to generate a `Quote` draft.
  - **Operations Agent (The Manager):** Verifies availability (e.g., checking if Carlos has time this week) to include proposed scheduling in the quote.

  ### Mobile UX Flow (375px)
  1. **Intake/Inquiry:** Customer submits a request via IG DM or web form ("Need a quote to fix a leaking pipe under the sink").
  2. **Agent Processing:** The Ambassador Agent drafts a quote based on the "Plumbing Diagnostic" service item and typical repair costs.
  3. **Owner Review:** Carlos receives a push notification. Taps to open the OHC app and sees the "Draft Quote" card.
  4. **Approval:** The UI shows the line items. Carlos can tap to adjust prices or simply tap "Approve & Send".
  5. **Client Acceptance:** The client receives a polished SMS/Email link to a mobile-friendly quote page, where they can accept and pay the deposit via Stripe.

  ## Implementation Prompt
  **Feature Name:** Autonomous AI Quoting & Proposal Engine
  **Target Persona:** Carlos the Handyman & Nora the Agency Principal
  **Outcome:** The system automatically drafts quotes for incoming service requests based on the business's predefined pricing catalog. The owner can approve and send the quote with a single tap from their mobile device, leading directly to a deposit payment.

  **Critical User Journey (CUJ):**
  1. A customer submits a service request detailing their issue.
  2. The Sales Agent intercepts the request, queries the `ServiceItem` catalog, and generates a draft `Quote`.
  3. The owner receives a notification and reviews the draft quote on their mobile device (375px viewport).
  4. The owner taps "Approve & Send".
  5. The customer receives a link, views the quote, accepts it, and pays the required deposit via Stripe integration.

  **Acceptance Criteria:**
  - Implement `Quote` and `QuoteLineItem` data models with strict multi-tenant isolation.
  - The Sales Agent must successfully parse an inquiry and generate a structured draft quote.
  - The mobile UI must allow the owner to review, edit, and approve the quote seamlessly.
  - Implement E2E Playwright tests verifying the end-to-end flow from inquiry to approved quote.
  - Must function flawlessly on a 375px viewport.

  **Estimated Scope:** Medium

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
