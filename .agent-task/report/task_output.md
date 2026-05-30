issue_title: "Implement Proactive Inventory Risk Agent (Vigilant Manager)"
issue_description: |
  # Architecture Research for Proactive Inventory Risk Agent

  ## Problem Statement
  Small business owners like Priya (boutique owner) and Fatima (food cart operator) struggle to track inventory actively. A major pain point is when popular items run out of stock during peak hours, leading to missed sales and disappointed customers. They lack the time to constantly monitor dashboards. We need a system where the "Vigilant Manager" (Operations Agent) autonomously tracks inventory velocity and alerts the owner *before* a stockout happens, providing 1-tap resolution options.

  ## Research Report
  - **Shopify:** Relies on third-party apps for advanced inventory forecasting, which adds cost and complexity. Built-in alerts are basic reorder points.
  - **Wix/Squarespace:** Basic manual inventory tracking. No proactive velocity-based forecasting.
  - **Opportunity:** OHC can leapfrog by integrating a predictive model that doesn't just alert at 'zero', but alerts when current sales velocity indicates a stockout is imminent (e.g., "At this rate, you'll run out of vegan cakes by 2 PM").

  ## Design Doc
  ### Mobile UX Flow (375px first)
  1. **Notification:** Owner receives a push notification on their phone: "⚠️ Vegan Chocolate Cake is selling fast! 4 left. Tap to view."
  2. **Alert Screen:** A clean glassmorphism card appears over the dashboard showing:
     - Item: Vegan Chocolate Cake
     - Current Stock: 4
     - Forecast: Sold out in ~2 hours.
  3. **1-Tap Actions:**
     - "Mark as Sold Out (Stop taking orders)"
     - "Contact Supplier (Draft email)"
     - "Dismiss"

  ### Architecture Integration
  ```mermaid
  graph TD;
      A[Order Processing Service] -->|Event: Item Sold| B(Inventory Service);
      B --> C{Velocity Threshold Met?};
      C -- Yes --> D[Operations Agent (Vigilant Manager)];
      D --> E[Generate Alert & Recommended Actions];
      E --> F[Push Notification Service];
      F --> G[Mobile Client (Owner's Phone)];
  ```

  ### Key Design Decisions
  - **Event-Driven:** The system reacts to order events in real-time.
  - **Predictive, not Reactive:** Alerts are based on sales velocity, not just static low-stock thresholds.
  - **Action-Oriented:** Every alert must come with 1-tap actions to resolve the issue immediately.

  ## Implementation Prompt
  Implementer Agent: Develop the backend event listener and the 'Vigilant Manager' agent logic to track inventory velocity. When a threshold is breached, the agent should formulate a structured alert with suggested actions and dispatch it to the notification service. Ensure the payload structure supports the frontend rendering the 1-tap action cards.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
