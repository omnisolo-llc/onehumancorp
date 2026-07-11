issue_title: "OHC Autonomous Loyalty & Referral Engine"
issue_description: |
  # Research Report: Autonomous Loyalty & Referral Engine

  ## 1. Problem Statement
  Small businesses (like Priya's boutique or Fatima's food cart) rely heavily on repeat customers and word-of-mouth. However, traditional loyalty programs are disconnected—often requiring a separate app (like Smile.io or Yotpo on Shopify) or physical punch cards. They demand manual enrollment and management, meaning most SMB owners never properly launch or maintain them, leading to lost Lifetime Value (LTV).

  ## 2. Research Report
  - **Market Context**: Shopify apps for loyalty (e.g., Smile.io) add $49-$199/month and introduce friction at checkout. Square offers native loyalty, but it's passive (customer must remember to use it). Wix and Squarespace offer rudimentary coupon codes, but no holistic loyalty profiles.
  - **The OHC Opportunity**: Integrate an invisible, agent-driven loyalty system natively into the OHC ledger. The "Customer Success Agent" (The Ambassador) tracks purchases across all channels (online, in-store tap-to-pay, DM orders) and proactively rewards and engages customers.
  - **Competitor Gaps**:
    - *Shopify*: Fragmented experience, relies on third-party apps and explicit customer opt-in for most features.
    - *Square*: Good point-of-sale integration but lacks proactive AI re-engagement.
    - *Wix & Squarespace*: Minimal native loyalty capability.

  ## 3. Design Doc
  ### Architecture diagram
  ```mermaid
  graph TD
      A[Customer Purchase/Action] -->|Event| B(OHC Event Mesh)
      B --> C[Operations Agent]
      C -->|Update Ledger| D[Loyalty DB]
      B --> E[Customer Success Agent]
      E -->|Analyze Milestone| F{Milestone Reached?}
      F -->|Yes| G[Draft Reward Message]
      G --> H[Owner App Feed - 375px]
      H -->|1-Tap Approve| I[Send SMS/DM to Customer]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Feed (Owner App):** A new action card appears: "Sarah has reached VIP status (5 purchases). Send her a 15% off reward?"
  - **Interaction:** The owner can tap "Approve" (sends the AI-drafted message directly to the customer via SMS/Instagram DM) or "Edit".
  - **Customer Checkout View:** A seamless checkout experience where available points/rewards are automatically surfaced based on their phone number/email, requiring no separate login or punch card.

  ### AI Agent Integration Points
  - **Customer Success Agent (The Ambassador)**: Automatically detects when a customer hits a milestone (e.g., 5th purchase) or refers a friend. Drafts a personalized DM or SMS with a reward (e.g., "Hey Sarah! Thanks for referring John. Here's a 15% off link for your next purchase!").
  - **Operations Agent (The Manager)**: Ensures loyalty discounts are properly calculated at checkout and accounted for in the daily sales summary.

  ### Key Design Decisions
  - **Zero Opt-in:** Customers are automatically enrolled based on their global identity (email/phone).
  - **Proactive vs Passive:** Instead of waiting for a customer to check their points balance, the AI proactively notifies the owner to send a surprise reward when a milestone is hit.

  ## 4. Implementation Prompt
  **Feature Name**: Autonomous Loyalty & Referral Engine
  **Target Persona**: Priya the Boutique Owner
  **Outcome**: Priya's customers automatically earn points across both her online store and physical pop-ups. The Ambassador agent proactively texts top customers with rewards, driving repeat business without Priya managing a complex points system.

  **Acceptance Criteria**:
  1. A loyalty event (purchase) triggers an update to a customer's loyalty profile.
  2. The Ambassador agent detects a milestone and drafts a reward message.
  3. The owner sees the draft in their mobile feed and can approve it with one tap.
  4. The reward (discount) is automatically applied to the customer's next checkout.
  5. Playwright E2E test verifying the flow from purchase -> milestone detection -> owner approval -> checkout discount.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
