issue_title: "Architect Autonomous Multi-Channel Order Triage and Unified Feed"
issue_description: |
  # Mission Queue Protocol: Autonomous Multi-Channel Order Triage and Unified Feed

  ## Problem Statement
  For non-technical owners like Maya (the baker) and Carlos (the handyman), managing incoming requests across Instagram DMs, WhatsApp, SMS, and their OHC storefront is overwhelming. They currently have to manually check multiple platforms, which leads to missed opportunities, delayed responses, and lost revenue. They need a single, unified "Work Triage" feed on their mobile device where an AI agent automatically consolidates messages, identifies intent (e.g., "Is this a quote request or a casual question?"), groups them into actionable tasks, and drafts responses.

  **Observed Product Gap:**
  During live product testing on the mobile UI (375px), the current Work Triage view only lists notifications without context. The owner has to tap into each notification to understand the context, and there's no way to automatically turn a customer inquiry into a "Quote Request" or "Task". The lack of cross-channel aggregation and AI-driven intent classification forces the owner into an administrative role.

  ## Research Report
  - **Competitor Insights**:
    - **Tencent Workbuddy / WeCom**: Excellent at unifying customer communications with internal tasks, but heavy on enterprise CRM features.
    - **Shopify Inbox**: Good for basic chat, but lacks the ability to autonomously turn a conversation into a booked service or custom quote without manual data entry.
    - **HubSpot**: Powerful unified inbox, but too complex (requires setting up pipelines and deals) for a solo operator like Carlos.
  - **Gap Analysis**: Existing solutions act purely as message aggregators. OHC's unique opportunity is to combine message aggregation with **Agentic Action**. When an Instagram DM arrives saying "How much to fix a leaky sink?", the AI shouldn't just show the message; it should draft a reply, check Carlos's schedule, and prepare a Quick Quote card for Carlos to approve with one tap.

  ## Design Doc
  - **Architecture Diagram (Mental Model)**:
    - **Ingestion Layer**: Webhook endpoints in `src/server/integrations/` to receive messages from Meta Graph API (IG/WhatsApp), Twilio (SMS), and OHC Web Chat.
    - **Triage Pipeline (`src/server/ohc/triage.rs`)**: A new pipeline that processes incoming messages. It uses the `ohc:lock` Redis pattern to prevent race conditions when multiple messages arrive for the same customer.
    - **Intent Classification Agent**: Calls the LLM to classify the message intent (`Inquiry`, `Order_Update`, `Support`, `Spam`) and extract entities (Dates, Services requested).
    - **Unified Feed Data Model**: A `work_feed_items` Postgres table with RLS enabled (`tenant_id`), storing the consolidated view of tasks, messages, and required actions.
  - **Mobile UX Flow (375px)**:
    - **Home Screen**: A clean feed of cards. The top card says "New Request: Leaky Sink (Instagram)".
    - **Interaction**: Carlos taps the card. The UI shows the customer's message, the AI's intent classification, and a pre-drafted reply with an attached Quote Estimate.
    - **Action**: Carlos taps "Send & Propose Quote". The system dispatches the message via the original channel and moves the item from "Triage" to "Pending Customer".
  - **AI Agent Integration Points**:
    - **Customer Assistant Agent**: Triggered by the Triage Pipeline to draft replies.
    - **Operations Assistant Agent**: Triggered to check schedule availability and propose times.
  - **Key Design Decisions**:
    - **Event-Driven**: The ingestion layer must push events to the AI Job Queue (using Postgres `SKIP LOCKED`) to ensure background processing without blocking the webhook acknowledgment.
    - **Zero Mock Data**: The UI must render feed items directly from `work_feed_items`.

  ## Implementation Prompt
  **Role**: Implementer Agent
  **Task**: Build the Autonomous Multi-Channel Order Triage and Unified Feed backend pipeline and mobile UI.
  **CUJ**:
  1. An external webhook simulates an incoming Instagram DM to Carlos about a leaky sink.
  2. The Triage Pipeline processes the message, classifies it as a Service Request, and generates a `work_feed_item`.
  3. The Customer Assistant drafts a reply with a proposed quote.
  4. Carlos opens the OHC mobile app, sees the prioritized feed item, and approves the draft with one tap.

  **Acceptance Criteria**:
  - Create the `work_feed_items` table with strict multi-tenant RLS.
  - Implement `src/server/ohc/triage.rs` for processing incoming channel messages.
  - Integrate the Intent Classification Agent to structure the incoming text.
  - Build the 375px-optimized Unified Feed UI in Flutter/Tauri.
  - Write Playwright E2E tests verifying the feed updates when a new message arrives and the owner can approve the drafted response.
  - 100% unit test coverage for the new triage pipeline.
  - ZERO mock data in the UI; all feed items must be real backend records.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
