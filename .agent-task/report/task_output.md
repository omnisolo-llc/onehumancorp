issue_title: "Implement The Ambassador Agent: End-to-End Social DM Intake to Agent Feed"
issue_description: |
  ## Problem Statement
  Solopreneurs like Maya (Home Baker) lose out on potential sales because they cannot actively monitor social media Direct Messages (e.g., Instagram, WhatsApp) while operating their physical business. Current market solutions either function as dumb auto-responders or require the business owner to configure complex logic trees (e.g., ManyChat), which alienates non-technical owners. The platform lacks an end-to-end autonomous agent that ingests inquiries, drafts highly contextual replies, and surfaces them for 1-tap approval in a unified mobile-first interface.

  ## Research Report
  - **Competitor Analysis:** General website builders (Shopify, Wix) offer integrations with third-party chat apps but push the complexity of setup onto the user (the "App Tax" fatigue). Specialized chatbots require deep configuration. OHC needs a zero-setup, "Invisible AI Automation" approach.
  - **Code Audit:**
    - The schema for `agent_feed_items` exists (migration `031_agent_feed.sql`).
    - The Unified Agent Feed UI component exists in both the Next.js prototype and Tauri desktop mockups (`src/ui/tauri/src/ui/dashboard.html`).
    - E2E tests for the intake proposal flow (`src/e2e/nora_intake_proposal.spec.ts`) validate the concept of an intent leading to an Agent Feed card.
    - However, the concrete backend implementation connecting an external social webhook (like Instagram) through an LLM intent classifier to the creation of an `agent_feed_items` record (The "Ambassador" flow) is missing.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant IG as Instagram Webhook API
      participant Ambassador as OHC Ambassador Agent (Rust)
      participant Memory as RAG Context (Inventory/Policies)
      participant DB as Postgres (agent_feed_items)
      participant Mobile as OHC Mobile App (375px)

      Customer->>IG: DM: "Do you have vegan cakes?"
      IG->>Ambassador: Webhook Event (Message)
      Ambassador->>Memory: Query Business Context
      Memory-->>Ambassador: "Vegan cakes: 5 in stock"
      Ambassador->>Ambassador: LLM classifies intent & drafts reply
      Ambassador->>DB: Insert into agent_feed_items (Proposed Action)
      DB-->>Mobile: Push Notification / Feed Sync
      Mobile->>Mobile: Owner views 375px Action Card
      Mobile->>Ambassador: Tap "Approve & Send"
      Ambassador->>IG: Send Reply
  ```

  ### Mobile UX Flow
  - **Viewport Target:** Strictly 375px.
  - The notification arrives. The owner opens the app directly to the **Unified Agent Feed**.
  - A prominent "Action Card" is presented containing:
    - Customer Name & Platform Icon (Instagram).
    - Original message snippet.
    - The AI-drafted reply.
  - **Interactions:** Three large (min 44x44px touch target) buttons: `Approve & Send` (Primary), `Edit` (Secondary), `Discard` (Tertiary).

  ### AI Agent Integration Points
  - **Ingestion:** Secure webhook endpoint that validates incoming payloads (e.g., signature verification for Instagram Graph API).
  - **Intent & RAG:** The agent runs a prompt against `OHC_LLM_PROVIDER` (Gemini Pro preferred) passing the message and retrieving local tenant-scoped memory to construct the draft.
  - **Database Coordination:** The draft is serialized and saved in the `agent_feed_items` table with `lifecycle_state` = `pending_approval`.

  ## Implementation Prompt
  **User-Facing Outcome:** The business owner receives a drafted, accurate reply to a customer's Instagram DM in their Agent Feed and can send it with a single tap.
  **CUJ:**
  1. A webhook payload simulating an Instagram DM is sent to the new intake endpoint.
  2. The system asynchronously processes the message, querying existing mock inventory/policy data, and uses the LLM to generate a draft.
  3. The owner opens the Unified Agent Feed (mobile UI) and sees the new Action Card.
  4. The owner taps "Approve", which simulates sending the reply back to the social channel and marks the feed item as resolved.
  **Acceptance Criteria:**
  - Build the webhook ingestion endpoint.
  - Integrate the LLM call to classify intent and draft the reply.
  - Ensure the drafted action is persisted to `agent_feed_items`.
  - Update the Unified Agent Feed frontend to display this specific type of Ambassador card cleanly at a 375px breakpoint.
  - Write full unit tests and a Playwright E2E test covering this CUJ without mocking internal calls.

  **Estimated Scope:** Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []