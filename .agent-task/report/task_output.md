issue_title: "Intelligent Customer Auto-Responder & Background Agent Task Queue"
issue_description: |
  ## Problem Statement
  Small business owners (like Maya the Baker or Carlos the Handyman) are overwhelmed by the volume of customer inquiries across different channels (Instagram DMs, SMS, email, WhatsApp). They frequently miss sales opportunities because they cannot reply fast enough while doing the actual work. They need an invisible, autonomous agent to intercept customer queries, identify intent, check OHC's internal systems (inventory, order status, bookings), and reply automatically without manual intervention.

  ## Research Report
  Based on the OHC Small Business Platform Market Research & Strategy Report:
  - 38% of SMBs report "Instagram DM Overload" as a primary pain point.
  - The market currently lacks built-in agentic workflows; platforms like Shopify or Wix require third-party add-ons to achieve this, which creates complex setup paralysis for non-technical users.
  - OHC's primary differentiation is invisible AI automation. Implementing an Intelligent Customer Auto-Responder directly solves the #2 highest priority pain point and provides immediate high perceived value for our target personas.
  - To support this and future agent tasks, OHC requires a robust, scalable Background Agent Task Queue using PostgreSQL `SKIP LOCKED` and exponential backoff retry mechanisms to handle asynchronous agent operations reliably.

  ## Design Doc
  ### High-Level Architecture
  - **Unified Inbox Ingestion**: Webhooks from external channels (Instagram, WhatsApp) feed into a unified `Message` table with tenant isolation.
  - **Task Queue**: Inbound messages trigger an asynchronous job in the AI Job Queue (PostgreSQL `SKIP LOCKED` pattern).
  - **Agent Processing**: A background worker dequeues the job, retrieves tenant context, and uses the `Customer & Relationship Assistant` prompt to evaluate the intent (e.g., "Where is my order?").
  - **Execution**: The agent checks the database for order status or inventory, drafts a response, and dispatches it back through the respective channel's API.

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant Customer
      participant External API (IG/WhatsApp)
      participant OHC Ingestion Service
      participant AI Job Queue (Postgres)
      participant OHC Background Worker
      participant LLM Provider (Gemini/MiniMax)
      participant Database (Tenant Data)

      Customer->>External API: "Where is my order?"
      External API->>OHC Ingestion Service: Webhook Delivery
      OHC Ingestion Service->>Database: Save to Unified Inbox
      OHC Ingestion Service->>AI Job Queue: Enqueue Reply Task
      OHC Background Worker->>AI Job Queue: Dequeue (SKIP LOCKED)
      OHC Background Worker->>Database: Fetch Tenant Context & Order
      OHC Background Worker->>LLM Provider: Evaluate Intent & Draft Reply
      LLM Provider-->>OHC Background Worker: "Your order ships tomorrow."
      OHC Background Worker->>External API: Send Reply to Customer
  ```

  ### Mobile UX Flow
  1. Owner opens the OHC app (375px viewport).
  2. The home dashboard ("Work Command Center") displays an "Auto-Replied" summary metric indicating how many queries were handled automatically today.
  3. Tapping the metric opens the Unified Inbox. Messages that were auto-replied have an "AI Handled" badge.
  4. The owner can tap a message to read the AI's response and optionally step in to manually reply if needed.
  5. The UI heavily utilizes macOS-style Translucent Glass materials and clean Ubiquiti UniFi modular dashboard cards.

  ### AI Agent Integration Points
  - **Trigger**: New unread message event in the Unified Inbox.
  - **Memory/Context**: Tenant settings, recent order history, and product catalog are injected into the agent prompt context.
  - **Action**: Generates a text response and marks the message as handled or escalates to the owner if the query is too complex.

  ## Implementation Prompt
  **To the Implementer:**
  Implement the Intelligent Customer Auto-Responder feature along with its required Background Agent Task Queue architecture.

  **User Journey (CUJ):**
  1. A new message arrives via a mocked webhook endpoint simulating an external channel for a specific tenant.
  2. The system securely persists the message and enqueues a background task using the PostgreSQL `SKIP LOCKED` pattern.
  3. A background worker picks up the task, queries the LLM provider using the Customer & Relationship Assistant prompt and tenant data, and generates a reply.
  4. The response is saved back to the Unified Inbox and logged.
  5. The owner views the "AI Handled" message in the mobile-first UI.

  **Acceptance Criteria:**
  - Robust multi-tenant background queue system (PostgreSQL `SKIP LOCKED`) handling agent tasks.
  - Agent successfully intercepts, processes, and auto-replies to standard queries using tenant context.
  - End-to-end Playwright tests verifying the arrival of a message, the background processing, and the correct visual representation in the mobile-friendly (375px viewport) Unified Inbox.
  - Unit test coverage is 100% for new components.
  - Zero mock data in the UI; all states flow from the database.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
