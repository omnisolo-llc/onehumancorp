issue_title: "Implement Agentic Interactive Quoting & Deposit Engine"
issue_description: |
  # Research Report: Agentic Interactive Quoting & Deposit Engine

  ## 1. Problem Statement
  Custom service and product providers (e.g., Carlos the Handyman, Maya the Baker) frequently deal with unstructured customer requests ("How much for a 3-tier vegan wedding cake?" or "My pipe under the sink is leaking, here is a photo"). Existing platforms like Shopify and Wix are built for fixed-price commodities. They force small business owners into manual back-and-forth communications to generate quotes, slowing down sales and causing lead abandonment. Specialized tools like Jobber are expensive, non-integrated, and lack proactive AI automation.

  ## 2. Research Report
  - **Competitor Gaps**:
    - *Shopify*: Handles custom pricing poorly, requiring manual "Draft Orders."
    - *Wix*: Offers basic static quote forms but requires the owner to manually calculate and send responses.
    - *Jobber / Joist*: Powerful for contractors, but disconnected from the main e-commerce storefront and require high monthly fees. None have autonomous AI quoting capabilities.
  - **The OHC Opportunity**: By introducing a natively integrated "Salesperson" AI agent capable of digesting unstructured requests (text/images) and cross-referencing them against the business owner's pricing heuristics, OHC can reduce the quote-generation time from hours to seconds.

  ## 3. Design Doc
  ### Data Model (PostgreSQL)
  - `Quote`: `id`, `tenant_id`, `customer_id`, `status` (DRAFT, PENDING_APPROVAL, SENT, ACCEPTED, REJECTED, EXPIRED), `valid_until`.
  - `QuoteLineItem`: `id`, `quote_id`, `description`, `unit_price`, `quantity`, `is_optional`.
  - `PricingHeuristic`: `tenant_id`, `service_category`, `base_rate_cents`, `materials_markup_percentage`, `instructions` (e.g., "Always add a 20% buffer for plumbing emergencies").

  ### AI Integration & Flow
  - **Sales & Acquisition Agent**: Ingests the customer request (via Storefront or social DM). Uses Gemini Vision to analyze attached photos. Queries the `PricingHeuristic` table via pgvector/RAG to construct a draft `Quote`.
  - **Operations Agent**: Automatically reserves a provisional time slot in the unified booking calendar (using Redis Redlock) when the quote is sent.
  - **Finance Agent**: Generates a Stripe Payment Link for the required deposit amount upon quote approval.

  ### Mobile UX Flow (375px)
  1. **Owner Dashboard**: Carlos receives a push notification: "New Quote Drafted: Leaky Pipe Repair."
  2. **Review Screen**: Carlos taps the notification and sees a clean, Glassmorphism-styled card with the customer's photo, the AI's itemized breakdown, and the suggested deposit.
  3. **Interaction**: Carlos can tap a line item to adjust the price using the native numeric keyboard, or simply swipe right to "Approve & Send".
  4. **Customer Experience**: The customer receives an SMS/Email with a mobile-optimized Stripe checkout link for the deposit.

  ## 4. Implementation Prompt
  **Feature Name**: Agentic Interactive Quoting & Deposit Engine
  **Target Persona**: Carlos the Handyman
  **Outcome**: Carlos receives unstructured service requests. The AI automatically drafts an itemized quote based on his past jobs and rules. Carlos reviews it in 10 seconds on his Android phone, hits "Send," and collects the deposit via Stripe without typing a single line of text.

  **Next Actions for Engineering**:
  1. Create the `Quote`, `QuoteLineItem`, and `PricingHeuristic` database schemas with row-level tenant isolation.
  2. Implement the Quote Review UI card in the mobile-first frontend (375px optimized), ensuring touch targets are ≥ 44x44px.
  3. Wire the "Salesperson" Agent to intercept custom request events, call the LLM to parse the intent/images, and persist a draft `Quote`.
  4. Integrate Stripe Payment Intents to enforce deposit collection upon quote acceptance.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
