issue_title: "Unified Customer Inbox Architecture for Multi-channel Communication"
issue_description: |
  **Problem Statement**
  Small business owners like Carlos (the handyman) and Maya (the home baker) receive messages across many channels—Instagram DMs, emails, web chat, and SMS. Currently, there is no centralized, multi-tenant unified inbox. This forces owners to check multiple apps constantly, leading to missed leads and delayed responses. They need a single "Customer Inbox" where all inquiries surface in one place, allowing the Customer Success AI Agent to draft or auto-reply seamlessly.

  **Research Report**
  Leading platforms like Shopify (Inbox), Wix, and GoDaddy provide a unified inbox app that centralizes customer communications. Without a unified inbox, small businesses lose a significant competitive advantage. For OHC, where AI handles the heavy lifting, a central message repository is a strict requirement for the Customer Success agent to function effectively. The system needs to support multiple channels (Instagram, Email, Web Chat) and provide webhooks/polling for external platform integrations, while strictly enforcing row-level security per tenant.

  **Design Doc**
  * Architecture Diagram:
    ```mermaid
    erDiagram
      TENANT ||--o{ CUSTOMER : has
      TENANT ||--o{ CONVERSATION : owns
      CUSTOMER ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE {
        uuid id
        uuid tenant_id
        uuid conversation_id
        string channel "e.g., INSTAGRAM, EMAIL, WEB"
        string direction "INBOUND or OUTBOUND"
        text content
        boolean ai_draft
        timestamp created_at
      }
    ```
  * UI Wireframes/Flow:
    - Mobile (375px): A tab "Inbox" on the bottom nav. Shows a list of recent conversations. Tapping a conversation opens a chat view. Each message shows an icon indicating its source channel (Instagram, Web, etc.).
    - A quick "AI Draft" button allows the user to see what the Customer Success agent suggests replying, with options to "Send" or "Edit".
  * Mobile UX flow: Bottom nav -> Inbox -> Select Conversation -> View Thread -> Tap "Send AI Reply" or type manually.
  * AI agent integration: The Customer Success agent subscribes to new `INBOUND` messages and generates an `ai_draft` message on the thread, or auto-responds if confidence is high.
  * Key Design Decisions:
    - Multi-tenant RLS on all tables (`tenant_id`).
    - Generic `channel` string/enum to easily add WhatsApp, SMS, etc. later without schema changes.
    - AI drafts stored as messages with a flag to allow humans to review before sending.

  **Implementation Prompt**
  Implement the backend architecture for the Unified Customer Inbox.
  1. Create a database migration for the required core tables, ensuring all have `tenant_id` and strict PostgreSQL RLS policies.
  2. Implement the Go service layer with basic CRUD operations for conversations and messages, scoped to the authenticated tenant.
  3. Ensure the Customer Success AI agent has hooks to generate drafts for new incoming messages.
  4. Build comprehensive unit tests and write Playwright E2E tests verifying that a user can open the inbox, view a message, and send a reply.

  **Priority**: P0 (critical for Customer Success)
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
