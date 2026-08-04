issue_title: "Implement Custom Native Rust Omnichannel Chat (Chatwoot Replacement)"
issue_description: |
  **Mission Queue Protocol Brief**

  **Problem Statement:**
  OneHumanCorp (OHC) currently relies on evaluating third-party integrations to handle omnichannel chat. Based on the new engineering mandate, OHC must retire Chatwoot as an external service entirely and build a custom, multi-tenant omnichannel chat engine natively in Rust inside the `onehumancorp/mono` repository. This new native system needs to replicate the core functionalities of Chatwoot (inbox, conversations, unified messaging, and channel connectors for Email, Facebook, Instagram, LINE, SMS, Telegram, TikTok, Twilio SMS, Twitter, Web Widget, and WhatsApp) without the overhead and complexity of managing an external Ruby on Rails dependency. Owners/Operators like Maya (baker) and Carlos (field service) need a seamless, integrated communication hub directly within OHC to talk to customers on any channel natively.

  **Research Report:**
  I have conducted a source code evaluation of the Chatwoot project (https://github.com/chatwoot/chatwoot). Chatwoot is a robust Ruby on Rails application (with Vue.js frontend). Key components to replicate in Rust:
  -   **Core Models:** Accounts, Users, Inboxes, Conversations, Messages, Contacts, Channels.
  -   **Channels:** The channel architecture is highly modular (`app/models/channel/`). It includes native connectors for API, Email, Facebook Page, Instagram, LINE, SMS, Telegram, TikTok, Twilio SMS, Twitter Profile, Web Widget, and WhatsApp.
  -   **Features:** Agent routing, macros, canned responses, automation rules, CSAT surveys.
  -   **SaaS Viability:** A native Rust implementation running within our existing Kubernetes/Bazel/gRPC infrastructure will drastically reduce operational complexity, memory footprint, and latency compared to running a separate Rails stack. It perfectly aligns with the Cloud (multi-tenant) and Standalone (local, private) requirements.

  **Design Doc:**
  The native OHC Chat system will be built as a new Rust module within `src/server/` (e.g., `src/server/chat/` or integrated into `src/server/hub.rs` / `src/server/msgbus.rs`).
  1.  **Protobuf Definitions:** Create `src/proto/ohc/v1/chat.proto` outlining messages, conversations, and inboxes, integrated tightly with our existing `hub.proto` and `model.proto`.
  2.  **Database:** Expand our PostgreSQL schema to handle Chatwoot-equivalent data structures (messages, conversations, contacts, channels), isolated by `tenant_id` for multi-tenancy.
  3.  **Channel Adapters:** Implement Rust-native webhook handlers and API clients for the target channels (WhatsApp, Instagram, Web Widget, etc.), utilizing our existing Redis lock mechanism (`ohc:lock:{tenant_id}:...`) for concurrent message processing.
  4.  **UI Integration:** The Flutter front-end will connect to these new gRPC/REST endpoints, presenting the unified inbox inside the "Assistant-First Shell", effectively replacing any embedded Chatwoot widgets.

  **Implementation Prompt:**
  Build a native Rust omnichannel chat backend in OHC. Start by defining the core data structures (Conversation, Message, Contact) in protobuf and the database schema with tenant isolation. Implement the base `Inbox` logic to receive and store messages. Build the first channel connector: a simple Web Widget (API based) to prove the end-to-end flow. The Flutter UI must be able to display a conversation and send a message through this native Rust backend. This must completely replace the need for an external Chatwoot instance.

  **Priority:** P0 (Critical - Mandated Architecture Change)
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, p0, chatwoot-replacement]
assignees: []
