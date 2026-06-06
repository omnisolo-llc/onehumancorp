issue_title: "Architect Autonomous Multi-Tenant Loyalty & Referral Engine"
issue_description: |
  # Research Report: Autonomous Multi-Tenant Loyalty & Referral Engine

  ## Problem Statement
  Small business owners like Priya (boutique) and Maya (custom cakes) rely heavily on repeat customers and word-of-mouth referrals. However, setting up a loyalty program or a referral system on existing platforms requires integrating expensive third-party apps (e.g., Smile.io, LoyaltyLion), configuring complex point rules, and manually tracking referral codes. They don't have the time or technical expertise to manage "point economics." They need a zero-configuration, AI-driven loyalty engine that automatically rewards repeat customers and encourages referrals invisibly, driving higher Customer Lifetime Value (LTV) without adding operational overhead.

  ## Research Report
  ### The Small Business "Loyalty Gap"
  Customer retention is critical for SMBs, yet loyalty programs are typically reserved for larger retailers with dedicated marketing teams.
  - **Shopify**: Offers zero native loyalty features. Merchants must install expensive third-party apps ($49-$299/mo) which require complex setup of earning rules, VIP tiers, and redemption flows.
  - **Square**: Has a basic loyalty program, but it's heavily tied to their specific POS hardware and lacks advanced cross-channel (online + offline) AI-driven insights.
  - **Wix/Squarespace**: Limited to basic coupon codes; no true automated loyalty points or referral tracking out of the box.

  ### OneHumanCorp Differentiation
  OHC will provide an **invisible, AI-orchestrated loyalty and referral engine**. Merchants simply toggle "Enable Loyalty & Referrals," and the **Customer Success Agent** and **Sales Agent** take over. The AI analyzes purchase history, auto-generates personalized referral links for top customers via SMS/WhatsApp, and automatically applies loyalty discounts at checkout (both online and via the offline-first POS) based on zero-trust multi-tenant ledgers.

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      MERCHANT ||--o{ LOYALTY_PROGRAM : "configures"
      CUSTOMER ||--o{ LOYALTY_LEDGER : "earns/redeems"
      CUSTOMER ||--o{ REFERRAL_LINK : "shares"
      ORDER ||--o{ LOYALTY_LEDGER : "triggers"
      REFERRAL_LINK ||--o{ ORDER : "attributes"

      %% AI Departments Interactions
      CUSTOMER_SUCCESS_AGENT ||--o{ LOYALTY_LEDGER : "monitors VIP status"
      MARKETING_AGENT ||--o{ REFERRAL_LINK : "distributes via SMS/Email"
      FINANCE_AGENT ||--o{ ORDER : "calculates point value & discount"
  ```

  ```mermaid
  sequenceDiagram
      autonumber
      actor Customer
      participant POS as OHC Mobile POS / Storefront
      participant LoyaltyEngine as Zero-Config Loyalty Engine
      participant FinanceAgent as AI Finance Dept
      participant CS_Agent as AI Customer Success Dept

      Customer->>POS: Completes $100 Purchase
      POS->>LoyaltyEngine: Record Transaction & Identify Customer
      LoyaltyEngine->>FinanceAgent: Calculate Earned Points
      FinanceAgent-->>LoyaltyEngine: Credit 100 Points to Customer Ledger
      LoyaltyEngine->>CS_Agent: Evaluate Customer LTV & Milestones

      alt Is First Time Buyer
          CS_Agent-->>Customer: SMS: "Thanks for visiting! Here's $5 off your next visit. We tracked it automatically!"
      else Is VIP (High LTV)
          CS_Agent->>LoyaltyEngine: Generate Referral Link
          LoyaltyEngine-->>CS_Agent: Link Created (ohc.link/ref123)
          CS_Agent-->>Customer: SMS: "You're a VIP! Share this link with a friend. They get 10% off, and you get a free coffee!"
      end
  ```

  ### UI Wireframes & Screen Flow (375px Mobile-First)
  Adhering to the macOS-style Translucent Glass and UniFi modular dashboard aesthetics.

  **Screen 1: Merchant Setup (Priya's View)**
  - **Header**: "Customer Loyalty & Referrals"
  - **Card Layout**: A single translucent glass card.
  - **The "Magic" Toggle**: "Let AI manage my loyalty program."
  - **Expanded Options (Optional)**:
    - Reward Style: "Points" vs. "Cashback" vs. "Surprise Rewards" (Picker).
    - "Save & Activate" button fixed at the bottom.

  **Screen 2: Customer Experience (Offline/Online Checkout)**
  - **Online Checkout**: Customer enters phone number. The UI instantly updates: "You have $10 in loyalty cash! Tap to apply."
  - **In-Person POS**: Maya taps the customer's profile on her phone. The screen glows indicating a VIP customer. Maya taps "Redeem Free Cake Pop".

  **Screen 3: AI Insights & Dashboard (Merchant View)**
  - **Dashboard Card**: "Loyalty Impact"
  - **AI Insight Chip**: "Your referral program generated 5 new customers this week! The Customer Success Agent sent 12 referral texts to your top buyers."

  ### Mobile UX Flow
  1. **Activation**: Merchant toggles loyalty on with zero configuration. AI sets optimal default point ratios based on industry data.
  2. **Earning**: Customer buys online or taps-to-pay in person. Points are automatically credited to their unified profile via their phone number.
  3. **Distribution**: AI detects the best time to text the customer their point balance or a referral link.
  4. **Redemption**: Frictionless redemption at checkout without needing a separate app or QR code. The system recognizes the customer and offers the discount.

  ### AI Agent Integration Points
  - **Customer Success Department**: Monitors customer milestones (birthdays, 10th purchase) and sends personalized, conversational SMS messages to distribute rewards.
  - **Marketing Department**: Tracks the viral coefficient of referral links and suggests campaigns to the merchant.
  - **Finance Department**: Maintains the immutable, tenant-isolated ledger of points and translates them to currency values during checkout to ensure the merchant remains profitable.

  ### Key Design Decisions
  - **Phone Number as Universal ID**: Eliminates the need for physical punch cards or downloading a separate customer app.
  - **AI-Managed Dunning/Expiration**: The AI automatically warns customers before points expire to drive foot traffic, without the merchant lifting a finger.
  - **Strict Multi-Tenant Isolation**: Loyalty ledgers are strictly scoped per `tenant_id` to prevent cross-contamination or fraudulent point generation. Redlock distributed locks prevent race conditions during concurrent redemptions.

  ## Implementation Prompt
  **To Implementer Agent:**
  Implement the core Autonomous Multi-Tenant Loyalty and Referral Engine.
  - **User Journey (CUJ)**: A merchant (non-technical) toggles on the loyalty program. A customer makes a purchase and automatically earns points, which are tracked via their phone number. The Customer Success Agent automatically sends an SMS with a referral link to a high-LTV customer. The referred friend uses the link to make a discounted purchase, automatically crediting the original customer.
  - **Acceptance Criteria**:
    1. Create a tenant-isolated `loyalty_ledger` table with row-level security.
    2. Implement the engine to calculate and award points on order completion.
    3. Integrate the Customer Success Agent to generate and send referral SMS messages based on customer purchase milestones.
    4. Ensure point redemption works seamlessly at the checkout API level, calculating the correct cart discounts.
    5. Provide a 375px mobile-first UI for the merchant to toggle the feature and view the AI's impact.
  - **Constraint**: The merchant UI must not contain complex rules engines. Expose only high-level toggles (e.g., "Points" vs "Cashback"). AI must handle the exact point economics.

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
