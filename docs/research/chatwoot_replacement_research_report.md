
> Superseded architecture: Chatwoot was removed in favor of the native omnichannel design in `docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md`. The material below is retained as historical research only.

# Native Rust Omnichannel Chat Infrastructure (Chatwoot Replacement)

## Problem Statement
OHC currently relies on external Chatwoot integrations for omnichannel customer support, which adds latency, complexity, and operational overhead. We need to retire the third-party Chatwoot dependency and implement a high-performance, multi-tenant omnichannel chat engine natively in Rust within `onehumancorp/mono`. This will provide small-business owners (like Maya handling Instagram DMs or Carlos handling WhatsApp inquiries) with a seamless, unified inbox experience directly within OHC.

## Research Report
- **Tool Audited:** Chatwoot (source code benchmarking at `https://github.com/chatwoot/chatwoot`)
- **Findings:**
  - Chatwoot handles various channels (WhatsApp, Web Chat, Instagram, Email, SMS, Telegram, Line) via distinct channel models and webhook adapters.
  - Key data entities include `Account`, `Inbox`, `Conversation`, `Message`, `Contact`, `AgentBot`.
  - It uses PostgreSQL for storage and Redis for PubSub/Sidekiq queues.
  - The API is extensive, providing REST endpoints for widget interactions, webhook ingestions, and agent dashboard operations.
- **SaaS Viability for OHC:** Implementing this natively in Rust allows us to leverage our existing multi-tenant PostgreSQL (with row-level security) and Redis architecture. It ensures tight integration with our AI agents (e.g., Customer Assistant drafting replies) without crossing network boundaries to third-party services.

## Design Doc
- **Core Entities:** `Tenant` (Account), `Inbox` (Channel configuration), `Conversation`, `Message`, `Contact`.
- **Channel Connectors:** Modular Rust crates/services for Web Widget (WebSocket/REST), WhatsApp (webhook parsing), Instagram/Facebook (Meta Graph API webhooks), and Email (IMAP/SMTP or SendGrid/Postmark webhooks).
- **Integration with OHC:**
  - Webhooks from external providers will be ingested by a new Rust API service.
  - Messages are parsed, standardized, and stored in PostgreSQL under the respective `tenant_id`.
  - Real-time events are pushed to the OHC Frontend (Flutter) via WebSocket/SSE.
  - OHC AI agents can subscribe to new message events to automatically draft replies or triage work.
- **User Experience:** Owners will see all messages from different channels in a single, unified "Work Triage" feed in the OHC app.

## Implementation Prompt
- Create the core database schema (PostgreSQL) for `inboxes`, `conversations`, `messages`, and `contacts` with multi-tenant row-level security.
- Implement a native Rust API service that provides endpoints for:
  - Web widget initialization and message sending/receiving.
  - Webhook ingestion from at least one major provider (e.g., WhatsApp or Instagram) as a proof-of-concept.
- Build the corresponding Flutter UI components for a unified inbox view that updates in real-time.
- Ensure the solution can operate locally (Standalone) and in the Cloud (Multi-tenant).
- Acceptance Criteria: A user can send a message via a mock WhatsApp webhook, it appears in the OHC unified inbox, and the owner can reply from the OHC app, which triggers an outgoing API call.

## Priority
P0

## Estimated Scope
Large
