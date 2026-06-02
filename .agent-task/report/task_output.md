issue_title: "[architecture] Autonomous Customer Churn Prediction and Winback Engine"
issue_description: |
  # Issue Brief: Autonomous Customer Churn Prediction and Winback Engine

  ## Problem Statement
  Small business owners (like Leo the music tutor or Priya the boutique owner) often don't realize a customer has stopped buying from them until months later. They are too busy running daily operations to notice when a loyal customer slowly fades away. Without a dedicated marketing team to analyze purchasing patterns and send re-engagement emails, these "at-risk" customers churn silently. By the time the owner notices, it's usually too late. Users need an invisible system that constantly analyzes purchase frequency, predicts when a customer is likely to churn, and automatically drafts personalized, timely winback offers for the owner's simple 1-tap mobile approval.

  ## Research Report
  - **Competitive Audit**:
    - **Shopify/Wix**: Rely heavily on basic "abandoned cart" emails or simplistic time-based triggers (e.g., "Hasn't bought in 30 days"). They do not learn individual customer purchasing cadences (e.g., Customer A buys every week, Customer B buys every 3 months).
    - **Klaviyo/Mailchimp**: Require the owner to understand data modeling to set up complex "Expected Date of Next Order" (EDNO) segments and flows.
    - **OHC Advantage**: OHC's Autonomous AI agents constantly monitor the unified `Customer360` interaction timeline. By observing individual purchase cadences, the AI can detect anomalies (e.g., "Leo's student usually books weekly, but it's been 14 days"). The engine proactively surfaces the insight and drafts a tailored winback message for a seamless 1-tap approval in the owner's mobile feed.
  - **Key Findings**:
    - Acquiring a new customer costs 5x to 25x more than retaining an existing one.
    - Increasing customer retention rates by 5% increases profits by 25% to 95%.
    - Personalized, timely winback offers (sent right when the customer deviates from their normal pattern) have significantly higher conversion rates than generic mass emails.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ CUSTOMER360 : "owns"
      CUSTOMER360 ||--o{ INTERACTION_TIMELINE : "recorded in"
      CUSTOMER360 ||--o{ CHURN_PREDICTION : "has"

      CUSTOMER360 {
          uuid id
          string email
          string phone
          string status "Active, At-Risk, Churned"
          float expected_purchase_cadence_days
      }

      INTERACTION_TIMELINE {
          uuid id
          string source "Order, Booking"
          timestamp occurred_at
      }

      CHURN_PREDICTION {
          uuid id
          float churn_probability
          timestamp next_expected_order_date
          timestamp last_calculated_at
      }
  ```

  ### AI Agent Integration (The Ambassador & The Salesperson)
  The Churn Prediction Engine runs as a background process and notifies the agents when action is needed.

  ```mermaid
  sequenceDiagram
      participant Engine as Churn Prediction Engine
      participant DB as Customer360 DB
      participant Ambassador as The Ambassador (Agent)
      participant Feed as Activity Feed
      participant User as Mobile Dashboard (Owner)

      Engine->>DB: Scan Interaction Timelines (Nightly Job)
      DB-->>Engine: Return deviations from expected cadence
      Engine->>Engine: Calculate Churn Probability (e.g., Jack is at 85%)
      Engine->>Ambassador: Trigger: Winback Opportunity for Jack
      Ambassador->>Ambassador: Analyze Jack's purchase history & preferred channels
      Ambassador->>Feed: Draft Action: "Send 15% discount SMS to Jack"
      Feed->>User: Push Notification: "Jack might be slipping away. Win him back?"
      User->>Feed: 1-Tap Approve
      Feed->>Ambassador: Execute: Send SMS via Twilio
  ```

  ### Key Design Decisions
  1. **Individualized Cadence Learning**: The engine does not use a blanket "30 days = churned" rule. It calculates an `expected_purchase_cadence_days` for each customer based on their historical `INTERACTION_TIMELINE`.
  2. **Zero-Jargon Mobile UI**: The owner never sees complex analytics like "EDNO" or "Churn Probability Models." They simply see an actionable card: "Jack usually books by now. Send a quick check-in?"
  3. **Multi-Tenant Isolation**: Prediction models and customer data are strictly isolated per tenant using PostgreSQL Row Level Security (RLS).
  4. **Proactive, Not Reactive**: The system triggers actions *before* the customer officially churns (when they hit the "At-Risk" threshold).

  ## Implementation Prompt
  **Goal**: Build the "Autonomous Customer Churn Prediction and Winback Engine" to automatically identify at-risk customers and draft re-engagement messages for non-technical small business owners.

  **Core User Journey (CUJ)**:
  1. **The Silent Departure**: Priya the boutique owner has a loyal customer, Emily, who normally buys a new dress every 2 months. It's now been 3 months.
  2. **The Autonomous Prediction**: The nightly background job analyzes Emily's timeline, calculates her deviation, and marks her status as "At-Risk".
  3. **The 1-Tap Winback**: "The Ambassador" agent sees the At-Risk status, drafts a personalized SMS ("Hi Emily! We just got some new dresses in your favorite colors. Here's 10% off if you drop by this week!"), and places it in Priya's Activity Feed. Priya taps "Approve" on her phone, and the SMS is sent.

  **Acceptance Criteria**:
  - **Prediction Logic**: Implement a background job (or AI process) that calculates expected purchase cadences and updates a "Churn Probability" or "Status" (Active -> At-Risk -> Churned) for each customer in `Customer360`.
  - **Agent Drafting**: Connect "The Ambassador" to automatically generate a personalized winback draft when a customer transitions to "At-Risk".
  - **Mobile Feed Integration**: The drafted action must appear in the mobile Activity Feed as a premium, translucent glass card with a clear "Approve & Send" button.
  - **Tenant Isolation**: Ensure all data processing respects strict `tenant_id` boundaries.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
