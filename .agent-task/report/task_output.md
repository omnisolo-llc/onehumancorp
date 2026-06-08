issue_title: "[Architecture] Implement Agentic Feed & Action Cards for Unified Inbox"
issue_description: |
  ## Problem Statement
  Small business owners (like Maya the baker and Carlos the handyman) are overwhelmed by incoming requests across multiple channels (Instagram DMs, emails, Stripe payments). Traditional platforms force them to manually check dashboards, install third-party plugins (the "Shopify tax"), and configure complex automation rules. The OHC platform lacks a unified, push-based "Agent Feed" that centralizes intent-classified events, generates AI-drafted responses, and presents them as simple, mobile-first "Action Cards" (Approve/Edit/Discard).

  ## Research Report
  - **Competitor Analysis**: Shopify and Wix rely on app ecosystems (e.g., Klaviyo for cart recovery, Zendesk for support). Users must stitch these together. Tencent WorkBuddy and DingTalk offer more integrated automation but often feel like corporate IT tools.
  - **OHC Differentiation**: OHC's value proposition is "Zero-Setup" and "Invisible AI Automation." By natively integrating an LLM-driven event ingestion pipeline (as outlined in `docs/business/market_research/agent_feed_deep_dive.md`), OHC can eliminate the need for third-party automation apps.
  - **Persona Impact**:
    - **Maya**: Receives a DM about vegan cakes. Instead of opening Instagram, she gets an OHC Action Card with a pre-drafted reply checking her inventory.
    - **Carlos**: Gets a missed call. OHC automatically drafts an SMS quote based on his standard pricing.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Webhooks: Stripe, IG, Email] --> B[Event Bus / Queue]
      C[Internal State: Inventory, Orders] --> B
      B --> D[Event Ingestion Worker]
      D --> E[Intent & Context Resolution Layer]
      E <--> F[RAG: Tenant Memory & Policies]
      E <--> G[LLM Provider: Gemini/MiniMax]
      E --> H[Action Card Generator]
      H --> I[Mobile-First Agent Feed UI]
      I --> J{User Action}
      J -->|Approve| K[Execute Action via Connector]
      J -->|Edit| L[Update Draft & Execute]
      J -->|Discard| M[Log Feedback for LLM]
  ```

  ### Mobile UX Flow (375px First)
  1. **Home/Feed Screen**: A vertical list of clean, translucent Glassmorphism cards.
  2. **Card Anatomy**:
     - **Context Header**: E.g., "Instagram DM from @vegan_eats (2 mins ago)".
     - **Draft Content**: "Yes, we have 3 vegan chocolate cakes left today! Would you like to reserve one?"
     - **Actions**: Large, 44x44px minimum touch targets for `Approve` (Primary), `Edit` (Secondary), `Discard` (Ghost).
  3. **Interaction**: Tapping `Approve` immediately dispatches the action and dismisses the card with a satisfying success animation.

  ### AI Agent Integration Points
  - **Work Triage Agent**: Subscribes to the Event Bus, classifies intent using `OHC_LLM_PROVIDER`, and determines priority.
  - **Customer Success Agent**: Drafts the specific replies using RAG context (past orders, tone of voice).
  - **Operations Agent**: Verifies inventory or scheduling availability before the draft is generated.

  ### Key Design Decisions
  - **Push vs. Pull**: The feed is push-based (like a social feed) rather than a pull-based dashboard.
  - **Atomic Actions**: Every card must represent a single, executable action to minimize cognitive load.
  - **Tenant Isolation**: Event context and RAG queries must strictly enforce `tenant_id` boundaries.

  ## Implementation Prompt
  **Goal**: Implement the end-to-end flow for the unified Agent Feed, starting with the backend event ingestion and ending with the mobile-first UI rendering Action Cards.

  **Critical User Journey (CUJ)**:
  1. As an owner, I open the OHC app to the `/assistant` (or `/feed`) route.
  2. I see a list of prioritized Action Cards representing recent customer inquiries or system alerts.
  3. I review an AI-drafted reply to a customer inquiry.
  4. I tap "Approve".
  5. The system successfully executes the action and updates the card's state.

  **Acceptance Criteria**:
  - Implement a basic event ingestion endpoint/worker to simulate incoming requests.
  - Integrate the LLM service to generate a draft response and create an Action Card record.
  - Build the mobile-first (375px) UI using the design system's translucent glass materials.
  - Ensure the "Approve" action works end-to-end and updates backend state.
  - Provide 100% unit test coverage for new backend logic and at least 3 Playwright E2E tests for the CUJ.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
