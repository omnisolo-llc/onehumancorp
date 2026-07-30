issue_title: "Native Rust Omnichannel Chat System for OHC"
issue_description: |
  **Mission Queue Protocol Report**

  **Problem Statement**
  Small-business owners and operators (Maya the home baker, Carlos the field service owner, Fatima the food cart operator) rely heavily on conversational channels (WhatsApp, Instagram DMs, SMS, Web Chat) to serve customers and capture demand. Historically, OHC might have considered relying on a third-party open-source service like Chatwoot for conversational capabilities. However, integrating and managing an external, Ruby on Rails-based third-party service introduces significant complexity for a non-technical owner, fractures the user experience, creates latency in the AI agent feedback loop, and violates OHC's core value of "Radical Simplicity" and the explicit mandate to implement native Rust systems. Owners need a unified, native conversational assistant that operates seamlessly within the OHC platform, without the operational overhead of a disconnected third-party tool.

  **Research Report**
  As mandated, Chatwoot has been evaluated and is formally RETIRED as a third-party integration target. I cloned and benchmarked the Chatwoot open-source repository (`https://github.com/chatwoot/chatwoot`) to understand its architecture and feature set.
  *   **Chatwoot Capabilities Evaluated:**
      *   **Channel Adapters:** WhatsApp (via Twilio/Cloud API), Instagram, Facebook Page, Telegram, Line, SMS (Twilio/Bandwidth), Web Widget, Email, API. Checked source code like `app/models/channel/whatsapp.rb` and `app/services/whatsapp/send_on_whatsapp_service.rb`.
      *   **Core Primitives:** Accounts (Tenants), Inboxes (Channel configurations), Conversations, Messages, Contacts.
      *   **Workflow:** Routing, SLAs, Macros, Canned Responses, Agent assignments.
  *   **Strategic Misalignment:** Chatwoot is a heavy Rails monolith designed primarily for traditional human-led support teams. OHC requires an AI-first, owner-centric, highly concurrent, and low-latency system built natively in Rust. Relying on an external Chatwoot deployment would compromise OHC's multi-tenant SaaS architecture (Go/Bazel/PostgreSQL -> moving to Rust) and standalone deployment goals.
  *   **Conclusion:** OHC must build a native Rust multi-tenant omnichannel chat engine that achieves parity with Chatwoot's core conversational features but is fundamentally designed for AI agent orchestration and "assistant-first" interactions.

  **Design Doc**
  The native Rust Omnichannel Chat System will be a core OHC microservice (or set of crates) managing all external conversational I/O.
  *   **Unified Inbox Model:** Implement a `Tenant` -> `Inbox` -> `Conversation` -> `Message` data hierarchy in PostgreSQL (with Row-Level Security), heavily inspired by Chatwoot's domain model but optimized for Rust/SQLx.
  *   **Channel Connectors (Rust Traits):** Develop a modular connector architecture where each channel (WhatsApp, Web Chat, Instagram DM, SMS) implements a standard Rust trait for sending/receiving messages and managing connection health.
  *   **AI First Integration:** The system must natively integrate with OHC's AI Job Queue. Incoming messages from any channel immediately trigger the "Work Triage" agent. The "Customer & Relationship Assistant" agent can seamlessly draft replies directly into the conversation stream.
  *   **Real-time Event Bus:** Utilize Redis (or similar high-performance pub/sub) for real-time WebSocket updates to the OHC Flutter/PWA frontend when new messages arrive or agent drafts are ready.
  *   **Owner Experience:** The owner sees a single, unified "Work Feed" or "Inbox" in the OHC UI. They do not need to manage complex routing rules; the AI assistant groups, prioritizes, and proposes responses across all channels.

  **Implementation Prompt**
  Implement a native Rust multi-tenant omnichannel chat system within the OHC repository.
  1.  **Core Data Model:** Design and implement the database schema (PostgreSQL) and Rust structs for `Inboxes`, `Conversations`, `Messages`, and `Contacts`, ensuring strict tenant isolation (RLS).
  2.  **Initial Channel Connectors:** Implement the foundational channel connector interface and build at least two functional connectors: a generic Web Chat Widget API and a WhatsApp Cloud API connector.
  3.  **Real-time Delivery:** Implement the WebSocket or SSE infrastructure to deliver new messages and updates to the frontend in real-time.
  4.  **AI Orchestration Hook:** Create the necessary event triggers so that incoming messages on any inbox are automatically enqueued for processing by the OHC AI assistant (Work Triage/Customer Assistant).
  5.  **Verification:** The implementation MUST include 100% unit test coverage for the Rust components and comprehensive E2E Playwright tests simulating a customer sending a message (e.g., via Web Chat) and the owner viewing it in the OHC UI. No mock data; test against the live local stack.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
