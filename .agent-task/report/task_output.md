issue_title: "[Architecture] Autonomous Customer Loyalty & Retention Engine"
issue_description: |
  # Research Report: Autonomous Customer Loyalty & Retention Engine

  ## Problem Statement
  Small business owners like Priya (Boutique Owner) and Maya (The Home Baker) struggle with customer retention. They acquire customers through social media or word-of-mouth but lack the time and technical expertise to implement complex loyalty programs or automated follow-ups. Traditional platforms (Shopify, Wix) require expensive third-party apps (like Smile.io or Yotpo) that are difficult to configure and manage on a mobile device. OHC needs an invisible, autonomous loyalty engine that works quietly in the background to recognize repeat customers, issue rewards, and re-engage dormant buyers without requiring manual setup.

  ## Research Report
  - **Competitor Gap:** Shopify's loyalty solutions are entirely app-based, adding $50-$200/month and significant configuration overhead. Wix has basic loyalty points but requires manual rule creation. Neither integrates deeply with AI.
  - **Persona Focus:** Priya needs to automatically text a VIP discount to customers who have spent over $500. Maya needs to re-engage customers who ordered a birthday cake exactly 11 months ago.
  - **Proposed Solution:** A zero-config Loyalty Ledger built into the KAIROS Orchestrator. The Customer Success Agent and Marketing Agent collaborate to track purchase history, assign VIP tiers automatically, and generate personalized, context-aware re-engagement messages via the Omnichannel Inbox.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      ORDER ||--o{ LOYALTY_ENGINE : "Triggers"
      LOYALTY_ENGINE }|--|| CUSTOMER_PROFILE : "Updates"

      LOYALTY_ENGINE {
          string spiffe_identity "Zero Trust routing"
          string tenant_id "Multi-tenant isolation"
      }

      LOYALTY_ENGINE ||--o{ POINTS_LEDGER : "Records"
      CUSTOMER_PROFILE ||--o{ CS_AGENT : "Analyzes for re-engagement"

      CS_AGENT }|--|| NOTIFICATION_ROUTER : "Dispatches"

      POINTS_LEDGER {
          string tenant_id
          string customer_id
          int points_balance
          string tier
      }
  ```

  ### Mobile UX Flow (375px First)
  1. **Zero Setup:** The loyalty engine is active by default. No complex rules to configure.
  2. **Customer View:** A customer completes a purchase and sees a beautiful, translucent glassmorphic card: "You just unlocked Gold Status! 10% off your next order."
  3. **Owner View:** Priya opens the OHC app. The Business Advisory Agent shows a plain-language summary: "3 VIP customers returned this week. AI sent them a thank-you note."
  4. **Manual Override:** In the customer's profile, Priya can tap a single button to "Send Bonus Reward" using a native mobile dial to select the amount.

  ### AI Agent Integration
  - **Customer Success (CS) Agent:** Monitors the `POINTS_LEDGER`. If a customer hasn't purchased in 6 months, it drafts a personalized re-engagement message (e.g., "Hi Sarah, we have some new vegan cakes we think you'd love!").
  - **Finance Agent:** Automatically applies loyalty discounts at checkout seamlessly.

  ### Key Design Decisions
  - **Invisible Tiers:** Use a simplified, default 3-tier system (e.g., Member, VIP, Elite) based on lifetime value rather than complex point conversions.
  - **Event-Driven:** Every checkout event flows through the NATS Event Mesh to instantly update the loyalty ledger.
  - **Multi-Tenant Security:** Strict row-level security on the `POINTS_LEDGER` to ensure tenants cannot see each other's customer data.

  ## Implementation Prompt
  **To Implementer Agent:**
  Implement the underlying data model and background event listeners for the Autonomous Customer Loyalty Engine.
  1. Create the `POINTS_LEDGER` schema in PostgreSQL with strict row-level security enforcing `tenant_id` isolation.
  2. Implement an event consumer that listens for `CheckoutCompleted` events on the NATS mesh and updates the customer's loyalty tier.
  3. Create a scheduled background job using the KAIROS worker queue that identifies dormant customers and queues a task for the Customer Success agent to draft a re-engagement message.
  4. Ensure all logic is fully covered by unit tests mocking the database and event mesh.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
