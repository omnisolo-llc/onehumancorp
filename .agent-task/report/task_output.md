issue_title: "Implement Autonomous VIP Customer Loyalty & Re-engagement System"
issue_description: |
  # Research Report: Autonomous VIP Customer Loyalty & Re-engagement System

  ## 1. Problem Statement
  Small business owners and operators (e.g., Priya the boutique owner, Leo the music tutor) struggle with customer retention. They lack the time and technical expertise to analyze purchase histories, set up point-based loyalty systems, or configure complex CRM marketing triggers. As a result, they lose their most valuable repeat customers (VIPs) to competitors who have dedicated marketing teams. The owner needs a system that proactively identifies valuable customers and suggests personalized re-engagement actions without requiring manual data analysis or campaign configuration.

  ## 2. Research Report
  - **Market Context**: Legacy platforms like Shopify require third-party apps (e.g., Smile.io, Yotpo) to manage loyalty programs. These apps are expensive, add bloat to the storefront, and have complex dashboards that overwhelm non-technical users. Standard CRM tools (Mailchimp, Klaviyo) require the user to manually segment audiences and design email flows.
  - **The OHC Opportunity**: OHC can differentiate by shifting from a "passive tool" to an "active agent". Instead of providing a dashboard for the owner to build a loyalty program, the OHC Sales/Customer Success Agent (The Ambassador/The Promoter) continuously analyzes the unified ledger (payments, bookings) to identify VIPs and dormant customers, autonomously drafting personalized offers for the owner's approval.
  - **Competitor Gaps**:
    - *Shopify/Wix*: Require complex plugin configuration for loyalty rules.
    - *Square*: Offers basic loyalty (points per dollar) but lacks proactive agentic outreach.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Unified Ledger / Order History] -->|Daily Async Job| B[Customer Segmentation Engine]
      B --> C{Identify VIPs & Dormant Users}
      C -->|High LTV / Dormant| D[Sales Agent: The Promoter]
      D --> E[Draft Personalized Offer / Discount]
      E --> F[Owner Unified Agent Feed]
      F -->|1-Tap Approve| G[Send via SMS/Email/WhatsApp]
  ```

  ### Key Design Decisions
  - **No Rules Configuration**: The owner does not configure "10 points per dollar". The AI analyzes Lifetime Value (LTV) and purchase frequency to automatically segment customers into "VIP", "Regular", and "Dormant".
  - **Omnichannel Context**: The engine looks at both online purchases (eCommerce) and in-person Tap-to-Pay (POS) data to build a holistic customer profile.
  - **Agentic Execution**: The system drafts the communication (e.g., "Hey Sarah, we miss you! Here is 20% off your next dress.") and surfaces it as an actionable card in the owner's mobile feed.

  ### Mobile UX Flow (375px First)
  1. **Agent Feed Display**: When the owner opens the OHC app, an Action Card appears: "Loyalty Alert: 5 VIP customers haven't purchased in 60 days. Want to send them a 15% 'We Miss You' discount?"
  2. **Preview & Edit**: Tapping the card expands it to show the AI-drafted message and the list of 5 customers. The owner can edit the message or adjust the discount toggle.
  3. **1-Tap Action**: A large (min 44x44px), thumb-friendly "Approve & Send" button at the bottom of the screen.
  4. **Confirmation**: A brief success state (e.g., green checkmark, haptic feedback) before returning to the main feed.

  ### AI Agent Integration Points
  - **The Data Analyst**: Periodically scans PostgreSQL `orders` and `customers` tables to calculate LTV, Recency, Frequency, and Monetary (RFM) metrics.
  - **The Promoter (LLM)**: Uses the customer's purchase history context (e.g., "Sarah usually buys summer dresses") to generate highly personalized outreach copy.

  ## 4. Implementation Prompt
  **Feature Name**: Autonomous VIP Loyalty & Re-engagement Agent
  **Target Persona**: Priya (Boutique Operator)
  **User-Facing Outcome**: Priya receives proactive suggestions in her feed to re-engage her best customers with personalized offers, driving repeat sales with just one tap.

  **Critical User Journey (CUJ)**:
  1. The system runs a background job identifying a dormant VIP customer.
  2. The Sales Agent drafts a personalized SMS offer for that customer.
  3. The drafted offer appears in Priya's unified agent feed as an Action Card.
  4. Priya reviews the card on her mobile device (375px) and taps "Approve & Send".
  5. The system dispatches the SMS and logs the interaction on the customer's profile.

  **Acceptance Criteria**:
  - Implement a daily CRON/background worker that segments customers based on purchase history.
  - Integrate with the LLM provider to generate contextual re-engagement copy.
  - Create the `triage-card` UI component for the agent feed, ensuring strict adherence to 375px mobile-first design and 44x44px minimum touch targets.
  - Write Playwright E2E tests verifying the entire flow from background job simulation to owner approval.
  - Ensure zero mock data is used in the UI; all customer data must flow from the unified ledger.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
