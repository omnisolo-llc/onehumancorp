issue_title: "Native Rust Omnichannel Chat System Architecture"
issue_description: |
  **Title**: Architect and Implement Native Rust Omnichannel Chat System to Replace Chatwoot

  **Problem Statement**:
  Currently, OHC requires external Chatwoot services for omnichannel chat. This introduces external dependencies, potential SLA risks, and breaks the native, cohesive "OneHumanCorp" assistant experience for our users (like Maya, Carlos, and Priya). They expect a unified inbox where they can talk to customers on Instagram, WhatsApp, or website live chat natively without juggling third-party portals or noticing slow handoffs between OHC and Chatwoot.

  **Research Report**:
  - We audited the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) to understand its architecture.
  - Chatwoot's core models revolve around `Account` (tenant), `Inbox`, `Channel::*` (Email, Api, Facebook, Twitter, WhatsApp, etc.), `Conversation`, `Message`, and `Contact`.
  - It handles multi-tenancy at the account level (`account_id` on all entities).
  - WebSockets provide real-time updates to the UI, while background jobs handle outgoing webhooks, email processing, and notifications.
  - Relying on external Chatwoot breaks the multi-tenant SaaS model we want to control end-to-end, especially for integrating AI Agent capabilities directly into the chat stream (e.g., auto-drafting replies).
  - Competitors like Shopify Inbox, Wix Inbox, and GoDaddy Conversations offer fully integrated, native experiences.
  - To match Chatwoot's functionality natively in Rust, we need:
    - Multi-tenant data models for Contacts, Inboxes, Conversations, and Messages (similar to Chatwoot but mapped to OHC's `tenant_id`).
    - Channel Adapters mapped to OHC integrations (e.g. `src/server/integrations/*`).
    - WebSocket streaming for real-time frontend updates via NATS/Redis pubsub.
    - AI Agent integration points so the assistant can automatically reply or draft messages based on the business's context.

  **Design Doc**:

  *Architecture Diagram (Mermaid.js)*:
  ```mermaid
  graph TD
      A[Mobile/Web Client] -->|WebSocket/REST| B(Native Rust Chat API)
      B --> C[(Tenant PostgreSQL DB)]
      B --> D[Channel Event Subscribers]
      D --> E[WhatsApp/Instagram/Web Widget]
      B --> F[NATS Pub/Sub]
      F --> A
      F --> G[AI Assistant Job Queue]
      G --> H[LLM Integration]
  ```

  *UI Wireframes & Screen Flow (375px first)*:
  - **Unified Inbox List**: A clean, translucent card layout showing active conversations across all channels. Unread messages have a distinct, visually pleasing indicator.
  - **Conversation View**: Native chat interface with sticky header (contact name/avatar). Message bubbles flow from bottom. A dynamic text input area that includes a prominent "Draft with AI" button.
  - **Mobile UX Flow**: Tap Inbox -> View List -> Tap Conversation -> Read -> Tap AI Suggestion -> Send. Everything fits 375px without horizontal scroll.

  *AI Agent Integration Points*:
  - **Pre-Processing**: When a message arrives via a Channel Adapter, an AI classification job runs to categorize intent (e.g., "quote request", "complaint") and tag the conversation.
  - **Drafting**: The AI assistant automatically drafts a reply based on business context (inventory, calendar) and presents it as a pending state in the UI for the owner to approve and send.

  *Key Design Decisions*:
  1. **Built in Rust**: Ensures high performance and tight integration with OHC's existing backend (inside `onehumancorp/mono`).
  2. **Row-Level Security**: Every chat entity is keyed by `tenant_id` to guarantee data isolation.
  3. **Event-Driven Architecture**: Decouple message ingestion from processing using NATS and background jobs, matching the Chatwoot approach but built on OHC infrastructure.

  **Implementation Prompt**:
  As an implementer, build the native Rust omnichannel chat system to replace our external Chatwoot dependency. Your goal is to deliver a fully functional, real-time unified inbox for our owners.
  - **CUJ**: A customer sends a message via a mock channel; the business owner (e.g., Maya) sees it arrive instantly in the OHC mobile-friendly web UI, reviews an AI-drafted reply, and clicks send.
  - **Acceptance Criteria**:
    - Backend API and WebSocket infrastructure built in Rust.
    - Multi-tenant data models for Conversations and Messages.
    - Frontend UI follows the 375px mobile-first translucent design system.
    - AI agent can intercept incoming messages and draft responses.
    - 100% test coverage and E2E Playwright tests verifying the real-time chat flow.
    - NO external Chatwoot dependency.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
