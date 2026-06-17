issue_title: "[Research] Autonomous Abandoned Cart Recovery Agent"
issue_description: |
  # Research Report: Autonomous Abandoned Cart Recovery Agent

  ## 1. Problem Statement
  Small business owners often lose a significant amount of revenue due to abandoned shopping carts. While enterprise platforms handle this with complex, expensive third-party integrations (like Klaviyo or Mailchimp), OHC’s non-technical owner personas (like Priya the boutique owner or Maya the home baker) need an invisible, zero-configuration solution. They need an AI agent that automatically detects abandoned carts, generates personalized follow-up messages based on the customer's context and store inventory, and recovers the sale without manual intervention.

  ## 2. Research Report
  - **Market Context**: Legacy platforms (Shopify, Wix) rely on app ecosystems for cart recovery, creating an "app tax" and setup friction for merchants.
  - **OHC Opportunity**: By embedding the recovery process natively into the AI Agent layer, OHC can provide a proactive, context-aware "Sales Agent" that works out-of-the-box.
  - **Competitor Gaps**:
    - *Shopify*: Requires paid apps (e.g., Klaviyo) for advanced, personalized recovery flows. Native recovery is basic and non-intelligent.
    - *Wix*: Basic built-in automations, but lacks LLM-driven personalization and proactive inventory awareness.

  ## 3. Design Doc

  ### Architecture Diagram (Concept)
  ```mermaid
  sequenceDiagram
      participant Checkout as Checkout Service
      participant DB as PostgreSQL (Cart State)
      participant Queue as Job Queue (Delayed)
      participant Agent as Recovery Agent (LLM)
      participant Comm as Communication Service (Email/SMS)

      Checkout->>DB: Update cart state (abandoned)
      Checkout->>Queue: Enqueue Delayed Job (e.g., 1hr)
      Queue-->>Agent: Trigger recovery workflow
      Agent->>DB: Fetch cart & user context
      Agent->>Agent: Generate personalized message
      Agent->>Comm: Send message to customer
  ```

  ### Data Model Enhancements
  - **Cart State Tracking**: Ensure `carts` or `checkout_sessions` table tracks `last_activity_at` and `recovery_status`.
  - **Job Queue Delay**: Leverage PostgreSQL for delayed job execution (`run_at` timestamp).

  ### AI Agent Integration
  - **The Recovery Agent**: Monitors the queue for abandoned carts. When triggered, it uses RAG to pull the cart contents, store tone, and user history to draft a highly personalized message (e.g., "Hey, noticed you left the Vegan Chocolate Cake...").
  - **Incentive Generation**: Optional capability to generate a single-use discount code to incentivize completion.

  ### Mobile UX Flow (375px)
  - **Zero Setup**: The feature is enabled by default.
  - **Owner Visibility**: The owner sees a summary card in their Agent Feed: "Sales Agent recovered 3 carts this week ($150)." No complex logic builder is shown.

  ## 4. Implementation Prompt
  **Feature Name**: Autonomous Abandoned Cart Recovery Agent
  **Target Persona**: Priya the Boutique Owner
  **Outcome**: Priya's abandoned online carts are automatically followed up on by an AI agent with personalized messages, recovering lost revenue without any manual configuration.

  **Critical User Journey (CUJ)**:
  1. A customer adds items to their cart on Priya's storefront but leaves without paying.
  2. The system detects the abandonment (e.g., 1 hour of inactivity).
  3. The Recovery Agent generates a personalized email/SMS based on the cart contents and Priya's brand tone.
  4. The customer receives the message, clicks the link, and completes the purchase.
  5. Priya receives a notification in her Agent Feed summarizing the recovered revenue.

  **Next Actions**:
  1. Implement delayed job scheduling in the PostgreSQL job queue to trigger the recovery workflow.
  2. Create the Recovery Agent service to fetch cart context and generate personalized messages using the LLM.
  3. Integrate the agent with the communication layer to send the messages.
  4. Build the Agent Feed summary card to report recovered revenue to the owner.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
