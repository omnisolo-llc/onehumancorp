issue_title: "Architecture: AI-Driven Customer Churn Prediction & Proactive Retention Engine"
issue_description: |
  ## Problem Statement
  For business owners like Leo (music tutor) and Priya (boutique operator), losing recurring customers or students without notice directly hurts revenue. Currently, OHC captures transactions, bookings, and messaging but requires the owner to manually review lists or dashboards to notice a lapse in engagement. A tutor might not realize a student hasn't booked in a month, or a boutique owner might miss that a high-value customer hasn't purchased recently. The platform needs an autonomous engine that securely monitors engagement velocity, predicts churn before it happens, and empowers the AI Customer Assistant to proactively draft personalized re-engagement offers.

  ## Research Report
  - **Market Context**: Traditional CRMs like HubSpot or Salesforce provide "last contacted" filters and lead scoring, but they require technical configuration and manual dashboard reading. Small business platforms like Square or Shopify offer automated "Win-back" emails, but these are often generic, rule-based (e.g., "30 days since last purchase"), and easily ignored as spam.
  - **OHC's Unfair Advantage**: Because OHC unifies operations, quoting, booking, and multi-channel messaging via an Assistant-First interface, the churn prediction can act on nuanced signals (e.g., a customer canceling a booking and failing to reschedule, or a sudden drop in message frequency). The KAIROS AI agents can then draft hyper-personalized follow-ups based on the customer's exact history.
  - **Persona Fit**:
    - **Leo**: The agent notices a student missed two weeks and drafts a message: "Hi [Name], missed you at lessons lately! Want to schedule a catch-up session this Thursday?"
    - **Priya**: The agent flags a top-tier customer who hasn't visited in 60 days and drafts a personalized 15% discount offer for a new arrival in their preferred size.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ CUSTOMER : manages
      CUSTOMER ||--o{ ENGAGEMENT_EVENT : generates
      CUSTOMER ||--o{ CHURN_PREDICTION : has
      CHURN_PREDICTION ||--o{ RETENTION_ACTION : triggers

      CUSTOMER {
          uuid id PK
          uuid tenant_id FK
          string name
          datetime last_engaged_at
          float risk_score
      }
      ENGAGEMENT_EVENT {
          uuid id PK
          uuid customer_id FK
          string event_type "Booking | Purchase | Message"
          datetime occurred_at
      }
      CHURN_PREDICTION {
          uuid id PK
          uuid customer_id FK
          float probability
          string primary_factor
          datetime predicted_at
      }
      RETENTION_ACTION {
          uuid id PK
          uuid prediction_id FK
          string status "Draft | Approved | Sent"
          string proposed_message
      }
  ```

  ### Mobile UX Flow (375px)
  1. **Triage Feed Alert**: The owner opens the OHC mobile app. In the "Action Required" feed, a card appears: "Retention Opportunity: 3 customers are slipping away."
  2. **Review Details**: Tapping the card reveals a translucent glass-styled list. For each customer, it shows a "Health Score" and the reason (e.g., "Usually books bi-weekly, but hasn't booked in 4 weeks").
  3. **Agent Action Proposal**: The Customer Assistant AI presents a pre-drafted, personalized message for each at-risk customer via their preferred channel (SMS/WhatsApp/Email).
  4. **1-Tap Execution**: The owner taps "Approve & Send" or taps the text to tweak it slightly before sending.

  ### AI Agent Integration Points
  - **The Vigilant Manager (Operations/Analytics)**: Background worker running within KAIROS that periodically scans the `ENGAGEMENT_EVENT` ledger. It uses lightweight time-series analysis to update the `risk_score` for active customers.
  - **The Silent Ambassador (Customer Success)**: When a `risk_score` crosses a threshold, this agent creates a `RETENTION_ACTION` draft, utilizing the `pgvector` memory store (AutoDream) to recall the customer's favorite items or recent conversation topics to formulate a highly personalized message.

  ### Key Design Decisions
  - **Implicit Signal Tracking**: The engine aggregates existing system events (bookings, POS transactions, messaging) into `ENGAGEMENT_EVENT`s. No new data entry is required from the owner.
  - **Approval-Gated Outreach**: The AI will *never* send a retention message autonomously without the owner's explicit 1-tap approval, maintaining the "Owner Clarity" and trust core values.
  - **Tenant Data Isolation**: Churn prediction models run strictly within the boundary of a single `tenant_id` to comply with OHC's zero-trust security mandate.

  ## Implementation Prompt
  Implement the AI-Driven Customer Churn Prediction and Proactive Retention Engine.
  Develop the data models for `EngagementEvent` and `ChurnPrediction` with strict `tenant_id` isolation. Create a background KAIROS shared task that periodically calculates engagement velocity and generates risk scores for customers. When a customer reaches an "at-risk" threshold, trigger the Customer Assistant AI to draft a personalized retention message using their interaction history. Finally, surface these drafted messages in the mobile-first Triage Feed, allowing the owner to review, edit, and approve the outreach with a single tap. Ensure UI components utilize the standard macOS-style Translucent Glass materials. Add robust Playwright E2E tests validating the end-to-end flow from event ingestion to owner approval.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
