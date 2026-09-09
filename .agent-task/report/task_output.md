issue_title: "Native Rust Omnichannel Communication Architecture"
issue_description: |
  ## Problem Statement
  Small business owners like Maya and Carlos receive inquiries across Instagram DMs, SMS, WhatsApp, and their website widget. Currently, they must manually switch between these apps to answer questions, track orders, or generate quotes. This fragmentation leads to lost revenue, delayed responses, and administrative overload. OHC lacks a unified, multi-tenant inbox built to safely centralize these channels and provide AI Agents the context they need to assist seamlessly.

  ## Research Report
  - **Shopify Inbox**: Centralizes chat and email but struggles with advanced conversational commerce (e.g. custom quoting in DM).
  - **WeCom / DingTalk**: Superb native integrations and auto-assignments, heavily relying on high-performance message buses.
  - **Wix / Squarespace**: Focuses on simple forms, neglecting real-time asynchronous channels like WhatsApp.
  - **Operator Communities**: Consistently mention that dropping a lead in IG DMs is the #1 source of lost custom orders.

  ## Design Doc
  ### Architecture diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : owns
      Inbox ||--o{ ChannelAdapter : connects
      ChannelAdapter ||--o{ Conversation : spawns
      Conversation ||--o{ Message : contains
      Conversation ||--o{ Contact : involves
  ```
  ### Mobile UX flow
  1. User (Maya) opens OHC on her 375px mobile screen.
  2. The home dashboard highlights a "Work Triage" feed with unread IG DMs and WhatsApp inquiries.
  3. Maya taps a WhatsApp inquiry; the native chat view opens with an AI-generated draft quote for a custom cake.
  4. She reviews the draft and taps "Send".

  ### AI agent integration points
  - Agents subscribe to the unified `Conversation` event stream.
  - On new messages, the Customer Assistant reads the `Conversation` history and drafts a response.
  - Operations/Sales agents observe intent (e.g., "I'd like to book") to extract structured data (Date, Amount) and present action cards.

  ### Key design decisions
  - **Shared Omnichannel Data Model**: We will implement a `Conversation`, `Message`, `Contact`, and `ChannelAdapter` entity structure natively in Rust.
  - **Multi-Tenant Isolation**: Strict enforcement using `tenant_id` at the database (Postgres RLS) and Rust service boundary.
  - **WebSocket Real-time**: Secure SSE/WebSocket delivery to the Tauri frontend for real-time responsiveness.

  ## Implementation Prompt
  **Goal:** Build the Native Rust Omnichannel Customer Support engine for OHC.
  **CUJ:** As Maya, I want all my WhatsApp and IG DM messages to flow into a single OHC Work Triage inbox on my mobile phone, so I can review AI-drafted replies and respond from one place.
  **Requirements:**
  1. Implement the native Rust service layer for `Conversation` and `Message` management.
  2. Create channel adapters for Web Widget and SMS.
  3. Wire up the internal event bus to notify AI agents of new messages for auto-drafting.
  4. Ensure 100% E2E test coverage of the Work Triage inbox using Playwright (starting from the home page).
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
