issue_title: "[Research] Autonomous Abandoned Cart Recovery Agent"
issue_description: |
  # Research Report: Autonomous Abandoned Cart Recovery Agent

  ## Problem Statement
  Small business owners like Maya (baker) and Priya (boutique owner) lose a significant portion of potential revenue to abandoned carts. For non-technical users, setting up multi-stage email recovery flows using third-party tools like Klaviyo or Mailchimp is complex, intimidating, and requires technical knowledge (HTML emails, webhooks, trigger delays). These business owners need an automated system that handles cart recovery invisibly, converting lost sales without manual intervention.

  ## Research Report
  - **Market Context**: According to Baymard Institute, the average shopping cart abandonment rate is around 70%. Recovery emails can capture up to 10% of these lost sales.
  - **Competitor Analysis**:
    - **Shopify**: Offers basic built-in recovery emails, but multi-stage flows require complex third-party apps like Klaviyo which add significant monthly costs ("The App Tax") and setup friction.
    - **Wix/Squarespace**: Provide simple automated emails, but lack intelligent timing or dynamic personalization based on user behavior.
  - **OHC Opportunity**: OHC can differentiate by embedding an intelligent "Sales & Acquisition" Agent (The Salesperson) that autonomously monitors abandoned carts, determines the optimal time to reach out, generates personalized, persuasive follow-up messages, and even negotiates (e.g., offering a small, dynamically calculated discount if the cart is high-value).

  ## Design Doc
  ### Architecture
  1. **Event Trigger**: Storefront checkout service emits `CartAbandoned` events (e.g., cart inactive for X hours) to a message queue or event bus.
  2. **Agentic Processing**: "The Salesperson" agent subscribes to these events. It analyzes the cart contents, customer history, and business rules.
  3. **Content Generation**: The agent generates a personalized email/SMS draft using LLM, incorporating product images, descriptions, and a clear call-to-action (CTA).
  4. **Approval/Dispatch**: Based on configuration, the message is either auto-sent or pushed to the business owner's mobile device for 1-tap approval.

  ```mermaid
  graph TD;
      A[Storefront Checkout] -->|Cart Inactive| B(Event Bus: CartAbandoned)
      B --> C[The Salesperson Agent]
      C --> D{Customer Context & History}
      C --> E[LLM Draft Generation]
      E --> F{Auto-send enabled?}
      F -- Yes --> G[Send Email/SMS]
      F -- No --> H[Push Notification to Owner for Approval]
      H -->|Approved| G
  ```

  ### Mobile UX Flow
  - Owner receives push notification: "Agent drafted a follow-up for a $50 abandoned cart. Tap to review."
  - Owner opens the OHC mobile app (375px view) to see the drafted message and customer details.
  - 1-tap "Approve & Send" or "Edit" buttons.

  ## Implementation Prompt
  Implement the Autonomous Abandoned Cart Recovery workflow within the Sales & Acquisition agent department.
  1. Create the event listener for `CartAbandoned`.
  2. Integrate the LLM prompt to generate a personalized recovery message based on cart items.
  3. Implement the notification flow to request owner approval via the mobile-first dashboard (or auto-send based on settings).
  4. Ensure 100% unit test coverage for the event handling and message generation logic. Add a Playwright E2E test verifying the owner approval UI flow.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
