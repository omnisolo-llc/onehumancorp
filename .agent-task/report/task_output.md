issue_title: "[feature] Autonomous Subscription & Membership Engine"
issue_description: |
  ## Problem Statement
  Small business owners like Leo (music tutor needing subscription lesson packages) and Priya (boutique owner wanting a loyalty membership) struggle with predictable recurring revenue. Existing solutions like Stripe Billing are too developer-centric, while platforms like Shopify require duct-taping expensive 3rd-party apps (like Recharge) just to get basic subscription functionality. They need an invisible, zero-config recurring billing and membership engine that handles dunning, plan tiering, and member perks automatically.

  ## Research Report
  - **Market Landscape**: The subscription e-commerce market is growing rapidly, but tools for micro-businesses lag.
  - **Competitor Analysis**:
    - **Shopify**: Relies heavily on 3rd-party apps for subscriptions. Complex to set up, disjointed UX, and adds monthly overhead costs.
    - **Stripe**: Powerful but requires developer integration or managing a separate Stripe Dashboard, which breaks our "no manuals, no code" constraint.
    - **Patreon/Substack**: Great for creators but lacks physical product or service booking integrations.
  - **OHC Opportunity**: A natively integrated subscription engine where an AI Finance Agent automatically manages failed payments, card expiries, and membership tier upgrades via conversational SMS or email, completely abstracting the billing complexity away from the business owner.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ MembershipTier : offers
      MembershipTier ||--o{ Subscription : defines
      Customer ||--o{ Subscription : owns
      Subscription ||--o{ Invoice : generates
      Subscription }|--|| AICoordinator : monitored_by

      Tenant {
          string id
          string business_name
      }
      MembershipTier {
          string id
          string name
          float price
          string interval
      }
      Customer {
          string id
          string name
          string default_payment_method
      }
      Subscription {
          string id
          string status
          datetime current_period_end
      }
      Invoice {
          string id
          float amount
          string status
      }
  ```

  ### Mobile UX Flow (375px Viewport)
  - **Creation Flow**: The business owner taps "Add Subscription" from their mobile dashboard. They define the name (e.g., "Weekly Lessons"), price, and billing interval. Clean, macOS-style translucent cards allow easy configuration.
  - **Management**: A unified "Recurring Revenue" dashboard card shows active subscribers, MRR (Monthly Recurring Revenue), and AI Agent activity (e.g., "Agent recovered 3 failed payments this week").
  - **Customer View**: Customers see a "Manage Subscription" portal optimized for mobile, where they can update payment methods via tap-to-pay or Apple/Google Pay with one click.
  - **Grandmother Test**: No terms like "dunning", "proration", or "webhooks". Just "What are you selling?", "How much?", and "How often?".

  ### AI Agent Integration Points
  - **Finance Agent**: Automatically handles dunning. If a payment fails, the agent texts/emails the customer (e.g., "Hi! Your card for Leo's Lessons didn't go through. Tap here to update it.") and retries intelligently.
  - **Marketing Agent**: Identifies loyal customers and suggests to the owner: "Priya, 5 customers buy this coffee every week. Want me to offer them a subscription?"

  ### Technical & Security Integrity
  - **Multi-Tenant Isolation**: All subscription and invoice records are strictly partitioned by `tenant_id` enforced at the data access layer.
  - **Zero Trust**: Services communicating with the payment gateway use short-lived credentials managed via SPIFFE/SPIRE.
  - **Performance Targets**: Under 200ms latency for checkout flows. Background job queue (e.g., Redis-backed) processes recurring billing cycles asynchronously to avoid blocking user requests.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Build the backend data models and core logic for the Subscription & Membership Engine. Implement the ability for a business owner to create a recurring product/service and for a customer to subscribe to it. Integrate this with the AI Finance Agent for automated failed payment recovery. Focus on the user-facing outcome: a seamless, mobile-first subscription management experience. Ensure all database interactions strictly enforce multi-tenancy. Do not prescribe specific database schemas; design them to fit our robust, high-performance architecture. Include end-to-end tests verifying the AI agent's dunning behavior.

  **Estimated Scope**: Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
