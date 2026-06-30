issue_title: "[Architecture] Proactive Unified Agent Feed for Mobile Operations"
issue_description: |
  # Title: Proactive Unified Agent Feed for Mobile Operations

  ## Problem Statement
  For non-technical owners like Maya (the baker) or Fatima (the food cart operator), traditional "dashboards" are overwhelming and useless on a 375px mobile screen. They don't have time to navigate through complex menus, check multiple inboxes, and look at analytical charts to figure out what needs to be done. They need a system that tells them exactly what requires their attention right now, why it matters, and provides a 1-tap way to approve the next action. The current OHC dashboard lacks a unified, proactive feed that aggregates tasks, messages, and AI proposals into actionable cards.

  ## Research Report
  *   **Current Capabilities:** The platform has foundational services for events, inbox, and specific AI agents, but they are disparate. There are early traces of an "Agent Feed" concept in UI and some backend tests, but the holistic architecture linking LLM intent resolution, event pipelining, and proactive mobile UX is incomplete.
  *   **Competitor Analysis:**
      *   *Shopify Sidekick / Wix:* These are reactive chatbots. The user has to know what to ask (e.g., "How are my sales?"). They do not proactively push operational tasks into a unified feed.
      *   *Linear / Notion:* Excellent feed/inbox mechanisms for knowledge workers, but not tailored for physical/local business operations (like accepting a deposit or replying to an Instagram DM).
  *   **Gap Identified:** A central "Agent Feed" that acts as the single source of truth for the owner's day. It must ingest events from all channels (webhooks, orders, messages), use LLMs to resolve intent and draft actions, and present them as simple "Action Cards" (Approve/Edit/Discard) on mobile.
  *   **Strategic Advantage:** Shifting from "reactive dashboard" to "proactive feed" is the core differentiator of the OHC promise. It completely abstracts software management, turning the app into an actual "assistant" that prepares the work for the owner's final approval.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  graph TD;
      subgraph Event Sources
          Stripe[Stripe Webhooks]
          IG[Instagram/WhatsApp DMs]
          Orders[New Orders / Bookings]
          Jobs[Scheduled Cron Jobs]
      end

      subgraph Ingestion & Processing
          EventBus[Event Bus / Message Queue]
          FeedWorker[Agent Feed Worker]
          LLM[LLM Intent & Draft Engine]
          DB[(PostgreSQL agent_feed)]
      end

      subgraph Presentation
          MobileApp[OHC Tauri Mobile App]
          ActionCards[Unified Agent Feed UI]
      end

      Stripe -->|Event| EventBus
      IG -->|Message| EventBus
      Orders -->|State Change| EventBus
      Jobs -->|Trigger| EventBus

      EventBus -->|Consume| FeedWorker
      FeedWorker -->|Classify & Draft| LLM
      LLM -->|Proposed Action| FeedWorker
      FeedWorker -->|Save Item| DB

      DB -->|Push/Pull| MobileApp
      MobileApp -->|Render 375px Cards| ActionCards
  ```

  ### Mobile UX Flow (375px First)
  1.  **Zero-State / Good Morning:** User opens the app. No complex charts. A simple greeting and a vertical feed of "Action Cards."
  2.  **Card 1 (Customer Interaction):** "Maya, someone asked about vegan cakes on Instagram." The card displays the AI-drafted reply: "Yes, we have vegan cakes! Here's a link to order." Buttons: `[Approve & Send]` | `[Edit]`.
  3.  **Card 2 (Operations):** "You have 3 orders to fulfill for tomorrow morning." Buttons: `[View List]` | `[Mark In Progress]`.
  4.  **Card 3 (Growth/Advisory):** "It's been a month since your last promo. Should I draft an email to your 50 past customers?" Buttons: `[Yes, draft it]` | `[Not right now]`.
  5.  **Action Execution:** When the user taps `[Approve]`, the app sends the mutation to the backend, the state updates to 'COMPLETED', and the card smoothly slides away.

  ### AI Agent Integration Points
  *   **All Agents feed into this system:** The Operations Agent pushes fulfillment tasks, the Customer Success Agent pushes message drafts, the Finance Agent pushes invoice reminders.
  *   **Intent Resolution Engine:** A central layer that intercepts raw events, queries the tenant's specific context (inventory, past chats), and drafts the JSON payload for the action card.

  ### Performance & Security Integrity
  *   **Mobile-First Rendering:** Cards must be lightweight JSON objects. The feed must load instantly (edge-cached or highly optimized).
  *   **Multi-Tenant Isolation:** Feed items are strictly isolated by `tenant_id` at the database query level.
  *   **Real-time Updates:** The feed should ideally use WebSockets or Server-Sent Events (SSE) to update instantly without the user needing to pull-to-refresh.

  ## Implementation Prompt
  Implement the Proactive Unified Agent Feed for Mobile Operations.
  The system must act as the central nervous system for the business owner. Build the backend ingestion pipeline that takes raw events (e.g., new order, new message), uses an LLM to determine the intent and draft a response/action, and stores it in an `agent_feed` table.
  Create the mobile-first (375px) UI that displays these as actionable cards (Approve, Edit, Discard). The UI must follow the macOS Translucent Glass aesthetic with clean, readable typography.
  Acceptance criteria:
  1. An event is triggered in the backend.
  2. The LLM successfully drafts an action.
  3. The action appears as a card in the mobile UI.
  4. The user can tap 'Approve' to execute the action and clear the card.
  Ensure strict tenant isolation and write Playwright E2E tests for the feed interaction. Do not use any mocked data in the UI; all feed items must originate from the database.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
