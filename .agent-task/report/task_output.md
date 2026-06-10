issue_title: "Agentic Zero-Friction Loyalty & Rewards System"
issue_description: |
  # Research Report: Agentic Zero-Friction Loyalty & Rewards System

  ## Title
  Agentic Zero-Friction Loyalty & Rewards System

  ## Problem Statement
  Small business owners (like Priya the Boutique Operator and Fatima the Food Cart Operator) struggle to retain customers due to the friction of traditional loyalty programs. Current systems either require physical punch cards (which get lost), app downloads (too much friction for a small business), or manual POS entry (slows down checkout). There is no "zero-friction" way to automatically track purchases, calculate rewards, and proactively notify customers of their perks without manual owner intervention or customer effort. This leads to missed repeat sales and disconnected customer experiences.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Square / Stripe:** Offer basic loyalty features, but they are often opt-in at the terminal and lack proactive, cross-channel engagement (e.g., they don't automatically DM a customer on Instagram if they earned a reward in-store).
  - **Shopify / Smile.io:** Require customer accounts and log-ins, creating friction for in-person or casual purchases.
  - **FiveStars / Toast:** Dedicated systems that require specific hardware or app downloads, isolating the data from the main business operations.
  - **OHC Opportunity:** Utilize the "Omnichannel Customer Memory" and the "Sales & Revenue Assistant". By linking payment methods (tap-to-pay, online checkout) to the unified customer graph, OHC can automatically track loyalty points without any sign-ups. The "Promoter Agent" can then proactively notify customers of their rewards via their preferred channel (SMS, DM, email) and automatically apply discounts to their next purchase, online or in-store.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[In-Store Purchase (Tap-to-Pay)] -->|Webhook| C(Loyalty Engine)
      B[Online Purchase] -->|Webhook| C
      C -->|Identify Customer| D[Unified Customer Graph DB]
      C -->|Calculate Points| E[Ledger/Wallet DB]
      E -->|Threshold Reached Event| F[Event Mesh]
      F --> G[The Promoter Agent]
      G -->|Draft Notification| H[Action Required Queue]
      H -->|Owner Approval / Auto-Send| I[Omnichannel Dispatcher (SMS/DM/Email)]
      D -->|Next Purchase| J(Auto-Apply Reward)
  ```

  ### Mobile UX Flow (375px)
  1. **Customer View:** Receives an SMS or DM: "Hey! You just earned a free coffee at Fatima's Cart. Reply 'Claim' to use it on your next pre-order."
  2. **Owner View (Agent Feed):** Sees a summary card: "The Promoter Agent notified 12 customers about their new rewards today. 4 have already claimed them." (Minimal owner intervention needed, just visibility).
  3. **POS / Checkout View:** When the customer taps their card or checks out online, the system instantly recognizes the tokenized payment method, links it to their profile, and prominently displays any available rewards for 1-tap application.

  ### AI Agent Integration
  - **The Promoter Agent:** Monitors the loyalty ledger. When a customer hits a threshold, it drafts a personalized message based on their past purchases (e.g., "You've earned 20% off your next custom cake, Maya!"). It determines the best channel to send the message based on past interactions.
  - **The Accountant Agent:** Tracks liability of unredeemed points and factors it into financial summaries.

  ### Key Design Decisions
  - **Zero-Friction Tracking:** Loyalty is tied to tokenized payment methods (e.g., card fingerprints from Stripe) or phone numbers, completely eliminating the need for a separate "Loyalty App" or physical card.
  - **Proactive Engagement:** The system pushes rewards to the customer; the customer doesn't have to remember to check a portal.

  ## Implementation Prompt
  **Feature Name:** OHC Agentic Loyalty System
  **Target Persona:** Priya (Boutique Operator), Fatima (Food Cart Operator)
  **Outcome:** Implement a zero-friction loyalty points engine that automatically tracks purchases across all channels via payment tokens. Integrate "The Promoter Agent" to automatically notify customers when they reach reward thresholds and allow 1-tap reward application at checkout.

  **Next Actions:**
  1. Design and implement the `LoyaltyWallet` and `RewardLedger` tables in the database, linked to the `Customer` and `Tenant` entities.
  2. Create the `Loyalty Engine` service that listens to successful payment events (both POS and online) and credits points based on tenant-defined rules.
  3. Integrate `The Promoter Agent` to listen for "Threshold Reached" events and draft/send cross-channel notifications.
  4. Update the Checkout and POS flows to automatically query and surface available rewards for the identified customer.

  **Priority:** P1
  **Estimated Scope:** Medium

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
