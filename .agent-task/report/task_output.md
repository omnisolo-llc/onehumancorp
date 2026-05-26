issue_title: "Omni-Channel AI Conversation & Quoting Gateway"
issue_description: |
  # Omni-Channel AI Conversation & Quoting Gateway

  ## Problem Statement
  Small business owners like Maya (baker), Carlos (handyman), and Fatima (food cart) receive customer inquiries across fragmented channels (Instagram DMs, WhatsApp, SMS, web chat). Managing these messages manually causes delayed responses, lost leads, and context switching overhead. They need an invisible AI agent that instantly replies to inquiries across all channels, handles FAQ (e.g., "do you do vegan cakes?"), generates quotes with deposit links, and routes high-intent orders to a unified mobile inbox.

  ## Research Report
  - **Market Context**: Shopify relies on third-party apps (Gorgias, Chatdesk) for unified inbox and AI replies, adding $50-100/mo. Wix offers basic web chat but lacks native Instagram/WhatsApp agentic capabilities.
  - **User Pain Points**: Carlos misses word-of-mouth leads because he's on a ladder. Maya sleeps and misses Instagram DMs from different time zones. Fatima struggles with English text pre-orders via WhatsApp.
  - **Proposed Solution**: A centralized Omni-Channel Conversation Gateway that ingests webhooks from Meta (IG/WhatsApp), Twilio (SMS), and native web chat. Messages are normalized and routed to the Operations/CS AI Department to generate context-aware replies, create drafts, or trigger quote/invoice generation.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      WebhookGateway ||--o{ ConversationStream : "ingests"
      ConversationStream ||--o{ Message : "contains"
      Message }o--|| AI_CS_Department : "evaluated by"
      AI_CS_Department ||--o{ ActionIntent : "generates"
      ActionIntent ||--|| QuoteDraft : "triggers"
      ConversationStream }o--|| UnifiedInbox : "syncs to"

      WebhookGateway {
          string channel_type
          string external_id
      }
      ConversationStream {
          string tenant_id
          string customer_id
          string status
      }
      Message {
          string content
          boolean from_customer
      }
  ```

  ### AI Agent Integration Points
  - **CS Agent (Listening)**: Triggers on new `Message`. Uses semantic search on tenant catalog/FAQ.
  - **Operations Agent (Action)**: Triggers when CS Agent detects booking/quote intent. Invokes pricing engine and creates a checkout link.
  - **Language Agent**: Auto-translates incoming/outgoing messages (crucial for Fatima).

  ### Mobile UX Flow (375px first)
  1. **Unified Inbox Screen**: Bottom nav item with notification badge. Shows a unified list of conversations with channel icons (IG, WhatsApp).
  2. **Conversation View**: Chat bubbles. AI auto-replies are slightly translucent or have an AI sparkle icon to indicate they were sent automatically.
  3. **Quote Generation Card**: Inside the chat, Carlos taps the "+" button, selects "Send Quote", inputs $200 for "Pipe Repair", and taps send. The AI formats it and sends a deposit link via the native channel.

  ### Key Design Decisions
  - **Zero Trust / Isolation**: Tenant IDs are strictly enforced at the Webhook Gateway. Channel credentials (Meta API keys) are stored in the hybrid secrets vault.
  - **Grandmother Test**: No configuration for the AI. Users just connect their IG account, and the AI immediately knows their catalog and business hours.
  - **Visual Excellence**: Unified Inbox and Conversation View must use macOS-style Translucent Glass materials combined with clean Ubiquiti UniFi modular dashboard card layouts.

  ## Implementation Prompt
  Implement the ConversationStream and Message data entities in the PostgreSQL schema with strict tenant isolation. Create the WebhookGateway API endpoints to ingest Meta and Twilio payloads. Wire the normalized messages to a new AI task queue for the CS Department to generate automated replies. Create the "Unified Inbox" mobile-first view (React/Tailwind using macOS glassmorphism) that aggregates these streams. Ensure a 375px viewport displays the chat perfectly and allows sending a manual override message or a deposit link. Acceptance Criteria: End-to-end flow from receiving a simulated webhook to seeing an AI-generated reply draft in the mobile UI.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []