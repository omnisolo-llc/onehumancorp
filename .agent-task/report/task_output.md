issue_title: "Implement AI-Driven Loyalty & Referral Program Agent"
issue_description: |
  ## Problem Statement
  Small business owners (e.g., Priya the boutique operator, Fatima the food cart owner) struggle to build recurring revenue and incentivize repeat business because setting up traditional loyalty points and referral tracking is too complex. They often resort to manual punch cards or clunky third-party apps that don't integrate with their primary customer database or online storefront. The gap is the lack of a native, zero-setup loyalty and referral engine that runs autonomously.

  ## Research Report
  - **Market Context**: Platforms like Shopify require third-party apps for robust loyalty programs (e.g., Smile.io, Yotpo), which adds monthly costs and fragments customer data. Traditional POS systems (Square) offer loyalty but lack proactive AI engagement to drive referrals.
  - **The OHC Opportunity**: By deeply integrating loyalty points and referral tracking into the OHC central ledger and powering it with the Sales/Customer Success Agent, OHC can automatically reward repeat customers and incentivize them to refer friends without the owner lifting a finger.
  - **Competitor Gaps**:
    - *Shopify*: Expensive third-party apps required.
    - *Square*: Basic points system, but passive and not deeply integrated with an autonomous marketing agent.
    - *Wix/Squarespace*: Limited native loyalty capabilities.

  ## Design Doc
  ### Data Model (PostgreSQL)
  - `loyalty_accounts`: Tracks a customer's points balance and tier status.
  - `loyalty_transactions`: Immutable ledger of points earned, redeemed, or expired.
  - `referral_codes`: Unique codes assigned to customers for sharing.
  - `referrals`: Tracks the state of a referral (e.g., clicked, signed up, purchased).

  ### AI Agent Integration
  - **The Promoter Agent**: Monitors customer purchase frequency and automatically drafts personalized messages offering referral bonuses to high-value customers. It also identifies at-risk (churning) customers and automatically offers them a targeted point bonus to return.
  - **The Customer Success Agent**: Handles inquiries about points balance and redemptions seamlessly via chat/DMs.

  ### Mobile UX Flow (375px)
  1. **Customer View (Storefront)**: A unified "My Rewards" tab showing current points, progress to the next reward, and a one-tap button to share their referral link via native OS share sheet.
  2. **Owner View (Dashboard)**: The owner sees a clean "Loyalty Engine" card summarizing total points issued, revenue driven by referrals, and AI-suggested engagement campaigns (e.g., "Drafted a double-points weekend promo for 50 dormant customers - Tap to Approve").

  ## Implementation Prompt
  **Feature Name**: Autonomous Loyalty & Referral Engine
  **Target Persona**: Priya the Boutique Operator
  **Outcome**: Priya's customers automatically earn points on every in-store and online purchase. The system proactively texts her top customers a referral link, rewarding both the referrer and the new customer when a purchase is made. Priya manages the entire program from a single card on her mobile dashboard.

  **Next Actions**:
  1. Design and implement the core `loyalty_accounts`, `loyalty_transactions`, `referral_codes`, and `referrals` schema with row-level security (RLS).
  2. Integrate the loyalty ledger into the checkout flow (both online and POS terminal) for automatic point accrual and redemption.
  3. Extend the Promoter Agent to detect high-value customers and autonomously draft referral campaigns.
  4. Build the mobile-first (375px) "My Rewards" UI for the customer storefront and the "Loyalty Engine" metrics card for the owner dashboard.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
