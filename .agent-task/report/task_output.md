issue_title: "[architecture] Autonomous Subscription & Retainer Billing Engine"
issue_description: |
  # Issue Brief: Autonomous Subscription & Retainer Billing Engine

  ## Problem Statement
  Service providers like Leo (Music Tutor) and retainers for creatives need to bill clients on a recurring basis. Setting up subscriptions is historically complex: it involves connecting third-party apps, configuring Stripe recurring billing logic, setting up dunning (failed payment retries), and manually chasing clients whose cards decline. For a non-technical user, this friction means they often default to manual invoicing, leading to late payments and high churn.

  ## Research Report
  - **Shopify**: Handles subscriptions via expensive third-party apps like ReCharge. Setup is incredibly complex and requires significant technical and pricing configuration.
  - **Wix/Squarespace**: Offer native subscriptions, but they are rigid and not easily tied to service bookings (like a monthly lesson package).
  - **GoDaddy**: Very basic recurring payments but lacks any autonomous recovery or dynamic adjustments.
  - **OHC Advantage**: OHC will use the "Finance & Payments" Agent to autonomously handle the entire subscription lifecycle. It will generate the recurring packages, process the billing, proactively reach out to clients before a card expires, and handle dunning invisibly, surfacing only key "1-Tap" approvals to the owner.

  ## Design Doc
  ### High-Level Architecture
  - **Core Entities**: `SubscriptionPlan`, `CustomerSubscription`, `BillingEvent`, `DunningAction`.
  - **Agent Integrations**:
    - **Finance & Payments Agent**: Manages the Stripe Billing API, tracks revenue, and handles the actual charging logic.
    - **Customer Success Agent**: Drafts and sends personalized emails to customers whose payments fail or cards are about to expire.
    - **Operations Agent**: Automatically provisions or restricts access to the service (e.g., booking slots) based on subscription status.

  ### Architecture Diagram (Mermaid)
  ```mermaid
  sequenceDiagram
      participant Owner as Business Owner (Mobile)
      participant Finance as Finance Agent
      participant CS as Customer Success Agent
      participant Stripe as Stripe Billing
      participant Customer as Client

      Owner->>Finance: 1-Tap: Create $100/mo Tutor Package
      Finance->>Stripe: Configure recurring product
      Customer->>Stripe: Subscribes & Pays
      Stripe--xFinance: Webhook: Payment Failed (Month 2)
      Finance->>CS: Trigger Dunning Protocol
      CS->>Customer: "Hi, your card for guitar lessons failed..."
      CS->>Owner: Activity Feed: "Card failed for John. Auto-recovery active."
  ```

  ### Mobile UX Flow (375px First)
  1. **Creation**: Owner goes to "Services", taps "Add Subscription Package". The AI suggests pricing based on their existing hourly rate (e.g., 4 lessons for the price of 3.5). 1-Tap "Approve".
  2. **Active Management**: Dashboard card shows MRR (Monthly Recurring Revenue) simply: "You make $400/mo automatically."
  3. **Issue Resolution**: If a payment fails, a card appears in the Activity Feed: "John's payment failed. The Ambassador has emailed him a secure update link. Tap to pause his lessons until paid."

  ### Key Design Decisions
  - **Zero Configuration Dunning**: Business owners should never have to configure retry schedules or write failed-payment emails. The CS agent handles this using best practices automatically.
  - **Service-Linked Subscriptions**: Subscriptions must inherently link to the Operations Agent's booking system (e.g., paying $100/mo grants 4 booking credits).

  ## Implementation Prompt
  **User-Facing Outcome**: A service provider can create a recurring subscription package with one tap, and all billing, failed payment recovery, and service access management is handled invisibly by AI agents.
  **Critical User Journey (CUJ)**:
  1. User creates a recurring service package.
  2. Customer subscribes.
  3. System simulates a failed payment.
  4. Customer Success Agent autonomously drafts and sends a recovery email.
  5. Owner receives a status update in their unified activity feed.

  **Acceptance Criteria**:
  - Integrate Stripe Billing for recurring charges, strictly isolated per tenant.
  - The Finance Agent must successfully process webhooks for successful and failed payments.
  - The Customer Success Agent must autonomously generate and send dunning communications.
  - The mobile activity feed must reflect subscription health events.

  **Top 5 Codebase Inconsistencies Discovered**:
  1. Mixed legacy UI: `src/ui/next` exists alongside `src/ui/tauri`, but Next.js is deprecated while still hosting some API routes.
  2. Hardcoded test user credentials in `src/e2e/README.md` and `src/e2e/e2e-seed.sql` pose a security risk if ever leaked to production.
  3. No clear multi-tenant database separation enforcement visible in all API routes (relies on application logic instead of RLS everywhere).
  4. `src/server/services` contains empty or heavily nested modules that could be flattened for better maintainability.
  5. The use of `.md` files for research is robust, but there is no structured index or automated way for the KAIROS orchestrator to parse past decisions.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
