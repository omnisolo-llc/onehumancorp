issue_title: "Native Rust Omnichannel Customer Support & Communication Engine"
issue_description: |
  Title: Native Rust Omnichannel Customer Support & Communication Engine

  Problem Statement:
  Owners like Maya (baker) and Carlos (handyman) receive inquiries across Instagram DMs, WhatsApp, SMS, and website chat. Toggling between multiple apps leads to missed messages, lost context, and delayed responses. They need a single, unified inbox where an AI assistant can draft replies, maintain customer context, and seamlessly hand off to human operators, all within an offline-tolerant, mobile-first interface.

  Research Report:
  - Market Context: Competitors like HubSpot, Shopify Inbox, and WeCom offer unified inboxes, but often cater to enterprise or require heavy setup. OHC needs a zero-setup, zero-jargon experience.
  - Performance Benchmarks: Modern realtime systems (e.g., Discord, WhatsApp) utilize high-concurrency WebSocket/SSE connections. Rust is uniquely positioned for this due to its low memory footprint and predictable latency.
  - User Feedback: Operators consistently cite "managing messages across apps" as a top time-waster. An AI-first approach (where the assistant triages and drafts) fundamentally shifts the workflow from "typing replies" to "approving drafts."
  - Architectural Findings: Multi-tenant isolation at the communication layer is critical. Connections must be tenant-scoped, and data access strictly bound by row-level security.

  Design Doc:

  Mobile UX Flow (375px first):
  1. Unified Feed: The operator opens the app to a single unified "Inbox" tab.
  2. Message List: Each thread displays the customer name, the channel icon (Instagram, SMS, Web), and a preview. Unread/urgent messages are pinned.
  3. Thread View: Inside a thread, the operator sees the full history. The AI assistant pre-drafts a context-aware reply in a translucent suggestion bubble above the composer.
  4. Action: The operator taps "Send" to approve the AI draft or edits it using the native mobile keyboard.
  5. Offline Support: If offline, messages are queued locally and sent when connectivity is restored, with clear "pending" indicators.

  AI Agent Integration Points:
  - Customer Assistant: Listens to the incoming message stream, enriches context from past interactions, and generates draft replies.
  - Work Triage: Analyzes urgency and intent, grouping inquiries that require immediate action.

  Architecture Diagram:
  erDiagram
      Tenant ||--o{ Conversation : owns
      Tenant ||--o{ ChannelAdapter : configures
      Conversation ||--o{ Message : contains
      Conversation ||--o{ Participant : includes
      Participant }|--|| Contact : references
      ChannelAdapter ||--o{ Conversation : routes
      Message {
          uuid id
          uuid conversation_id
          uuid sender_id
          text content
          string status
          timestamp created_at
      }
      Conversation {
          uuid id
          uuid tenant_id
          string channel_type
          string status
      }

  Implementation Prompt:
  Implement a native Rust omnichannel communication engine for OHC. The system must support WebSocket/SSE for real-time messaging, handle incoming webhooks from external channels (e.g., Instagram, SMS), and route messages to unified, tenant-isolated conversations. The engine should provide hooks for the Customer Assistant AI to stream draft replies. Ensure the API is fully compatible with the 375px-first mobile frontend, including offline message queuing and status syncing. Strictly enforce multi-tenant isolation across all data models and connections.

  Priority: P0
  Estimated Scope: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
