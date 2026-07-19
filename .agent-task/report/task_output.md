issue_title: "Architecture & Design: Agentic Zero-Touch Loyalty & Referral Engine"
issue_description: |
  # Mission Queue Protocol: Agentic Zero-Touch Loyalty & Referral Engine

  ## Problem Statement
  Small business owners (like Priya the boutique owner and Maya the baker) know that repeat customers and word-of-mouth referrals are their most profitable revenue sources. However, setting up a loyalty or referral program on legacy platforms like Shopify requires installing expensive third-party apps (e.g., Smile.io, Yotpo, LoyaltyLion) which cost $50-$200/month. Furthermore, these apps require complex configuration of points, tiers, and email triggers that non-technical owners find overwhelming. As a result, they either abandon the setup or run passive programs that customers forget about.

  ## Research Report
  - **Competitor Analysis**:
    - **Shopify**: Relies heavily on the "App Tax" for loyalty. Apps are disconnected from the core unified customer profile and require manual rules configuration.
    - **Wix/Squarespace**: Basic native loyalty, but relies on passive points accumulation. It expects the customer to log in and redeem, which often sees low engagement.
    - **Square**: Has a built-in loyalty program that works well in-store (tap-to-pay) but is disconnected from proactive digital marketing and omnichannel messaging.
  - **OHC Opportunity**: OHC can eliminate the friction by introducing an Agentic Zero-Touch Loyalty Engine. Instead of forcing the owner to configure points rules, the "Growth Agent" (Marketing Department) autonomously tracks customer lifetime value (LTV), purchase frequency, and milestones across all channels.
  - **Agentic Advantage**: The AI proactively identifies when a customer becomes a "regular" and drafts a personalized WhatsApp/IG DM or email with a custom reward or referral link. The owner simply approves the drafted message in their daily Action Feed.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Unified Ledger / Orders] -->|Event Stream| B(Customer Identity Graph)
      B --> C{LTV & Milestone Evaluator}
      C -->|Threshold Met| D[Growth Agent]
      D -->|Generate Referral Code| E[Promotions API]
      D -->|Draft Personalized Msg| F[Action Required Queue]
      F --> G[Mobile App Feed 375px]
      G -->|1-Tap Approve| H[Omnichannel Dispatcher]
      H --> I[Customer via WhatsApp/Email/IG]
  ```

  ### Mobile UX Flow (375px First)
  1. **Home Feed (Mobile)**: Maya opens the OHC app. The top Action Card reads: "Growth Opportunity: Sarah just placed her 3rd order! Send a 10% referral code to reward her?"
  2. **Interaction**: Maya taps the card. She sees a brief context: "Sarah's LTV is $150. AI Draft: 'Hi Sarah, thank you so much for your 3rd cake order! Here is a special link to share 10% off with a friend, and you get $10 off your next order too!'"
  3. **Action**: Maya taps the large "Approve & Send" button (≥ 44x44px touch target). The system handles the promo code creation and message dispatch.
  4. **Settings (Advanced)**: For owners who want control, a simple toggle screen allows them to adjust the aggressiveness of the Growth Agent or the default discount percentage.

  ### AI Agent Integration Points
  - **The Growth Agent (Marketing)**: Monitors the `Customer Identity Graph` for transaction milestones (e.g., 3rd purchase, 1-year anniversary, high LTV). It integrates with the Promotions API to generate unique referral codes and uses the LLM to draft highly contextual, brand-aligned messages.

  ### Key Design Decisions
  - **Proactive vs. Reactive**: No point ledgers or complicated reward tiers. The system focuses on surprise-and-delight moments driven by AI.
  - **Zero Configuration**: The feature works out of the box. The agent learns the owner's average order value and suggests appropriate rewards.

  ## Implementation Prompt (For Engineering Swarm)
  **Feature Name**: Agentic Loyalty & Referral Engine
  **Target Persona**: Priya (Boutique Operator) and Maya (Home Baker)
  **Outcome**: The system autonomously identifies loyal customers and drafts personalized referral/reward messages for the owner to approve, driving repeat sales without requiring any setup of points or tiers.
  **CUJ & Acceptance Criteria**:
  1. Trigger an order event that pushes a customer's total order count to 3.
  2. Verify that the LTV & Milestone Evaluator detects this and places an event in the queue.
  3. The Growth Agent must consume the event, generate a unique promotion code via the existing API, and draft a message.
  4. The drafted message must appear in the owner's mobile Action Feed as an approval card.
  5. The owner (via E2E Playwright test on a 375px viewport) clicks "Approve".
  6. The system dispatches the message to the mocked external channel and records the promo code as active.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
