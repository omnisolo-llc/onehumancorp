issue_title: "Build Native Rust Omnichannel Customer Support & Chat Engine"
issue_description: |
  **Problem Statement**:
  OHC currently relies on external third-party tools like Chat-woot for customer support and chat functionality. This creates a fragmented experience for our users, who have to manage customer interactions in a separate tool, switch contexts constantly, and pay for additional subscriptions. For personas like Maya (Home Baker) who sells through Instagram DMs, or Carlos (Field Service Owner) who manages requests via SMS, a unified, native chat experience within OHC is critical to keep their daily work ordered and visible.

  **Research Report**:
  We have evaluated the source code of the legacy CW platform (https://github.com/chat-woot/chat-woot), a popular open-source omnichannel customer engagement suite. It provides a robust set of features for managing customer conversations across various channels (WhatsApp, Web Widget, Instagram, Email, SMS, Telegram, Line, Twitter, Facebook).
  Key capabilities include:
  *   **Omnichannel Inbox**: Aggregating messages from different platforms into a single view.
  *   **Channel Connectors**: Integrating with APIs of various platforms (e.g., Twilio for SMS/WhatsApp, Facebook Graph API for Instagram/Messenger).
  *   **Conversation Management**: Assigning conversations to agents, tracking status (open, resolved, snoozed), applying labels, and managing SLA policies.
  *   **Automation & Macros**: Rules and canned responses to streamline support.
  *   **Web Widget**: A customizable chat widget for websites.

  However, maintaining an external dependency on such a SaaS product conflicts with OHC's core value of "Radical Simplicity" and the promise of "Ask one assistant; it coordinates messages...". By building a native omnichannel chat engine in Rust directly within `onehumancorp/mono`, we can achieve seamless integration with OHC's AI Work Assistant (Work Triage, Customer Assistant), unify the UI, and provide a single, cohesive experience for our owner/operator personas.

  **Design Doc**:
  The native Rust omnichannel chat engine will be integrated directly into OHC's backend and frontend.

  1.  **Backend (Rust)**:
      *   Implement a new microservice or module within the existing Rust backend for managing channels, conversations, messages, and contacts.
      *   Data Model: Design PostgreSQL tables with row-level security (tenant isolation) for `channels`, `conversations`, `messages`, `contacts`, `canned_responses`, etc., heavily inspired by the legacy schema but adapted for OHC's architecture.
      *   Channel Adapters: Build native Rust connectors for priority channels:
          *   **Web Chat Widget**: A native endpoint for real-time WebSocket communication with the OHC frontend.
          *   **WhatsApp / SMS**: Integration (e.g., via Twilio or a similar provider API).
          *   **Instagram / Facebook Messenger**: Integration via Meta Graph API.
          *   **Email**: Parsing incoming emails and sending replies.
      *   AI Integration: Expose APIs for OHC's AI assistant to analyze incoming messages (Work Triage), draft replies (Customer Assistant), and trigger actions (e.g., creating tasks or bookings based on chat content).

  2.  **Frontend (Flutter + PWA)**:
      *   **Unified Inbox View**: Create a new "Inbox" or "Messages" section within the OHC Assistant-First Shell. This view will aggregate conversations from all connected channels.
      *   **Conversation Details**: A rich chat interface supporting text, attachments, quick replies, and inline AI assistance (e.g., "Draft a reply", "Suggest next action").
      *   **Settings/Configuration**: Simple, intuitive screens for non-technical users to connect their Instagram, WhatsApp, or set up the Web Widget, hiding technical jargon like webhooks or API keys behind simple OAuth flows where possible.

  **Implementation Prompt**:
  *   **User Outcome**: The owner (e.g., Maya, Carlos) can open OHC and immediately see all customer messages from Instagram, WhatsApp, and their website in one prioritized feed. They can reply directly from OHC, have the AI assistant draft responses, and turn conversation context into bookings or tasks without leaving the app.
  *   **Acceptance Criteria**:
      *   A new "Omnichannel Chat" module is implemented in the Rust backend, supporting tenant isolation.
      *   At least two initial channel connectors are built (e.g., Web Chat Widget and one social channel like Instagram or WhatsApp).
      *   The Flutter frontend includes a unified Inbox view that displays messages from all connected channels in real-time.
      *   Users can send and receive text messages through the unified interface.
      *   The UI must be fully functional and responsive on mobile (375px) and desktop.
      *   100% unit test coverage for new backend and frontend code.
      *   Comprehensive E2E Playwright tests verifying the end-to-end flow of receiving a message, viewing it in the UI, and sending a reply.
      *   ZERO mock data in the UI; all data must flow through the real backend.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
