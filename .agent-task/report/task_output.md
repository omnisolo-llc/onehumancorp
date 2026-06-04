issue_title: "[architecture]_autonomous_ai_loyalty_and_referral_engine.md"
issue_description: |
  # Research Report: Autonomous AI Loyalty and Referral Engine

  ## 1. Problem Statement
  Small business owners like Priya (Boutique Owner) and Fatima (Food Cart Operator) struggle to retain customers and drive repeat business. Existing loyalty programs (like Smile.io or Yotpo) are built for enterprise or require significant manual configuration (points rules, VIP tiers, email campaigns). Solo founders do not have the time to configure points systems, analyze purchase frequency, or manually email lapsed customers. They need a system that invisibly tracks customer value and autonomously rewards loyalty and referrals without any configuration.

  ## 2. Research Report
  - **Market Context**: Customer acquisition costs are rising, making retention critical for SMBs.
  - **Competitor Flaws**: Shopify requires expensive 3rd-party apps for loyalty. Wix and Squarespace have basic built-in tools but require manual setup. None use AI to automatically segment customers or proactively engage them based on purchase behavior.
  - **OHC Opportunity**: Treat loyalty as an invisible layer handled by the "Customer Success" and "Marketing" AI departments. The system automatically identifies "High Value", "At Risk", and "New" customers, and autonomously sends tailored offers or referral codes without the owner lifting a finger.

  ## 3. Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Event: Purchase/Booking] --> B(Event Router);
      B --> C{Customer Identity Graph};
      C -->|Update Profile| D(Loyalty Ledger);
      D --> E{Proactive Engagement Engine};
      E -->|Detects Milestone| F[Customer Success Agent];
      F -->|Generates Offer| G(Notification: SMS/Email);
      D --> H{Referral Engine};
      H -->|Generates Unique Link| I(Post-Purchase Follow-up);
      I -->|Shared by Customer| J[New Customer Acquisition];
  ```

  ### UX/UI Strategy (Mobile-First)
  - **Zero Configuration**: No complex points setup. The owner just toggles "Enable AI Loyalty" in the Marketing tab.
  - **Owner Dashboard (375px)**: A simple glassmorphism card: "AI Loyalty saved 5 at-risk customers this week and generated $450 in repeat sales."
  - **Customer View**: Customers don't need to download an app. They receive Apple/Google Wallet passes with dynamic QR codes for in-store scanning (Fatima's food cart) or apply their phone number online.

  ### AI Agent Integration
  - **Customer Success Agent**: Analyzes purchase frequency. If a regular customer hasn't purchased in 30 days, the agent drafts a personalized "We miss you" discount and asks the owner for 1-tap approval.
  - **Marketing Agent**: Automatically generates and tracks unique referral links for top customers.

  ## 4. Implementation Prompt
  **To the Implementer**:
  Design and implement the core data structures and event processors for the Autonomous AI Loyalty and Referral Engine.
  1. Create a tenant-isolated data model to track customer loyalty state, purchase frequency, and referral linkages.
  2. Implement an event listener that updates the loyalty state upon successful checkout or booking completion.
  3. Create the background job logic for the Customer Success AI to scan for "At-Risk" or "Milestone" customers and draft proactive engagement notifications.
  Ensure all implementations are strictly multi-tenant, tested with 100% coverage, and follow the mobile-first UX principles for any dashboard data exposed. Do not build complex UI rules engines; focus on AI-driven autonomous actions.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
