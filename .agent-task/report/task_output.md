issue_title: "Implement Intelligent Reorder Triggers for Subscription & Recurring Orders"
issue_description: |
  # Research Report: Intelligent Subscription Replenishment & Reorder Triggers

  ## Executive Summary
  This research investigates a critical operational gap in OneHumanCorp (OHC): the lack of automated, intelligent reordering triggers for recurring subscriptions and low-stock scenarios. Currently, OHC lacks a unified data model linking recurring product usage rates to automated alerts and subscription renewals, heavily impacting small business owners like Maya (the baker) and Priya (the boutique owner). We propose an Agentic Subscription Replenishment architecture that seamlessly monitors usage/sales rates, predicts depletion, and surfaces proactive reorder notifications or directly drafts purchase orders.

  ## 1. Market Mapping & Competitor Discovery (Track 1)
  Competitors like Shopify require expensive third-party apps (e.g., ReCharge or Skio) for subscription logic, which adds significant cost ("App Tax") and configuration friction for non-technical users. Square has basic recurring billing, but it operates in a silo apart from real-time inventory and supply-chain logistics. No platform currently offers an integrated AI assistant that actively models consumer usage and vendor supply chains to trigger "just-in-time" reorders seamlessly.

  ## 2. OHC Gap & Pain Point Identification (Track 3)
  - **Persona Focus:** Maya (Home Baker) & Priya (Boutique Operator). Maya needs reliable recurring revenue for consumable goods without manually checking when a customer is out of coffee or cake supplies. Priya struggles with vendor replenishment when popular recurring items dip below safe thresholds.
  - **The Gap:** The current OHC system handles basic "low stock" events via static thresholds, but it has no intelligence regarding subscription cycles, predictive depletion (burn rates), or automated drafting of purchase orders to replenish the stock required for upcoming subscriptions.

  ## 3. Deep Dive Architecture Design (Track 2 & Track 3)

  ### Data Model Enhancements (PostgreSQL)
  - **`Subscription` & `SubscriptionItem` Entities:** Integrate closely with Stripe Billing, mapping OHC customer profiles to recurring items and billing cycles.
  - **`DepletionModel`:** A lightweight statistical table storing the average burn rate of specific SKUs (e.g., "Customer X consumes 1 unit every 28 days").
  - **`AgentReorderIntent`:** A queue table for the Operations Agent to place draft purchase orders or "Customer Reorder Prompt" messages before finalizing them.

  ### AI Agent Coordination
  - **The Manager (Operations Agent):** Runs a daily job (using `SKIP LOCKED`) to check upcoming subscription renewals against the current `RawMaterial` and `FinishedGood` inventory. If stock is insufficient to fulfill the next 7 days of subscriptions, it autonomously drafts a Purchase Order for the vendor and pings the owner for 1-tap approval.
  - **The Promoter (Marketing Agent):** Monitors one-time buyers. If an item is consumable (based on the `DepletionModel`), it drafts a personalized email around day 25 (for a 30-day burn) offering a "Subscribe & Save" discount.

  ### Mobile-First Implementation
  - **Owner Feed (375px):** Action cards appear natively in the feed: "Action Required: Approve PO for 50 lbs Flour to fulfill 12 upcoming subscriptions."
  - **Customer Portal:** A mobile-optimized web view for customers to "Skip this month" or "Swap flavor," securely linked from email/SMS.

  ## 4. Proposed Implementation Steps & Issue Prompt

  **Feature Name:** Intelligent Reorder Triggers for Subscription & Inventory

  **Target Persona:** Maya the Baker

  **Outcome:** Maya can enable "Subscribe & Save" on her items. The Operations Agent will automatically forecast inventory needs based on upcoming subscription renewals and draft vendor POs in advance, sending Maya a simple 1-tap approval card on her phone.

  **Critical User Journey (CUJ):**
  1. Maya toggles "Enable Subscription" on her signature Vegan Cake mix via the mobile app.
  2. Customers begin subscribing.
  3. The Operations Agent analyzes the upcoming week's subscription deliveries against the `RawMaterial` (Flour, Sugar) inventory.
  4. Detecting a shortfall, the Agent drafts a `PurchaseOrder` for Maya's supplier.
  5. Maya receives a push notification on her 375px viewport: "Approve PO for Flour to secure next week's subscriptions?"
  6. Maya taps "Approve." The PO is finalized, and inventory forecasts are updated.

  **Next Actions for Engineering:**
  - **Step 1:** Implement the `Subscription` and `DepletionModel` PostgreSQL tables with row-level security for tenant isolation.
  - **Step 2:** Extend the `SupplyChainApi` and Operations Agent to include forecasting logic that aggregates upcoming subscriptions and checks against current `RawMaterial` thresholds.
  - **Step 3:** Implement the mobile-first (375px) Action Card in the Owner Feed to approve Agent-drafted Purchase Orders.
  - **Step 4:** Ensure E2E Playwright tests cover the end-to-end "Draft to Approval" flow for a reorder intent.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
