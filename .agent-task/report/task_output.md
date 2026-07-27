issue_title: "[Scout] Native Rust Omnichannel Chat System Parity with Chatwoot"
issue_description: |
  ## Mission Overview
  The objective is to retire third-party Chatwoot dependencies and establish a native Rust Omnichannel Chat Engine within OneHumanCorp's `mono` repository. Based on an analysis of the Chatwoot open-source project, OHC requires a scalable, multi-tenant conversational architecture natively in Rust that interfaces with WhatsApp, Web Widgets, SMS, and other channels.

  ## Problem Statement
  For non-technical owner/operators (like Maya the home baker or Nora the agency principal), managing customer conversations across Instagram DMs, WhatsApp, and their website is chaotic. They currently rely on separate external tools like Chatwoot, which fragments the owner experience, breaks contextual AI workflows, and introduces third-party dependency risks. They need a unified inbox integrated directly into OHC where all customer communications can be managed efficiently, with the AI assistant drafting replies and triggering business actions seamlessly.

  ## Research Report
  - **Tool Evaluated:** Chatwoot (Open Source Omnichannel Customer Support).
  - **Key Capabilities Analyzed:**
    - Channel Integrations: Web Widget, WhatsApp, Instagram, Facebook Page, Telegram, Line, SMS (Twilio).
    - Features: Pre-chat forms, HMAC authentication, feature toggles (attachments, emoji picker, mobile webview).
    - Architecture: Database-backed channels (`channel_web_widgets`, etc.) linked to unified inboxes and contacts.
  - **OHC Native Parity Plan:** Instead of integrating with a Chatwoot SaaS or self-hosted instance, OHC will build an equivalent multi-tenant chat system natively in Rust, integrated with the existing Go/PostgreSQL/Flutter stack.

  ## Design Doc
  - **Database:** Replicate the core concepts of `channels`, `inboxes`, `contacts`, `conversations`, and `messages`. Use PostgreSQL with row-level security (`tenant_id`).
  - **Backend (Rust):** Implement microservices/crates for channel adapters (e.g., WhatsApp Webhooks, Twilio SMS Webhooks, Native Web Widget API). Use gRPC for internal communication and REST for external webhooks/widget APIs.
  - **Frontend (Flutter):** Build a unified inbox interface. The assistant will triage messages, draft replies, and link conversations to customer profiles, orders, or bookings.
  - **Web Widget:** Develop a native JS SDK/snippet for owners to embed a chat widget on their websites, backed by the Rust API.

  ## Implementation Prompt
  Implement the foundational data models and a basic Web Widget channel adapter natively in Rust.
  - Create the PostgreSQL schema for the Web Widget channel, ensuring `tenant_id` isolation.
  - Implement the Rust API endpoints to handle Web Widget initialization, message sending, and receiving.
  - Develop a basic embeddable JS snippet that connects to these new Rust APIs.
  - Integrate the new chat events into the OHC AI Job Queue so the assistant can triage and draft replies for incoming widget messages.
  - Ensure the solution is multi-tenant and scalable. Do not prescribe specific function signatures; focus on the end-to-end flow from widget message to OHC inbox.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
