issue_title: "[Research] Autonomous Loyalty & VIP Retention System Architecture"
issue_description: |
  # Research Report: Autonomous Loyalty & VIP Retention System Architecture

  ## Executive Summary
  Small business owners often struggle with customer retention because they lack the time and tooling to engage their best customers. Current platforms require manual segmentation and rigid point systems. This report proposes an autonomous, agent-driven loyalty system that automatically tracks customer lifetime value across all channels (online, POS, social DMs) and proactively rewards and re-engages VIPs, shifting the burden from the owner to the AI.

  ## 1. Market Context & Competitor Discovery (Track 1)
  **The Problem:**
  Customer acquisition costs are rising, making retention critical. However, SMBs rarely have dedicated marketing teams to run loyalty programs.
  - **Shopify/Wix:** Loyalty programs are mostly bolted-on via third-party apps (e.g., Smile.io). They require extensive setup (defining points per dollar, tiers) and are often disjointed from in-store POS.
  - **Square:** Offers integrated loyalty, but it's largely passive (waiting for the customer to return and claim points) rather than proactive.

  **OHC Opportunity:**
  OHC can integrate loyalty directly into the Universal Ledger and Customer Graph. More importantly, we can employ a "Retainer Agent" to monitor purchasing habits and autonomously draft re-engagement campaigns or surprise rewards for VIPs, requiring only a 1-tap approval from the owner.

  ## 2. OHC Gap & Persona Pain Points
  **Persona Focus:** Priya (Boutique Owner) and Maya (Home Baker)
  - **Priya's Pain:** She notices regulars coming in less frequently but doesn't have time to sift through sales data to see who hasn't visited in 3 months and email them a discount.
  - **Maya's Pain:** She wants to reward her best custom-cake buyers with a free box of cupcakes on their birthday, but tracking this manually in a spreadsheet is impossible as she grows.
  - **The Gap:** OHC currently tracks orders and customers, but lacks an automated, agent-driven retention engine that connects purchasing velocity to proactive rewards.

  ## 3. Deep Dive Architecture Design (Track 2 & 3)

  ### Data Model & Invariants
  - **Entity `LoyaltyProfile`:** Linked to the `Customer` entity. Tracks lifetime value (LTV), purchase frequency, last purchase date, and accumulated "Credits" (a simpler concept than arbitrary points, mapping directly to fiat value or specific perks).
  - **Entity `RewardLedger`:** An append-only ledger tracking the issuance and redemption of rewards to ensure transactional integrity across online and POS channels.
  - **Event Bus Integration:** Every `OrderCompleted` event across online or terminal POS flows into the loyalty evaluation engine.

  ### AI Department Coordination
  - **The Retainer Agent (Customer Success/Marketing):**
    - Runs periodic background evaluations of the `Customer` graph.
    - Identifies "Churn Risk" VIPs (e.g., top 10% spenders who haven't purchased in 2x their average buying cycle).
    - Identifies "Milestone" events (e.g., 10th purchase, 1-year anniversary).
    - Drafts personalized messages (email or SMS via Twilio) offering a unique, auto-generated discount code or free gift.
    - Surfaces an "Action Card" to the owner's Agent Feed for approval.

  ### Mobile-First UX Flow (375px)
  1. **Owner View (Agent Feed):** Priya opens the OHC app. The feed shows a card: "3 VIPs haven't visited in 60 days. Approve 15% win-back offer?"
  2. **Approval:** Priya taps "Approve." The Retainer Agent sends the personalized messages.
  3. **Customer View (POS/Online Checkout):** When a VIP checks out (via Stripe Terminal in-store or online cart), the system automatically detects their identity and surfaces available rewards for 1-tap application, eliminating the need for physical punch cards or QR codes.

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Order Complete (Online/POS)] -->|Webhook/Event| B(Event Bus)
      B --> C{Loyalty Engine}
      C -->|Update LTV & Credits| D[(Customer Graph & Reward Ledger)]
      E[Background Cron] -->|Evaluate| D
      E -->|Identify VIP / Churn Risk| F[The Retainer Agent]
      F -->|Draft Contextual Reward| G[Action Queue]
      G --> H[Mobile Agent Feed 375px]
      H -->|Owner 1-Tap Approve| I[Omnichannel Dispatcher]
      I -->|SMS/Email| J[VIP Customer]
      J -->|Redeem at Checkout| K[Checkout Session/Terminal]
      K -->|Verify & Deduct| D
  ```

  ## 4. Implementation Prompt
  **Feature Name:** Autonomous VIP Retention System
  **Target Persona:** Priya the Boutique Owner

  **Outcome:** A system that automatically tracks customer value across online and in-store purchases and proactively suggests personalized reward campaigns to retain VIPs, without requiring manual data analysis from the owner.

  **Critical User Journey (CUJ) / Acceptance Criteria:**
  1. Simulate an online order and an in-store (Terminal) order for the same customer to verify their `LoyaltyProfile` updates accurately.
  2. Trigger the Retainer Agent background job. It must successfully identify a simulated "churn risk" VIP based on their last purchase date.
  3. The agent must draft a targeted re-engagement message and insert it into the Agent Feed.
  4. Build Playwright E2E tests verifying that the owner can view this draft on a 375px mobile viewport, approve it, and that the mocked customer receives the communication.
  5. Ensure strict tenant isolation on all new tables (`LoyaltyProfile`, `RewardLedger`).

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []