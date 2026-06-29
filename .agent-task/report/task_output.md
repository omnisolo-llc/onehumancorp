issue_title: "[research] Unified Mobile-First Agent Action Feed Architecture"
issue_priority: "P0"
issue_category: "research"
issue_type: "task"
issue_label: ["agent-report", "mobile-first", "agent-feed", "high-impact"]
assignees: []
issue_description: |
  # Research Report: Unified Mobile-First Agent Action Feed Architecture

  ## 1. Problem Statement
  Small Business (SMB) owners like Maya the Baker or Carlos the Handyman are overwhelmed by traditional e-commerce dashboards. They don't have the time or technical expertise to dig through menus to find out what needs their attention, nor do they want to perform complex setups. Current platforms either offer complex manual tools (Shopify) or basic chatbots that advise but do not execute. The missing piece is a proactive, mobile-first feed that aggregates actionable tasks and auto-generated drafts, allowing the owner to manage their business simply by clicking "Approve."

  ## 2. Market & Research Findings
  Our analysis across `ohc_smb_mobile_first_design_research.md`, `agent_feed_deep_dive.md`, and `agentic_autonomous_website_builders_smb_platform_gap_analysis.md` reveals a consistent theme:
  *   **The "Now What?" Syndrome**: Users abandon platforms after setup because they don't know what to do next to drive traffic or manage operations.
  *   **Mobile Management Gap**: Legacy platforms (Shopify, Wix) treat mobile apps as supplementary for viewing stats, but require a desktop for real management. The OHC target is 100% operation from a 375px mobile screen.
  *   **Advice vs Action**: SMBs want an AI that *executes* state changes (e.g., drafts replies, updates inventory), not just an AI that tells them *how* to do it.

  ## 3. Design Doc: Agent Action Feed Architecture
  The solution is the **Unified Agent Action Feed** – replacing the traditional complex dashboard with a chronological feed of Agent Proposals.

  ### 3.1 Architecture Overview
  *   **Event Ingestion Pipeline:** Webhooks (Stripe, Instagram) and internal system events (new orders, inventory alerts) are published to a central message bus (Redis Pub/Sub).
  *   **Agentic Resolution Layer (LLM):** When an event is ingested, the Intent Classifier categorizes it. The system queries tenant-specific data via RAG (inventory, policies, prior conversations). An LLM (Gemini Pro/MiniMax) then generates a proposed action or draft response.
  *   **Action Card Generation:** The proposed action is formatted into an `AgentActionCard` object and stored in PostgreSQL, associated with the tenant.
  *   **Mobile UI Delivery:** The Flutter/Web frontend polls or receives real-time updates of pending cards to display in the feed.

  ### 3.2 Mobile UX Flow (375px First)
  1.  User opens the OHC app and lands on the home feed.
  2.  Instead of static graphs, they see vertical "Cards":
      *   *Card 1 (Customer Success Agent):* "Drafted a reply to a DM from @customer asking about vegan cakes. [Approve & Send] [Edit]"
      *   *Card 2 (Marketing Agent):* "You haven't posted in 5 days. Drafted an Instagram post for your new Cupcake batch. [Approve & Schedule]"
  3.  All cards must have a touch target of at least 44x44px and use OHC Premium Tokens (Glassmorphism). No horizontal scrolling.

  ### 3.3 AI Agent Integration Points
  *   **Customer Success Agent (The Ambassador):** Listens to messaging webhooks, uses RAG on inventory/FAQs to draft replies.
  *   **Marketing Agent (The Promoter):** Listens to inventory updates or cron schedules to draft promotional content.
  *   **Operations Agent (The Manager):** Flags low stock or scheduling conflicts.

  ## 4. Implementation Prompt
  **Target:** Implementer Agent
  **Objective:** Build the core backend event-to-feed pipeline and the frontend React/Flutter UI for the Unified Agent Action Feed.

  **Backend Tasks:**
  *   Implement the `AgentActionCard` data model with multi-tenant isolation.
  *   Create the event ingestion listener that triggers an LLM draft request based on mock incoming events (e.g., `incoming_message`, `inventory_low`).

  **Frontend Tasks:**
  *   Create the `AgentActionFeed` UI component optimized for 375px.
  *   Implement the `AgentActionCard` UI with distinct visual styles based on the agent type (Marketing, Operations, etc.).
  *   Ensure the "Approve", "Edit", and "Dismiss" actions have proper loading states and minimum 44x44px touch targets.

  **Acceptance Criteria:**
  *   A user can view a feed of pending agent actions on a 375px screen without horizontal scrolling.
  *   Clicking "Approve" triggers a state update indicating the action was executed.
  *   E2E Playwright tests must verify the feed renders correctly and interactions work.

  ## 5. Scope & Priority
  *   **Priority:** P0 (Critical path for the OHC differentiator).
  *   **Scope:** Large (Requires backend pipeline and frontend UI coordination).
