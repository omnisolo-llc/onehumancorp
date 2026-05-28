issue_title: "Implement AI-Powered Custom Order & Dynamic Quote Engine"
issue_description: |
  # Problem Statement
  Small businesses like Maya (custom vegan cakes) and Carlos (handyman services) lose potential customers because providing an accurate quote requires back-and-forth messaging. Maya wakes up to Instagram DMs asking, "Can you do a 3-tier vegan cake for 50 people next Saturday?" while Carlos gets texts like, "How much to fix a leaky pipe under my sink?" If they are busy or asleep, the lead goes cold. They need an intelligent, invisible assistant that can ask clarifying questions, generate an accurate quote based on past jobs and predefined rules, and collect a deposit—all instantly, 24/7, without requiring them to lift a finger until the deposit is paid.

  # Research Report
  **Market & Competitive Analysis:**
  - **Shopify & Wix:** Focus heavily on standardized inventory. Custom orders require third-party forms (like Typeform) or clunky app integrations that just dump emails into an inbox. Neither platform has native conversational quoting.
  - **Squarespace:** Good for intake forms, but completely static. No intelligent pricing or dynamic back-and-forth.
  - **HoneyBook / Jobber:** Specialized tools for service businesses that handle quotes well, but they require the business owner to manually draft and send the quote after reading an intake form. They are also separate platforms from the main storefront.

  **The Opportunity:**
  By leveraging LLMs directly in the customer intake flow (via Instagram DMs, SMS, or the OHC storefront chat), OHC can collapse the "Inquiry -> Quote -> Deposit" funnel from days down to minutes. This capability transforms OHC from just a storefront into a proactive sales team for solopreneurs.

  # Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Customer Channels
          IG[Instagram DM] --> OmniGateway[OHC Omni-Channel Gateway];
          SMS[SMS] --> OmniGateway;
          Web[Web Storefront Chat] --> OmniGateway;
      end

      OmniGateway --> ContextRouter[AI Context Router];

      subgraph OHC Agent Swarm
          ContextRouter --> SalesAgent[Sales & Quoting Agent];
          SalesAgent <--> Memory[Episodic Memory / Past Quotes Vector DB];
          SalesAgent <--> PricingRules[Merchant Pricing Rules / Master Catalog];
      end

      SalesAgent --> QuoteService[Quote Generation Service];
      QuoteService --> PaymentGateway[Stripe / Payment Gateway];

      PaymentGateway -- "Deposit Paid" --> OpsAgent[Operations Agent: Schedule & Notify];
      OpsAgent --> MerchantApp[Merchant Mobile App Notification];
  ```

  ### Mobile UX Flow (375px First)
  **Merchant Setup Flow:**
  1. **Dashboard:** Maya opens the OHC app. She taps "Custom Orders & Quotes" from the Operations card.
  2. **Rules Engine:** She sets basic guidelines using natural language. (e.g., "Vegan cakes start at $50. Add $10 per extra tier. Need 3 days notice.")
  3. **Approval Settings:** She toggles a setting: "Auto-approve quotes under $200" or "Require my review before sending payment link."

  **Customer Experience (e.g., Web Chat on Mobile):**
  1. Customer taps "Request Custom Order" on Maya's storefront.
  2. An elegant glassmorphic chat interface opens. Customer types their request.
  3. The AI Sales Agent replies instantly, asking for any missing details (e.g., "What flavor and for how many people?").
  4. Once details are gathered, the chat presents a native, interactive "Quote Card" showing the price breakdown and a "Pay Deposit" button right in the chat stream.

  ### AI Agent Integration Points
  - **Sales Agent (Quoting):** Handles the conversational intake, negotiates (if allowed by merchant rules), and generates the structured quote object.
  - **Finance Agent:** Tracks the quote status, issues the initial deposit invoice, and schedules the final payment reminder.
  - **Operations Agent:** Once the deposit is secured, updates Carlos's or Maya's calendar to block the time/production capacity, and sends a push notification: "New Custom Order Secured: $150 Deposit Paid."

  ### Key Design Decisions
  - **Conversational Intake:** Moving away from static, intimidating web forms to a conversational AI flow increases conversion rates for custom requests.
  - **Invisible Complexity:** Merchants define pricing rules using natural language rather than complex logic builders. The AI translates this into quoting constraints.
  - **Omni-Channel Root:** The quoting engine must not be tied only to the web storefront. It must operate identically across connected social channels (IG DMs).

  # Implementation Prompt
  Implement the AI-Powered Custom Order & Dynamic Quote Engine.
  - **User-Facing Outcome:** Customers can chat with an AI assistant on the merchant's storefront (or connected channels) to describe a custom request. The assistant asks clarifying questions, generates a priced quote based on the merchant's predefined natural language rules, and provides an immediate deposit payment link. The merchant is notified only when the quote requires manual approval or when the deposit is paid.
  - **CUJ (Critical User Journey):**
    1. Merchant configures natural language pricing rules in the mobile app.
    2. Customer initiates a chat requesting a custom service/product.
    3. AI Agent converses to gather required parameters (date, size, scope).
    4. AI Agent generates a structured Quote object and presents a payment link to the customer.
    5. Customer pays deposit; Merchant receives a push notification of the secured job.
  - **Acceptance Criteria:**
    - Merchants can input pricing rules as plain text, which the agent successfully adheres to during quoting.
    - The quoting flow works entirely within a mobile-optimized (375px) chat interface.
    - The system successfully creates a pending invoice and transitions it to paid upon deposit collection.
    - The implementation must support multi-tenant isolation, ensuring one merchant's quoting rules do not leak to another.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
issue_estimated_scope: Large
