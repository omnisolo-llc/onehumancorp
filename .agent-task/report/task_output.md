issue_title: "[Research] Autonomous Agentic Smart Loyalty & Revenue Retention System"
issue_description: |
  # Research Report: Autonomous Agentic Smart Loyalty & Revenue Retention System

  ## 1. Problem Statement
  Small business owners (like Maya the Baker and Priya the Boutique Operator) struggle with customer retention. While enterprise platforms offer sophisticated, automated loyalty programs and win-back campaigns, SMBs are forced to piece together disjointed third-party email tools (like Klaviyo) and manual discount codes. This fragmented approach leads to missed revenue opportunities because owners lack the time and technical expertise to analyze purchasing patterns and trigger proactive retention campaigns.

  ## 2. Research Report
  - **Market Context**: Platforms like Shopify and Wix rely heavily on manual CRM management or paid third-party apps for loyalty programs. GoDaddy provides basic email marketing but lacks intelligent, automated segmentation based on real-time purchase behavior.
  - **The OHC Opportunity**: By integrating an AI-driven loyalty and retention engine directly into the core platform, OHC can turn passive customer data into active revenue without requiring the owner to build complex logic trees.
  - **Competitor Gaps**:
    - *Shopify*: Requires expensive apps (e.g., Smile.io) and manual campaign setup.
    - *Wix*: Basic built-in loyalty but lacks proactive AI agent drafting and timing optimization.
    - *Square*: Good point-of-sale loyalty, but disconnected from a cohesive, multi-channel AI marketing strategy.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Purchase Event] --> B{Central Ledger PostgreSQL}
      B --> C[Event Ingestion Pipeline Redis]
      C --> D[Retention Intelligence Engine]
      D --> E{Customer Segments}
      E -->|At-Risk| F[Marketing Agent]
      E -->|VIP| F
      F --> G[Draft Retention Campaign]
      G --> H[Owner Feed / Mobile Push]
      H -->|Owner Approves| I[Omnichannel Delivery SMS/Email/IG]
  ```

  ### Mobile UX Flow (375px) & UI Wireframes
  1. **Notification Card**: The owner receives a push notification and a card in their feed: "15 customers haven't ordered in 60 days. Tap to review a win-back offer."
  2. **Approval Screen**:
     - *Header*: "Win-Back Campaign"
     - *Body*: The drafted message (e.g., "Hi [Name], we missed you! Here's 10% off your next custom cake order.")
     - *Audience summary*: "15 At-Risk Customers"
     - *Action Area*: Two large, thumb-friendly touch targets (≥ 44x44px): "Approve & Send" and "Edit Offer".
  3. **Success State**: A brief, translucent glass-styled confirmation toast: "Campaign launched."

  ### AI Agent Integration Points
  - **Finance / Data Agent**: Continuously analyzes `Customer` and `Order` tables to identify behavioral cohorts (e.g., VIPs, Churn Risks, First-time buyers).
  - **Marketing Agent ("The Promoter")**: Receives cohort triggers and automatically drafts personalized, multi-channel messages (SMS, Email, IG DM) using the business's unique voice and current inventory/promotions context.

  ### Key Design Decisions
  - **Zero-Configuration Segmentation**: The system autonomously creates and updates cohorts based on transaction history rather than requiring the owner to build complex rules.
  - **Opt-in Action**: Campaigns are drafted autonomously but require explicit one-tap approval from the owner, ensuring control and building trust.

  ## 4. Implementation Prompt
  **Feature Name**: Autonomous Agentic Smart Loyalty & Revenue Retention System
  **Target Persona**: Maya the Baker
  **Outcome**: Maya effortlessly re-engages dormant customers through AI-drafted, personalized win-back campaigns that she simply approves with one tap on her phone.

  **Critical User Journey (CUJ)**:
  1. Maya logs into the OHC mobile app (375px view).
  2. The system has silently identified 20 customers who previously bought custom cakes but haven't ordered in 3 months.
  3. Maya sees a priority card in her "Work Triage" feed: "Win-back 20 past customers."
  4. She taps the card to view the Marketing Agent's drafted message and the proposed 10% discount code.
  5. Maya taps "Approve & Send".
  6. The system dispatches the messages and tracks conversions, later providing Maya with a plain-language summary of the revenue generated.

  **Acceptance Criteria**:
  - Implementation of the `CustomerCohort` data model with strict multi-tenant isolation.
  - Background worker to classify customers into cohorts based on RFM (Recency, Frequency, Monetary) analysis.
  - Marketing Agent capability to generate personalized draft campaigns triggered by cohort transitions.
  - Mobile-first approval UX conforming to the 375px viewport and 44x44px touch target constraints.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
