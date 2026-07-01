issue_title: "[architecture] Autonomous Loyalty & VIP Membership Engine"
issue_description: |
  ## Title
  Autonomous Loyalty & VIP Membership Engine

  ## Problem Statement
  Small business owners like Priya (boutique operator) and Fatima (food cart operator) rely heavily on repeat business, but they lack the time and technical tooling to build enterprise-grade loyalty programs (like Starbucks or Sephora). Current point systems are manual, disjointed from their POS/online checkout, and require the owner to remember to reward customers or manage clunky stamp cards. They need an invisible, AI-driven membership engine that autonomously tracks customer lifetime value, upgrades tiers, issues personalized perks (e.g., free delivery, secret menus), and proactively drafts reward notifications, turning casual buyers into loyal VIPs without any manual administrative overhead.

  ## Research Report
  Current SMB loyalty platforms are fragmented and reactive:
  *   **Square Loyalty / Toast:** Mostly static points-based systems where customers must actively claim rewards or staff must remember to prompt them at the register. Very manual and unpersonalized.
  *   **Shopify Apps (e.g., Smile.io):** Require complex setup, confusing points configurations, and don't inherently use AI to adapt rewards based on customer behavior.
  *   **OHC's Opportunity:** Integrate an autonomous loyalty engine directly into the unified customer ledger. The `Retention Agent` monitors purchasing behavior across all channels (online, in-store tap-to-pay, DMs), automatically assigns loyalty tiers, and drafts personalized messages (e.g., "Hey Sarah, you just unlocked VIP status! Your next cake has free delivery.") that the owner can approve with 1-tap.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Checkout as Payment/Checkout System
      participant EventMesh as Event Mesh
      participant Ledger as Customer Ledger
      participant Agent as Retention Agent
      participant Proposal as Action Proposal Ledger
      participant App as OHC Mobile App

      Checkout->>EventMesh: CheckoutCompleted Event
      EventMesh->>Agent: Trigger Loyalty Check
      Agent->>Ledger: Update Customer LTV & Frequency
      Agent->>Agent: Evaluate Tier Thresholds
      opt Tier Upgraded
          Agent->>Ledger: Set New Tier (e.g., Gold)
          Agent->>Proposal: Insert Drafted VIP Welcome Message
      end
      App->>Proposal: Fetch Pending Actions
      Proposal-->>App: "Review & Send VIP Perks" Card
      App->>Proposal: Owner 1-Tap Approves
      Proposal->>EventMesh: Dispatch Message Event
  ```

  ### Mobile UX Flow
  *   **Owner Action Feed:** A unified 375px feed showing proactive AI suggestions. A Glassmorphic card appears: "✨ 3 customers unlocked VIP today. [Review & Send Perks]".
  *   **Customer Profile:** The customer detail view displays a prominent translucent badge (e.g., 🥇 Gold Member) and a timeline of their lifetime value.
  *   **1-Tap Approval:** When the owner taps the card, they see drafted, context-aware messages for each customer, with a primary "Send All" button.
  *   **Simplicity:** Advanced threshold configurations (e.g., Spend $500 -> Gold) are hidden behind an "Advanced Settings" switch. Default tiers are automatically set based on industry heuristics.

  ### AI Agent Integration
  *   The **Retention Agent** leverages the core Vector Memory Layer to suggest perks that match the customer's previous purchase history (e.g., offering a free pastry to a customer who only buys coffee).

  ## Implementation Prompt
  Implement the Autonomous Loyalty & VIP Membership Engine.
  1. Define the data model extensions: add `loyalty_tier` and `lifetime_value` to the multi-tenant `Customer` schema, and define a `LoyaltyTierConfig` schema for the tenant.
  2. Create the `Retention Agent` background worker that listens for `CheckoutCompleted` events, updates the customer's LTV, evaluates tier thresholds, and generates a drafted message in the `Action Proposal Ledger`.
  3. Design a 375px-first Mobile UI component that surfaces these tier upgrades in the owner's Action Feed for 1-tap approval, adopting the macOS-style Translucent Glass materials.
  4. Ensure 100% unit test coverage for the new tier calculation logic and multi-tenant data access. Implement a Playwright E2E test verifying that a simulated checkout triggers a VIP upgrade proposal on the dashboard. No specific LLM vendor APIs should be hardcoded.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
