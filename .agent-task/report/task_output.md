issue_title: "Native Rust Omnichannel Chat System Architecture"
issue_description: |
  ## Native Rust Omnichannel Chat System Architecture

  **Problem Statement**
  OHC requires a robust, high-performance, and multi-tenant omnichannel chat and customer support system natively implemented in Rust, retiring the reliance on the external Chatwoot dependency. The system must seamlessly handle multi-channel communications (e.g., Email, SMS, Web Widget, WhatsApp) and integrate deeply into OHC's architecture, providing a unified Inbox for non-technical owner/operators (Maya, Carlos, Priya) to interact with their customers efficiently without being overwhelmed by technical details.

  **Research Report**
  An audit of the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) reveals a mature Rails-based architecture. Key components include:
  *   **Inboxes**: Centralized hubs connecting multiple channels (Web Widget, Email, SMS, etc.) to an Account (tenant).
  *   **Conversations**: Groupings of messages tied to a specific Contact and Inbox.
  *   **Messages**: Individual communication units supporting text, attachments, and rich media.
  *   **Channel Adapters**: Specific integrations (e.g., `Channel::Email`, `Channel::Sms`, `Channel::WebWidget`) that normalize incoming data into the standard Conversation/Message models.
  *   **Contacts**: Unified customer profiles aggregating conversations across different channels.

  OHC's Rust implementation must replicate this core functionality, adapting it to a strictly typed, high-concurrency, multi-tenant Rust environment. The system must prioritize Zero-Trust isolation (tenant boundaries) and support a mobile-first, reactive UX.

  **Design Doc (High-Level Architecture)**

  *   **Architecture Diagram (Mermaid.js)**
      ```mermaid
      erDiagram
          TENANT ||--o{ INBOX : owns
          TENANT ||--o{ CONTACT : manages
          INBOX ||--o{ CHANNEL : configures
          CHANNEL {
              string type "e.g., WebWidget, SMS, Email"
              json config
          }
          CONTACT ||--o{ CONVERSATION : participates
          INBOX ||--o{ CONVERSATION : contains
          CONVERSATION ||--o{ MESSAGE : contains
          MESSAGE {
              string content
              string sender_type "e.g., Agent, Contact, Bot"
          }
      ```
  *   **Mobile UX Flow (375px first)**
      *   The central "Work Triage" screen aggregates active conversations prioritized by urgency (SLA, customer sentiment).
      *   Tapping a conversation opens a familiar chat interface (similar to iMessage/WhatsApp), optimized for quick replies, canned responses, and AI agent assistance.
      *   "Advanced" settings (e.g., channel configuration, API keys) are hidden behind a clear "Settings" layer, avoiding clutter for daily operations.
  *   **AI Agent Integration Points**
      *   **Triage Agent**: Automatically categorizes incoming messages, assesses sentiment, and drafts initial responses for owner review.
      *   **Knowledge Agent**: Surfaces relevant FAQs or past conversation context based on message content.
      *   **Operations Agent**: Detects intent (e.g., booking request, order status) and provides inline actionable widgets within the chat interface.

  **Implementation Prompt**
  Implement the core Rust backend data models and gRPC services for the Native Omnichannel Chat System. This includes defining `Inbox`, `Conversation`, `Message`, and `Contact` entities with strict `tenant_id` isolation. Create a generic `ChannelAdapter` trait/interface to normalize incoming messages from diverse sources (starting with a simulated Web Widget channel). The implementation must integrate with OHC's existing Postgres schema and provide clear API boundaries for the Flutter frontend to consume. Ensure 100% unit test coverage for the new models and services.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
