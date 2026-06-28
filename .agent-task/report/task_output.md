issue_title: "Architect the Unified Agentic Work Feed (Assistant-First Shell)"
issue_description: |
  ## Problem Statement
  Small business owners (like Maya the Baker or Carlos the Handyman) currently suffer from "app tax" and dashboard fatigue. They have to constantly context-switch between disparate tools (email, Instagram DMs, calendar bookings, Stripe dashboards, manual task lists) to understand what needs their attention today. This reactive, "Dashboard-First" approach (used by legacy tools like Shopify and Wix) forces the owner to be a system administrator instead of an operator. OHC needs to fulfill its core promise: "Open OHC and immediately know what needs attention today."

  ## Research Report & Gap Analysis
  Based on our market mapping and competitor audit of platforms like Shopify (Sidekick), Wix, Durable, and HubSpot:
  1. **The Gap**: Existing AI tools are mostly *advisory* chatbots that require the user to initiate a prompt, or simple *generators* (like Durable for initial setup). They do not proactively manage daily operations.
  2. **The Opportunity**: SMBs do not want to read complex charts; they want a prioritized list of actions to take. We must transition the core OHC UI from a traditional service-oriented dashboard into an "Assistant-First Work Feed."
  3. **The Solution**: An intelligent triage engine that ingests multi-channel events (messages, orders, bookings, alerts) and presents them as actionable cards on a mobile-first (375px) feed.

  ## Design Doc: Unified Agentic Work Feed

  ### Architecture Diagram (Mermaid)
  ```mermaid
  graph TD
      subgraph Ingestion Layer
          IG[Instagram DMs] --> Events[Event Bus / Webhooks]
          Orders[New Orders / POS] --> Events
          Alerts[Inventory/System Alerts] --> Events
      end

      subgraph AI Triage Engine
          Events --> Classify[Intent Classification]
          Classify --> RAG[Query Tenant Context]
          RAG --> Draft[Generate Action/Reply Draft]
      end

      subgraph Unified Feed
          Draft --> FeedTable[(FeedItems DB - Tenant Scoped)]
      end

      subgraph User Shell (Mobile-First)
          FeedTable --> App[Assistant-First Work Shell]
          App --> Action[Owner taps 'Approve' or 'Edit']
          Action --> Execute[Agent Execution Layer]
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **The Core Screen**: The default state after login is not a chart of revenue, but a "Work Feed".
  2. **Action Cards**: The feed consists of translucent, macOS-style glass material cards (using OHC Premium Tokens).
  3. **Examples**:
     - *Card 1*: "Maya, you have 3 new Instagram DMs asking about custom cakes. I've drafted replies based on our availability." [Review Drafts]
     - *Card 2*: "Carlos, 2 appointments today. Your first is in 30 mins at 123 Main St." [View Route]
     - *Card 3*: "Inventory Alert: Flour is running low." [Re-order via Vendor]

  ### Key Design Decisions
  - **Assistant-Led**: The AI is the primary interface. It pre-computes the next best action and asks for owner consent (Human-in-the-Loop).
  - **Tenant Isolation**: All `FeedItem` records must strictly enforce `tenant_id` Row Level Security (RLS) in PostgreSQL.
  - **Zero-Trust**: The feed engine must assume inputs are untrusted and sanitize before rendering.

  ## Implementation Prompt (For Engineering Swarm)
  **Feature Name:** Unified Agentic Work Feed Shell
  **Target Persona:** All Owners (e.g., Jun - Location Manager)

  **Outcome:** Create the foundational database schema, API layer, and UI shell for the "Agentic Work Feed". The feed should display prioritized action items rather than static dashboards.

  **Critical User Journey (CUJ):**
  1. The owner opens the OHC mobile or web app.
  2. The home screen immediately presents a prioritized list of `FeedItem` cards (e.g., "Review Drafted Email", "Confirm Booking").
  3. The owner clicks a primary action button on a card (e.g., "Approve").
  4. The system executes the action and dismisses the card from the active feed.

  **Acceptance Criteria:**
  - Create the multi-tenant `feed_items` database schema in PostgreSQL.
  - Expose a secure API to fetch and mutate feed items.
  - Build the mobile-first (375px) UI Shell utilizing OHC Premium glassmorphism design tokens.
  - Must include Playwright E2E tests validating that a seeded user can see a feed item, click an action, and have the feed update correctly. ZERO mock data in the UI.

  ## Estimated Scope
  Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
