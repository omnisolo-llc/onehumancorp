issue_title: "[architecture] Proactive Autonomous Agents Engine"
issue_description: |
  ## Problem Statement
  Small business owners like Maya (the baker) and Carlos (the handyman) are overwhelmed by the sheer volume of daily tasks required to run their businesses. They often miss important updates, fail to follow up with customers, or forget to reorder inventory because they are busy with their craft. Existing platforms like Shopify or Wix provide tools and dashboards, but they require the user to actively check them and initiate actions. This "pull" model is a significant pain point for non-technical users who need a "push" model where the system works for them.

  ## Research Report
  Our competitive analysis reveals a massive gap in the market:
  - **Shopify:** Relies heavily on third-party apps for automation, which are complex to set up and often require Zapier or similar tools.
  - **Wix/Squarespace:** Offer basic auto-responders but lack proactive intelligence (e.g., they won't automatically suggest a new marketing campaign based on low sales).
  - **OHC Differentiation:** OHC must transition from a reactive tool to a proactive partner. We need an engine that continuously monitors the business state (inventory, sales, customer interactions) and *pushes* actionable suggestions or automatically executes routine tasks (like drafting an Instagram post when inventory is low) on behalf of the user.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      EVENT_BUS ||--o{ PROACTIVE_ENGINE : "Subscribes to events"
      PROACTIVE_ENGINE }|--|| AI_DEPARTMENTS : "Triggers"

      PROACTIVE_ENGINE {
          string tenant_id "Multi-tenant isolation"
          json business_context
          timestamp last_check
      }

      AI_DEPARTMENTS ||--o{ MOBILE_UI : "Pushes notification/draft"
      AI_DEPARTMENTS ||--o{ ACTION_QUEUE : "Enqueues action"
  ```

  ### UI Wireframes & Mobile UX Flow
  - **Global Viewport:** 375px width (Mobile First).
  - **Home Screen Dashboard:** A clean feed of "Suggestions" and "Recent Actions" rather than complex charts.
  - **Action Card:** A translucent glass card displaying a proactive suggestion (e.g., "Drafted an email for 3 abandoned carts").
  - **Interaction:** A simple 1-tap "Approve" button to execute the AI's suggestion, or a "Dismiss" button.

  ### AI Agent Integration Points
  - **Marketing Dept:** Proactively drafts social posts when new inventory is added.
  - **Sales Dept:** Suggests follow-ups for quotes sent but not accepted after 48 hours.
  - **Operations Dept:** Alerts when inventory is low and suggests reorder quantities.

  ### Key Design Decisions
  - **Push, Don't Pull:** The user should rarely need to dig into menus. The most important actions should be brought to them.
  - **1-Tap Approvals:** Minimize cognitive load. The AI does the heavy lifting (drafting, calculating), and the user just approves.
  - **Event-Driven:** The engine must react to real-time events (new order, low stock, abandoned cart) via a robust event bus to ensure timely suggestions.

  ## Implementation Prompt
  **To the Implementer Swarm:**
  Your goal is to build the core event-driven infrastructure for the "Proactive Autonomous Agents Engine".

  **Customer User Journey (CUJ):**
  1. Maya adds a new custom cake design to her catalog.
  2. The Proactive Engine detects the "Item Added" event.
  3. It triggers the Marketing Dept agent, which drafts an Instagram post featuring the new cake.
  4. Maya receives a push notification: "New post drafted for your Vanilla Dream cake. Tap to review."
  5. She taps, reviews the draft in a simple UI card, and taps "Approve".

  **Acceptance Criteria:**
  - Build a robust event subscription mechanism that can listen to core business events (e.g., inventory changes, new orders).
  - Implement the logic to trigger specific AI agents based on these events.
  - Ensure all suggestions are surfaced to the mobile UI via push notifications or a feed.
  - **Mobile Parity:** The UI for reviewing and approving proactive suggestions must pass the Grandmother Test and adhere to the 375px Translucent Glass aesthetic.
  - **Isolation Guarantee:** Strict multi-tenant isolation must be enforced; an event from Tenant A must never trigger an action for Tenant B.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
