issue_title: "Implement Agentic Subscription & Recurring Revenue Engine"
issue_description: |
  # Research Report: Agentic Subscription & Recurring Revenue Engine

  ## 1. Problem Statement
  Small business owners (e.g., Leo the Music Tutor, Priya the Boutique Owner) currently lack a frictionless way to offer subscriptions or recurring packages. Existing platforms like Shopify require expensive third-party apps (e.g., Recharge, Skio) which add a complex "app tax", disconnect the user experience, and lack agentic intelligence. Wix and Squarespace offer basic recurring billing, but require significant manual management when payments fail or customers want to pause. Owners need an invisible AI engine that handles the entire lifecycle of subscriptions, from initial offer to automated follow-up.

  ## 2. Research Report
  - **Market Context**: Subscriptions represent a massive growth sector for SMBs. However, the onboarding and management UX for tools like Shopify+Recharge is built for dedicated e-commerce managers, not solopreneurs operating from a 375px mobile screen.
  - **The OHC Opportunity**: By building a native subscription engine into OHC, we eliminate the need for third-party billing apps. More importantly, we can deploy the Finance and Customer Success Agents to actively manage the recurring relationships—drafting retention messages, automatically handling paused subscriptions, and updating inventory dynamically.
  - **Competitor Gaps**:
    - *Shopify*: Relies on third-party apps for subscriptions. Poor integrated mobile management.
    - *Wix*: Native subscriptions exist but are passive; no AI intervention for churn or pauses.
    - *Stripe Billing*: Excellent API, but requires the user to build their own UI or use Stripe's generic portals, which break the unified brand experience.

  ## 3. Design Doc
  ### Architecture & Data Model (PostgreSQL)

  ```mermaid
  erDiagram
      Tenant ||--o{ Product : offers
      Tenant ||--o{ SubscriptionPlan : configures
      Product ||--o{ SubscriptionPlan : link
      Tenant ||--o{ Customer : serves
      Customer ||--o{ Subscriber : maps_to
      SubscriptionPlan ||--o{ SubscriptionSchedule : instances
      Subscriber ||--o{ SubscriptionSchedule : holds
      SubscriptionSchedule ||--o{ RecurringInvoice : generates

      Tenant {
          uuid id PK
          string name
      }
      SubscriptionPlan {
          uuid id PK
          uuid tenant_id FK
          uuid product_id FK
          string interval
          decimal price
      }
      Subscriber {
          uuid id PK
          uuid customer_id FK
      }
      SubscriptionSchedule {
          uuid id PK
          uuid subscriber_id FK
          uuid plan_id FK
          string status
          timestamp next_billing_date
      }
      RecurringInvoice {
          uuid id PK
          uuid schedule_id FK
          string status
      }
  ```

  - `SubscriptionPlan`: The base configuration (e.g., "4 Lessons/Month", "$20/mo Coffee Bean Delivery"). Includes pricing, billing interval, and linked `Product` or `Service`.
  - `Subscriber`: The customer entity linked to a `SubscriptionPlan`.
  - `SubscriptionSchedule`: Tracks the state (active, paused, cancelled) and next billing date.
  - `RecurringInvoice`: The generated invoice linked to the schedule and Stripe integration.

  ### AI Agent Integration

  ```mermaid
  sequenceDiagram
      autonumber
      participant Stripe
      participant OHC_Backend
      participant FinanceAgent
      participant OwnerFeed

      Stripe->>OHC_Backend: Webhook (invoice.payment_failed)
      OHC_Backend->>FinanceAgent: Trigger Failed Payment Protocol
      FinanceAgent->>OHC_Backend: Query subscriber context
      FinanceAgent->>FinanceAgent: Draft personalized retry message
      FinanceAgent->>OwnerFeed: Push Action Card ("Review & Send")
      OwnerFeed->>FinanceAgent: Owner approves draft
      FinanceAgent->>OHC_Backend: Send message to Customer
  ```

  - **Finance Agent**: Automatically monitors Stripe webhook events for failed payments. Instead of just sending a generic email, it drafts a personalized message ("Hi [Name], your card on file for the monthly cake box failed. Update it here: [Link]") and pushes it to the owner's Agent Feed for one-tap approval.
  - **Customer Success Agent**: Identifies subscribers approaching renewal or those who have paused, drafting re-engagement offers.
  - **Operations Agent**: Automatically triggers fulfillment tasks or books recurring calendar blocks based on the active `SubscriptionSchedule`.

  ### Mobile UX Flow (375px)
  1. **Owner Creation Flow**: From the OHC app, the owner creates a new product and simply toggles "Offer as Subscription". They set the frequency (e.g., Weekly, Monthly) via large touch targets.
  2. **Customer Purchase Flow**: A clean, unified checkout experience where the subscription terms are clearly displayed alongside a seamless Stripe deposit/payment flow.
  3. **Owner Dashboard**: The Agent Feed surfaces actionable cards: "3 subscriptions are renewing tomorrow. Ensure inventory is ready." or "1 payment failed. Send follow-up?"

  ## 4. Implementation Prompt
  **Feature Name**: Agentic Subscription & Recurring Revenue Engine
  **Target Persona**: Leo the Music Tutor (needs recurring lesson packages) & Maya the Baker (needs monthly cake box subscriptions).
  **Outcome**: Users can natively offer products or services on a recurring basis. The system autonomously manages billing cycles, and AI agents surface retention and payment-failure actions to the owner's feed.

  **Next Actions**:
  1. Implement the core PostgreSQL data models (`SubscriptionPlan`, `Subscriber`, `SubscriptionSchedule`) with strict multi-tenant isolation.
  2. Integrate Stripe Billing/Invoicing logic to handle the recurring payment cycles seamlessly.
  3. Develop the Mobile-First Owner UI (375px) to create and manage subscriptions without complex menus.
  4. Build the Finance Agent capability to catch failed recurring payments and push actionable drafts to the Agent Feed.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
