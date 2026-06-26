issue_title: "[Architecture] Autonomous Intelligent Service Routing and Dispatch Engine"
issue_description: |
  **Title:** Autonomous Intelligent Service Routing and Dispatch Engine

  **Problem Statement:**
  Field service operators like Carlos (Handyman) and local operation managers like Jun rely on efficient routing and scheduling to maximize daily revenue and minimize downtime. Currently, small service businesses suffer from manual scheduling, where a new booking might be physically far from the preceding job, leading to excessive travel time and fuel costs. Furthermore, when delays occur or emergency jobs (like a burst pipe) come in, recalculating the day's route and notifying affected customers is a high-friction, error-prone process that pulls the owner away from actually doing the work. They need an invisible engine that treats scheduling and routing as a unified, AI-optimized problem without requiring them to act as a full-time dispatcher.

  **Research Report:**
  *Competitive Analysis:*
  - *Jobber & ServiceTitan:* These are the industry standards for field service management. They provide robust routing features and dispatch boards. However, they are complex, expensive, and require the owner to actively manage a "dispatch board" UI. They feel like enterprise software scaled down.
  - *Shopify/Wix:* Lack native field service routing entirely; they rely on generic booking apps that do not understand physical geography or travel time.
  - *Google Local Services:* Provides lead generation but no operational dispatch or route optimization.

  *Our Opportunity:*
  OneHumanCorp can differentiate by using an "Autonomous Dispatch Engine" that operates invisibly in the background. Instead of a complex drag-and-drop calendar UI, OHC uses the Operations Assistant Agent to dynamically evaluate incoming booking requests against travel times, existing schedules, and traffic patterns. When Carlos gets a new request, OHC doesn't just offer an open time slot; it offers the *optimal* time slot that minimizes travel. If Carlos runs late, the AI automatically drafts SMS updates to subsequent customers. This embodies the "AI Does Useful Work" and "Owner Clarity" values.

  **Design Doc:**
  *Mobile UX Flow & Wireframes (375px First):*
  1. *The Daily Run Sheet:* When Carlos opens the app, the primary view is the "Today" tab. It shows a simplified, vertically scrolling timeline of his day.
  2. *Smart Booking Injection:* When a new urgent request arrives via the unified inbox, a translucent glassmorphism notification card appears: "New urgent request: Leak at 123 Main St. AI suggests inserting at 1:00 PM (adds 10 mins travel). Accept & Notify others?"
  3. *One-Tap Acceptance:* Carlos taps "Accept". The engine recalculates, adjusts the run sheet, and the AI agent automatically sends SMS updates to the 2:00 PM and 3:00 PM customers that their service window is slightly shifted.
  4. *Offline Resilience:* The route and job details (including cached map tiles and customer phone numbers) are synced to the local Hive/SQLite store for offline access in basements or remote areas.

  *AI Agent Integration Points:*
  - *Operations Assistant:* Hooks into the scheduling module to evaluate travel time APIs (e.g., Google Distance Matrix) and proposes optimal slots.
  - *Customer Relationship Assistant:* Drafts and sends localized, context-aware SMS/WhatsApp messages to customers when ETAs change.

  *Key Design Decisions and Why:*
  - *Invisible Optimization:* No traditional "Dispatch Board" for single-operator or small teams. The AI manages the calendar internally; the user only sees the prioritized "Run Sheet" and smart suggestions. This reduces cognitive load.
  - *Proactive Customer Communication:* Automatically linking route delays to customer notifications solves the #1 complaint in field services (technician no-shows/delays) without manual effort from the operator.

  *Architecture Diagram:*
  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC Inbox
      participant Ops Agent
      participant Route Engine
      participant Carlos (Flutter App)
      participant Customer Agent

      Customer->>OHC Inbox: "I have a burst pipe! Can you come today?"
      OHC Inbox->>Ops Agent: Triage Request (Urgent)
      Ops Agent->>Route Engine: Request route injection for today
      Route Engine-->>Ops Agent: Optimal slot: 1:00 PM. Impact: +15m delay for PM jobs.
      Ops Agent->>Carlos (Flutter App): Present suggestion card (Push Notification)
      Carlos (Flutter App)->>Ops Agent: Approves suggestion
      Ops Agent->>Route Engine: Commit schedule change
      Ops Agent->>Customer Agent: Trigger delay notifications
      Customer Agent->>Customer (Subsequent Jobs): "Hi, Carlos is running slightly behind due to an emergency. New ETA is..."
  ```

  **Implementation Prompt:**
  *Role:* Implementer Agent
  *Goal:* Implement the backend scheduling logic and the Flutter frontend Daily Run Sheet that integrates with the Operations Agent for smart booking suggestions.
  *Acceptance Criteria:*
  1. Create the `ServiceRoute` and `RouteStop` data models with multi-tenant isolation.
  2. Implement a unified `DispatchService` in Go that interacts with the AI Ops Agent to calculate the best insertion point for a new job based on existing stops.
  3. Build the Flutter "Daily Run Sheet" view (mobile-first, 375px) using the OHC premium translucent design system.
  4. Implement the "Smart Suggestion" overlay card that allows the owner to accept an AI-proposed schedule change with one tap.
  5. Ensure the route data is cached locally for offline viewing.
  Do NOT prescribe specific routing APIs or database schemas—design the interfaces to allow for future extensibility.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
