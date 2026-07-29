issue_title: "Design and Implement OHC's Native Rust OmniChat Support Engine (Chatwoot Replication)"
issue_description: |
  # Design and Implement OHC's Native Rust OmniChat Support Engine (Chatwoot Replication)

  ## Problem Statement
  For non-technical owner/operators like Maya (home baker), Carlos (handyman), and Fatima (food cart operator), communication is a chaotic, multi-front war. They receive buyer inquiries on Instagram DMs, WhatsApp, SMS, Web Chat, and Email simultaneously. Trying to manually check each app leads to missed orders, slow responses, and extreme operational fatigue.

  Because third-party services like Chatwoot are 100% retired in our architecture, OHC requires a premium, native, high-performance, multi-tenant Omnichannel Customer Support & Chat Engine (to be called **"OHC OmniChat Engine"**) written in Rust inside `onehumancorp/mono`. This engine must centralize all conversation channels, unify client profiles through identity resolution, and deeply link with OHC's RAG knowledge base and KAIROS AI agents to deliver 1-tap automated reply drafts, all while performing beautifully on a 375px mobile screen.

  ---

  ## Research Report
  Our research benchmarked the open-source Chatwoot codebase alongside leading SMB platforms (Shopify Inbox, Wix Inbox, Squarespace, GoDaddy):
  1. **Omnichannel Inbox & Normalization**: Chatwoot’s channel adapters normalize messages into a standard database schema. Shopify Inbox only natively supports web-chat and Facebook, relying on slow third-party plugins for others.
  2. **Contact Identity Resolution (Identity Graph)**: When a customer reaches out via Instagram DM (`@maya_bakes`) and later via WhatsApp, standard tools create separate, fragmented threads. Chatwoot links identities under a single Contact using matching emails/phone numbers. OHC can leapfrog this by integrating an AI-assisted contact resolution loop.
  3. **High-Performance Rust WebSockets**: Real-time communication requires low-latency event broadcasting. While Chatwoot uses Ruby on Rails action cable (which struggles with heavy concurrent connection spikes), OHC's native Rust implementation built on Axum WebSockets and NATS/Redis pub-sub will easily scale to millions of concurrent active owners and customer widgets with minimal memory footprint.
  4. **AI-First Integration**: Unlike Wix or GoDaddy which treat AI as a separate post-process text editor widget, OHC's native integration generates context-aware RAG-based email and message drafts in the background and presents them on the owner's dashboard as a pending action item.

  ---

  ## Design Doc

  ### High-Level Architecture & Flow
  ```mermaid
  sequenceDiagram
      actor Customer
      participant Meta as Meta API (IG/WA)
      participant OHC as OHC Axum Ingress Gateway
      participant Identity as Identity Resolution Mesh
      participant DB as Postgres (Multi-Tenant RLS)
      participant Queue as AI Job Queue
      participant Agent as CS/Ops AI Agent
      participant WS as Real-Time Event Broker (Redis PubSub)
      actor Owner as Maya/Carlos (Mobile UI)

      Customer->>Meta: Sends message ("Do you do vegan custom cakes?")
      Meta->>OHC: Inbound Webhook Payload (HTTPS POST)
      OHC->>Identity: Resolve Sender (ig_handle -> customer_id)
      Identity-->>OHC: Return resolved customer profile
      OHC->>DB: Persist Inbound Message (thread status = pending_review)
      OHC->>Queue: Enqueue AI Draft Job
      OHC->>WS: Broadcast event "message.created" (via WebSockets)
      WS-->>Owner: UI updates conversation feed with message

      Note over Queue, Agent: Background RAG Processing
      Queue->>Agent: Trigger CS Agent with thread context
      Agent->>Agent: Pull custom catalog & baker operating hours
      Agent->>DB: Save AI Draft Payload (ai_draft_state = draft_ready)
      Agent->>WS: Broadcast event "ai_draft.created"
      WS-->>Owner: UI animates typing and presents AI draft reply "Hi! Custom bakes start at..."

      Owner->>OHC: Taps "Approve as-is" or "Save & Send"
      OHC->>Meta: Outbound Channel Send (Meta Graph API)
      OHC->>DB: Update Thread & Message status (sent)
      Meta-->>Customer: Receives instant helpful response
  ```

  ### Data Model & multi-tenancy Isolation (Multi-Tenant Postgres)
  All database structures implement `tenant_id` Row-Level Security (RLS) to enforce total user isolation.
  - **`ohc_contacts`**: Represents a single real customer (name, email, primary_phone, tenant_id).
  - **`ohc_contact_inboxes`**: Links a contact to multiple social platforms (channel_type [whatsapp, instagram, email, web_chat], channel_identifier [e.g. +1234567, @maya_bakes]).
  - **`ohc_conversations`**: Aggregates messages for a channel (tenant_id, contact_id, status [open, resolved, pending_review], ai_draft_state [none, draft_ready, approved]).
  - **`ohc_messages`**: The individual chat elements (tenant_id, conversation_id, sender_type [customer, human_agent, ai_assistant], content, attachment_url, is_private [for internal notes]).

  ### Mobile UX Flow (375px First Viewport)
  OHC OmniChat is designed first for 375px mobile screens, matching the macOS Translucent Glass aesthetic:
  1. **Omnichannel Inbox View**: A list of active threads. Each list item has a vibrant glass card design, featuring a prominent source indicator badge (📸 for Instagram, 💬 for WhatsApp, ✉️ for Web-Chat) and a "✨ AI Draft Ready" status chip if the AI co-pilot has written a response.
  2. **Vibrant Conversation Sheet**: Interactive bubble layout. Pulling down the top sheet reveals the **Buyer Context Overlay** (displaying lifetime orders, delivery schedule, and notes).
  3. **1-Tap AI Action Console**: Anchored above the mobile keyboard. It displays the AI draft in a translucent container. The owner can tap:
     - **"Approve as-is"**: Dispatches the message immediately.
     - **"Review & Edit"**: Swaps the UI into a textarea mode with the draft pre-populated for adjustments.
     - **"Dismiss"**: Discards the AI draft.

  ---

  ## Implementation Prompt
  **Outcome:** Implement OHC's high-performance native Rust OmniChat support and real-time message delivery system in `onehumancorp/mono`.

  ### Critical User Journey (CUJ)
  - **Step 1**: An inbound webhook message from Instagram, WhatsApp, or Web-Chat is captured by OHC's Axum Inbound Webhook gateway, verified for signature security, and stored in PostgreSQL under the correct `tenant_id` RLS space.
  - **Step 2**: The identity processor performs identity resolution to match the sender with an existing customer profile or creates a new one.
  - **Step 3**: An asynchronous task is dispatched to the background job queue. The Customer Success AI Agent performs RAG against the workspace's knowledge files and publishes a proposed draft response.
  - **Step 4**: The real-time WebSocket connection pushes the message and the completed AI draft to the mobile client, updating the UI instantly with a typing indicator and visual draft chip.
  - **Step 5**: The owner logs in on a 375px screen, reviews the draft via the Translucent Keyboard Console, and taps "Approve as-is", sending the outbound response back to the customer's social channel natively.

  ### Acceptance Criteria
  - **Multi-Tenant Security**: Strict `tenant_id` verification on all endpoints. No database leakage between workspaces is possible.
  - **No Webhook Blocking**: Webooks must return `200 OK` within 100ms. All identity resolution and AI drafting must be processed asynchronously.
  - **Real-time Synchronization**: Frontend clients must stay synced in real-time using Axum WebSockets and NATS/Redis pub-sub broadcasting.
  - **Mobile-First Responsiveness**: All inbox screens must fit on a 375px wide viewport without any horizontal scrolling, with interactive buttons meeting the 44x44px mobile touch target standard.
  - **Verification Coverage**: Comprehensive Rust unit tests verifying message normalization and multi-tenancy isolation. Write Playwright E2E tests simulating the live message ingestion, AI draft arrival, and owner approval flow.

  ---

  ## Priority & Scope
  - **Priority**: P1 (High-priority platform capability)
  - **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
