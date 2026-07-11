issue_title: "AI-Powered Dynamic Quoting & Invoicing Architecture for Service Operators"
issue_description: |
  ## Title
  AI-Powered Dynamic Quoting & Invoicing Architecture for Service Operators

  ## Problem Statement
  Service business owners like Carlos the Handyman waste significant time transitioning customer inquiries (e.g., "My sink is leaking", often with photos) into actionable, professional estimates and invoices. Traditional tools (like Jobber, Housecall Pro, or Square) provide blank invoice templates but require manual data entry, calculation of material costs, and tedious back-and-forth messaging. This manual friction slows down response times, causing owners to lose leads to faster competitors.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Jobber & Housecall Pro:** Excellent vertical SaaS for field services, but they function as traditional CRMs. The owner must manually create the customer, read the request, build the quote line-by-line, and dispatch it.
  - **Square Invoices:** Great payment collection, but zero context awareness. It's just a digital piece of paper.
  - **Shopify:** Terrible for service businesses. It forces a "product" paradigm on custom services.
  - **OHC Opportunity:** By leveraging the Operations Agent ("The Manager") and the Sales Agent, OHC can ingest an inquiry (text + images), analyze it using vision models, query the owner's price book/historical data, and instantly draft a comprehensive quote. The owner receives a 375px mobile push: "Review Quote for John: Sink Repair ($150)." One tap approves and sends it as a Stripe Payment Link for the deposit.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Inquiry: SMS/DM/Form + Image] -->|Webhook| B(Omnichannel Gateway)
      B --> C{Intent & Vision Analysis Engine}
      C -->|Gemini Vision| D[Operations Agent: The Estimator]
      D -->|Query| E[(Tenant Price Book & History DB)]
      D -->|Draft Quote| F[Action Required Queue]
      F --> G[Mobile App Feed 375px]
      G -->|Owner 1-Tap Approve| H[Stripe Invoice/Payment Link API]
      H --> I[Customer Receives Quote & Pays Deposit]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** A priority card appears: "Action Required: Quote Drafted for John Doe (Leaking Sink)".
  - **Interaction:** Tapping the card opens a split view. Top half shows the customer's message and attached photo. Bottom half shows the AI-drafted line items (e.g., "Plumbing Service Call: $75", "Estimated Materials: $25").
  - **Action:** Primary button "Approve & Send Quote", Secondary button "Edit Line Items".
  - **Visual Design:** Translucent Glass material cards. Clean, native-feeling inputs if editing is needed. No complex CRM terminology.

  ### AI Agent Integration Points
  - **The Estimator (Operations Agent subset):** Triggered by intake forms or DMs classified as "Service Request". Uses Gemini Vision to analyze issue photos, matches them against the `PriceBook` table, and generates a structured JSON quote payload.
  - **The Accountant (Finance Agent):** Once the quote is approved, this agent coordinates with Stripe to generate the Invoice and tracks the deposit payment, automatically updating the booking status when paid.

  ### Key Design Decisions
  - **AI Vision for Quoting:** Allowing customers to send photos that the AI actually uses to estimate parts and labor sets OHC completely apart from legacy tools.
  - **Mobile-First Approval:** The owner should rarely type a quote from scratch. The interface is optimized for reviewing and tweaking AI-generated numbers on a 414px/375px screen.
  - **Integrated Deposits:** The quote is inextricably linked to a Stripe Payment Link for the deposit, turning a quote directly into committed revenue.

  ## Implementation Prompt
  **User-Facing Outcome:** As a service operator, when a customer texts me a picture of a broken door, my OHC app pings me with a pre-written, itemized quote based on my hourly rate and standard material costs. I tap "Approve," and the customer receives a professional estimate with a deposit payment button.
  **CUJ & Acceptance Criteria:**
  1. A test webhook simulates a customer service request (text + image URL).
  2. The AI Agent ingests the payload, calls the Vision API/LLM to categorize the repair, and queries the tenant's `PriceBook`.
  3. A draft `Quote` record is created in the database and surfaced in the Action Required Queue.
  4. The UI displays the drafted quote card.
  5. The owner clicks "Approve & Send", which triggers the creation of a Stripe Invoice and sends the link back to the customer.
  6. Provide Playwright E2E tests: Simulate the intake, log in as the owner, verify the drafted quote card in the mobile feed, tap approve, and assert the final Quote status is "Sent".

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
