issue_title: "Native Omnichannel Chat & Messaging Engine (Rust)"
issue_description: |
  # Problem Statement
  Owners and operators like Maya (Home Baker) and Carlos (Field Service Owner) receive customer inquiries across multiple platforms (WhatsApp, Instagram DMs, Email, SMS, Web Chat). Managing these scattered communications leads to missed opportunities, delayed responses, and lost context. OHC currently lacks a unified, multi-tenant omnichannel communication engine natively built into the platform, forcing reliance on disconnected tools or manual triage. OHC must own this capability natively so AI assistants can triage, draft replies, and link messages to tasks and revenue seamlessly.

  # Research Report
  Chatwoot was evaluated as an initial third-party integration target, but per the OHC architecture mandate, external Chatwoot integration is retired. Instead, a comprehensive source code audit of the `chatwoot/chatwoot` repository was conducted to benchmark capabilities for a native Rust implementation.

  **Key Findings from Chatwoot Audit:**
  - **Channel Diversity**: Chatwoot supports API, Email, Facebook Page, Instagram, LINE, SMS, Telegram, TikTok, Twilio SMS, Twitter Profile, Web Widget, and WhatsApp.
  - **Data Modeling**: Centralized `conversations` map to `inboxes`, `contacts`, `assignees`, and `teams`.
  - **Operations Features**: It includes SLA policies, macros, canned responses, automated routing (agent bots), and detailed conversation statuses (open, snoozed, resolved).
  - **Architecture Fit**: Chatwoot is built on Rails. OHC will replicate the best of these models (inboxes, conversations, messages, channels) but implement them as high-performance, row-level secured microservices in Rust/Go/Bazel within the OHC monorepo, keeping AI assistants deeply integrated into the message event stream.

  # Design Doc
  **Integration Point:**
  Build a native Omnichannel Messaging Engine within OHC (`onehumancorp/mono`) that unifies all incoming communication into a single "Work Triage" feed.

  **Core Components:**
  1. **Channel Adapters**: Native Rust modules to handle Webhooks/APIs for WhatsApp, Instagram, Email, and a custom Web Widget.
  2. **Unified Inbox Data Store**: PostgreSQL tables with `tenant_id` row-level security for `inboxes`, `conversations`, `messages`, and `contacts`.
  3. **Event Bus (Pub/Sub)**: Real-time event streaming for incoming messages so OHC's AI agents can instantly draft replies and suggest context-aware actions (e.g., creating a booking or quote).
  4. **Web Widget**: A lightweight, mobile-first Flutter web component that owners can embed on their sites, fully matching the OHC premium token design system.

  # Implementation Prompt
  Implement the foundation of the native OHC Omnichannel Messaging Engine. Begin by creating the database schemas and the gRPC API service for the Unified Inbox.

  **User-Facing Outcome:**
  An owner can navigate to their OHC dashboard, see a new "Unified Inbox" section, and view a simulated incoming message from a Web Widget and an Email channel. The AI assistant should be able to see this message in the triage feed.

  **Acceptance Criteria:**
  - Data models for Inbox, Conversation, Contact, and Message are implemented with tenant row-level security.
  - Basic CRUD APIs via gRPC/REST are available.
  - A simple Flutter UI view exists to display the unified conversation list.
  - E2E Playwright test validates that a message can be created via API and appears in the owner's UI feed.
  - Zero reliance on external Chatwoot services.

  # Priority
  P0

  # Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
