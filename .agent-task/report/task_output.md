issue_title: "Architecture Feature: Core Agent Feed UI and Proactive Action Cards"
issue_description: |
  ## Problem Statement
  Small business owners (like Maya the baker and Carlos the handyman) are overwhelmed by fragmented systems and reactive dashboards. Existing platforms (Shopify, Wix) provide tools but require the owner to actively manage them, figure out what to do next, and manually execute tasks (like responding to DMs or updating inventory). There is a critical need for an "Invisible AI Automation" engine that does the heavy lifting and presents the owner with ready-to-approve actions, saving time and cognitive load.

  ## Research Report
  Based on the competitive analysis and user sentiment audits, platforms like Shopify rely on reactive chatbots (e.g., Sidekick) and a fragmented "App Tax" ecosystem. Wix provides basic setup but lacks operational depth. Users explicitly complain about "Setup Paralysis" and "DM Overload."
  Our deep dive into the OHC vision (see `agent_feed_deep_dive.md` and `[research]_ohc_smb_market_dynamics_agentic_workflows.md`) indicates that the core differentiator for OHC is shifting from a "Reactive Tool" to a "Proactive Agent." The Agent Feed is the central nervous system of this vision, acting as a unified inbox for business events and AI-drafted actions.

  ## Design Doc
  ### Architecture
  1.  **Event Ingestion Pipeline:** Webhooks (Stripe, Instagram Graph API) and internal state changes (Inventory, Orders) publish events to a central message bus (Redis Pub/Sub or Kafka).
  2.  **Intent & Context Resolution (LLM Layer):** An asynchronous worker consumes events, uses a Gemini-powered intent classifier, and queries the tenant's specific context (inventory, policies) via RAG.
  3.  **Action Generation:** The LLM drafts a proposed response or action (e.g., an Instagram reply confirming stock, or a restock order draft).
  4.  **Feed Persistence:** Action cards are stored in a PostgreSQL `agent_feed` table linked to the `tenant_id`.

  ### Mobile UX Flow (375px)
  - **The Feed:** The primary screen after login is a vertical, chronological feed of "Action Cards".
  - **Action Card UI:** Each card uses the OHC Premium Token library (translucent materials, clean hierarchy). It contains a brief summary of the event (e.g., "New DM from @customer") and the AI's drafted response.
  - **Interaction:** Cards have primary, high-contrast touch targets (≥ 44x44px) for "Approve & Send", and secondary buttons for "Edit" or "Discard".
  - **No Horizontal Scroll:** The entire flow is constrained to a 375px width.

  ### AI Agent Integration
  - **The Ambassador (CS Agent):** Handles incoming messages and drafts replies.
  - **The Operations Agent:** Monitors inventory and creates feed items for low stock.
  - **The Promoter (Marketing Agent):** Suggests social posts based on new inventory additions.

  ## Implementation Prompt
  **Feature Name:** OHC Agent Feed - Core UI and Action Cards
  **Target Persona:** Maya the Baker

  **Outcome:** Maya logs into her OHC mobile app and sees a feed of pending actions, such as drafted replies to Instagram DMs about vegan cake availability. She can tap "Approve" to instantly send the reply without typing a word.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1.  When Maya logs into the mobile app (375px viewport), she lands on the "Agent Feed" screen.
  2.  The feed displays a list of pending `Action Card` components.
  3.  Each card must display an event summary, a drafted LLM response, and an "Approve" button.
  4.  Clicking "Approve" transitions the card to a completed state and fires the corresponding backend action.
  5.  The UI must strictly adhere to the OHC Premium Token design system (translucent glass styling, correct spacing, no horizontal scrolling).
  6.  **Verification:** Implement Playwright E2E tests simulating Maya logging in, viewing a mocked incoming DM action card, and clicking "Approve".

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
