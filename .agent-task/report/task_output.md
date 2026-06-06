issue_title: "[Architecture] Autonomous Customer Loyalty and Rewards Engine"
issue_description: |
  # [Architecture] Autonomous Customer Loyalty and Rewards Engine

  ## Problem Statement
  Small business owners, like Priya the Boutique Owner or Maya the Home Baker, know that retaining existing customers is significantly cheaper than acquiring new ones. However, setting up a loyalty program (points, tiers, punch cards) typically requires integrating complex, expensive third-party apps (like Smile.io or Yotpo on Shopify), configuring intricate earning rules, and manually tracking redemptions. Non-technical users find these systems overwhelming to configure and maintain, leading to abandoned loyalty initiatives and lost repeat business. We need an invisible, zero-configuration loyalty system that automatically tracks customer lifetime value, awards points, and proactively engages customers without the owner lifting a finger.

  ## Research Report
  - **Competitive Baseline (Shopify/Wix):** Shopify and Wix both require third-party app installations or expensive higher-tier plans for robust loyalty features. Apps like Smile.io provide excellent functionality but require the merchant to understand point ratios, VIP tiers, and email configuration.
  - **The Gap:** There is no platform where an AI inherently understands customer purchase frequency and automatically proposes and manages a loyalty strategy out-of-the-box.
  - **OHC Vision:** The OHC Customer Success ("The Ambassador") and Finance & Payments ("The Accountant") agents will automatically track every transaction. The AI configures a tailored loyalty program based on the business type (e.g., a "digital punch card" for Fatima's food cart, or a "spend-based tier system" for Priya's boutique). The system operates seamlessly across both online storefronts and in-person POS.

  ## Design Doc
  ### High-Level Architecture
  1.  **Unified Ledger Integration:** Every transaction (online or POS) routes through the Unified Ledger, emitting an event to the AI Job Queue.
  2.  **Loyalty State Engine:** A multi-tenant ledger that tracks customer points, tier status, and redemption history. It ensures point operations are atomic and idempotent.
  3.  **Agentic Rule Manager:** The Operations and Customer Success agents continuously analyze purchase patterns. They dynamically generate or adjust loyalty rules (e.g., "Double points on slow Tuesdays") and request 1-tap approval from the owner.
  4.  **Omni-Channel UI:** Customers can view their points via a mobile-optimized Link-in-Bio portal, Apple/Google Wallet pass, or instantly at the POS via SMS/Phone lookup.

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Mobile Client & POS (375px)
          App[OHC POS / Online Store] --> API[OHC API Gateway];
      end

      subgraph OHC Backend
          API --> Ledger[Unified Transaction Ledger];
          Ledger --> LoyaltyEngine[Loyalty State Engine];
          LoyaltyEngine --> Postgres[(Postgres Loyalty Tables)];
          Ledger -- Events --> JobQueue[AI Job Queue];
      end

      subgraph Agent Workforce
          JobQueue --> CSAgent[Customer Success Agent];
          JobQueue --> MarketingAgent[Marketing Agent];
          CSAgent --> Postgres;
      end

      subgraph Customer Interactions
          CSAgent -.-> SMS[SMS / Email Rewards Alert];
          MarketingAgent -.-> Promo[Dynamic Promotion Approval];
      end

      style App fill:#1e1e2e,stroke:#cba6f7,stroke-width:2px,color:#fff
      style LoyaltyEngine fill:#1e1e2e,stroke:#a6e3a1,stroke-width:2px,color:#fff
      style CSAgent fill:#1e1e2e,stroke:#f9e2af,stroke-width:2px,color:#fff
  ```

  ### UI Wireframes & Screen Flow (375px First)
  - **Owner Dashboard:** A minimalist Glassmorphism card labeled "Customer Loyalty." It displays a plain-English summary: "You have 42 loyal customers. 15 are close to a reward." A single button says "Send 20% Off to VIPs".
  - **Customer View:** A clean, branded page (accessible via link or QR code) showing a visual progress bar towards their next reward, optimized for touch with a large "Redeem Reward" button.
  - **POS Integration:** When an owner rings up a customer, entering their phone number brings up a frosted-glass overlay indicating available rewards for instant 1-tap application.

  ### Mobile UX Flow
  1. **Transaction:** Customer purchases a coffee from Fatima's cart. Fatima enters the customer's phone number on the 375px POS screen.
  2. **Instant Credit:** A soft haptic buzz and a toast notification: "Fatima earned 10 points! (100 total)".
  3. **Agent Action:** The Customer Success Agent detects the customer has reached a threshold and automatically sends an SMS: "Thanks for visiting Fatima's! You've earned a free side on your next visit. Tap here to view."

  ### AI Agent Integration Points
  - **Customer Success (The Ambassador):** Drafts and sends milestone emails/SMS, tracks customer lifetime value, and handles inquiries about point balances.
  - **Business Advisory (The Advisor):** Analyzes the financial impact of the loyalty program and suggests optimizations ("Your current reward is too easy to reach; let's increase the point requirement by 10%").
  - **Marketing & Advertising (The Promoter):** Incorporates loyalty incentives into social media campaigns ("Join our rewards program today!").

  ### Key Design Decisions
  - **Zero-Config Default:** The system automatically selects a default loyalty structure (spend-based or frequency-based) based on the business category selected during onboarding.
  - **Wallet Pass Integration:** Generating Apple/Google Wallet passes reduces friction for customers to remember their loyalty status, increasing repeat visits.
  - **Event-Driven AI:** Instead of rigid CRON jobs, loyalty events are processed via the asynchronous AI Job Queue to ensure UI performance remains instantaneous.

  ## Implementation Prompt
  **Objective:** Architect and implement the Autonomous Customer Loyalty and Rewards Engine.
  **CUJ & Acceptance Criteria:**
  1. Design the core multi-tenant loyalty ledger schemas in PostgreSQL to track points, tiers, and customer balances safely.
  2. Implement the API endpoints required for the POS and Online Store to apply and redeem points instantly.
  3. Build the event-driven integration where a completed transaction in the Unified Ledger automatically enqueues a job for the Customer Success Agent to process loyalty points.
  4. Ensure the POS UI gracefully handles reward lookups and applications in a 375px viewport with offline-resilient optimistic updates where possible.
  5. Provide comprehensive unit and integration tests covering point accrual, redemption, and multi-tenant isolation.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
