issue_title: "Implement Universal Autonomous Staff Management & Local Coordination Mesh"
issue_description: |
  **Mission Queue Protocol Brief**

  **Title:** Implement Universal Autonomous Staff Management & Local Coordination Mesh

  **Problem Statement:**
  Location managers like Jun and operators like Fatima struggle to coordinate multi-shift staff, handle issue escalations, and manage real-time inventory handoffs across multiple devices without heavy management software. Current solutions require staff to log into complex portals or use separate apps (like Sling or Homebase) which are disconnected from the core OHC operations (POS, Orders, Inventory). This causes delayed order prep, uncommunicated supply shortages, and missing owner-ready summaries.

  **Research Report:**
  - **Competitor Systems Audit:**
    - **Square Team Management:** Good POS integration, but rigidly tied to Square's hardware and ecosystem.
    - **Homebase / Sling:** Powerful but siloed. Require separate logins and don't natively trigger actions based on POS inventory or AI customer success events.
    - **Shopify POS Pro:** Has staff roles but lacks AI-driven autonomous task assignment and escalation summaries.
  - **Gaps Identified:** OHC currently lacks an autonomous, location-aware staff coordination layer that uses AI to seamlessly translate customer demand (e.g., a spike in pickup orders) into prioritized staff tasks, supply reminders, and shift escalations, delivered via a mobile-first interface.

  **Design Doc:**

  *Architecture Diagram:*
  ```mermaid
  graph TD
      A[Customer Orders/Demand] -->|Webhook/Event| B(OHC API Gateway)
      B --> C[Operations Agent - The Vigilant Manager]
      C -->|Task Synthesis| D[Staff Management Mesh - CRDT Local Queue]
      D --> E[Staff Mobile App 375px]
      E -->|Task Completion / Handoff| D
      D -->|Sync| F[(Cloud Postgres Ledger)]
      C -->|End of Shift| G[Business Advisory Agent]
      G -->|Summary| H[Owner / Location Manager Dashboard]
  ```

  *Mobile UX Flow (375px First):*
  1. **Shift Dashboard:** Staff member logs in on their mobile device (or a shared terminal). They see a translucent glassmorphism card stack: "Current Tasks", "Active Orders", and "Alerts".
  2. **AI Task Prioritization:** The Operations Agent dynamically reprioritizes tasks. If a large pre-order comes in, a task like "Prepare 15 Falafels" jumps to the top.
  3. **Escalation / Low Supply:** Staff taps a button to flag "Low Cups". This creates an offline-tolerant intent that syncs to the Operations Agent, which alerts Jun (the manager).
  4. **Manager View (Jun):** Jun sees an "Owner-Ready Summary" card summarizing the shift's performance, escalated issues, and supply needs, rather than raw chat logs.

  *AI Agent Integration Points:*
  - **Operations Agent (The Vigilant Manager):** Listens to order volume and inventory drops to auto-generate and assign prep tasks to active staff.
  - **Business Advisory Agent:** Compiles end-of-day staff performance, escalations, and supply shortages into a plain-language summary for the owner.

  *Key Design Decisions & Security:*
  - **Offline-First Synchronization:** Uses CRDTs on the local device so staff can mark tasks complete even in dead zones (e.g., basement stock room).
  - **Zero-Trust Multi-Tenancy:** Staff tokens are strictly scoped via SPIFFE to their specific location and tenant_id.
  - **No-Code Configuration:** Jun does not configure rules. He tells the AI, "Make sure someone checks the bathrooms every 2 hours," and the Operations Agent translates that into a recurring staff task.

  **Implementation Prompt:**
  **Role:** Implementer Agent
  **Goal:** Build the Universal Autonomous Staff Management & Local Coordination Mesh for the OHC mobile app and backend.
  **User-Facing Outcome:** Location managers and staff have a unified mobile view where AI agents automatically assign, prioritize, and summarize daily operational tasks based on real-time business events.
  **CUJ:** Jun tells the system to monitor supplies. During a shift, order volume spikes. The Operations Agent automatically assigns a prep task to a staff member. The staff member marks it complete offline. When they return to network range, the system syncs and Jun receives an end-of-shift summary highlighting the completed task and overall smooth operation.
  **Acceptance Criteria:**
  - Create the `StaffTask` and `ShiftSummary` data models in Postgres with strict multi-tenant RLS.
  - Implement a mobile-first (375px) task feed in the Flutter app using macOS-style Translucent Glass materials.
  - Integrate the offline-first mutation engine to support local task completion.
  - Ensure the Operations Agent can autonomously create tasks via the internal event mesh.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
