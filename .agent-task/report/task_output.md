issue_title: "[Research] AI Agentic Subscriptions & Predictive Replenishment Architecture"
issue_description: |
  ## Issue Title
  [Research] AI Agentic Subscriptions & Predictive Replenishment Architecture

  ## Problem Statement
  Small business owners with recurring or consumable models (like Maya the baker, or a local coffee roaster) struggle to manually track and forecast when customers need to reorder or manage their active subscriptions. Traditional platforms require manual subscription app installations, complex recurring billing logic configurations, and separated analytics. Customers often forget to restock, leading to lost recurring revenue. OHC currently lacks a unified, agent-driven subscription and intelligent replenishment architecture that operates silently without owner configuration.

  ## Research Report
  - **Market Context**: Platforms like Shopify require third-party apps (e.g., Recharge, Skio) for robust subscription management. These apps add $100-$300/mo overhead, introduce disjointed UI experiences, and heavily rely on human-defined rules.
  - **The OHC Opportunity**: Integrate recurring billing natively via Stripe Billing and orchestrate it using the OHC `Sales & Revenue Assistant` and `Customer Success Assistant`.
  - **Persona Fit**:
    - *Maya (Baker)*: Can offer "Cake of the Month" clubs.
    - *Fatima (Food Cart)*: Can offer prepaid meal plans.
    - *Priya (Boutique)*: Can offer predictive replenishment for standard staple items.

  ## Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      A[Customer/Subscriber] -->|Purchases Subscription| B[OHC Checkout/Stripe API]
      B --> C[Central Ledger PostgreSQL]
      C --> D[AI Event Bus / Redis Queue]
      D --> E[Sales & Revenue Assistant]
      D --> F[Customer Success Assistant]
      E -->|Tracks Revenue & Invoicing| G[Owner Financial Dashboard]
      F -->|Predictive Restock Alert/Draft| H[Customer Inbox/SMS]
      H -->|Customer Approves| I[Stripe Re-bill]
      C -->|Trigger Fulfillment| J[Operations Assistant]
      J --> K[Owner Work Feed]
  ```

  ### Mobile UX Flow (375px First)
  1. **Owner View**: The owner opens the OHC mobile app. The "Today's Priorities" feed (Work Triage) highlights: "3 upcoming subscription renewals to prepare." No complex subscription dashboards required.
  2. **Customer View**: Customers receive an SMS/WhatsApp from the OHC Customer Success Agent (acting on behalf of the owner): "Hi, it looks like you might be running low on coffee. Want me to process a refill and ship it tomorrow? Reply 'Yes' to confirm."
  3. **Action**: The customer replies 'Yes'. The agent invokes the Stripe API to charge the saved payment method, records the transaction in the Central Ledger, and creates a fulfillment task for the owner.

  ### AI Agent Integration Points
  - **Work Triage (Feed)**: Displays upcoming subscription fulfillment needs and revenue projections organically.
  - **Customer & Relationship Assistant**: Monitors purchase history frequency, identifies "restock" patterns, and proactively drafts/sends SMS or email replenishment prompts.
  - **Sales & Revenue Assistant**: Handles the background logic of updating Stripe subscriptions if a customer wants to pause or skip a month via chat.

  ## Implementation Prompt
  Implement the core data models and service layer for AI Agentic Subscriptions & Predictive Replenishment.
  1. **Data Model**: Design PostgreSQL schemas for `Subscriptions`, `SubscriptionPlans`, and `FulfillmentSchedules` with strict multi-tenant isolation (`tenant_id`).
  2. **Service Layer**: Implement the backend logic to interface with Stripe Billing (or similar) to handle recurring charges and webhooks.
  3. **Agent Integration**: Expose a tool for the `Customer Success Assistant` to query a customer's subscription status and past purchase frequency to predict replenishment dates.
  4. **Acceptance Criteria**:
     - The system must correctly identify customers due for a restock based on historical data.
     - The AI Agent must be able to use a tool to draft a restock message.
     - The data model must fully support row-level security for tenant isolation.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
