issue_title: "OHC AI Centralized Work Inbox: Unifying Communications for SMB Operators"
issue_description: |
  # OHC AI Centralized Work Inbox: Unifying Communications for SMB Operators

  ## Problem Statement
  Operators (like Maya the baker or Carlos the handyman) are experiencing "Omnichannel Chaos." They currently have to manually monitor WhatsApp, Instagram DMs, SMS, and emails to track bookings and queries. This results in missed opportunities, fragmented customer history, and high anxiety because there is no single source of truth for "what needs my attention today." Existing platforms (Shopify, Wix) fail to provide a simple, unified, and actionable inbox, forcing owners to use disconnected third-party tools.

  ## Research Report
  - **Market Gap**: Based on the OHC Global SMB Market Research Report, "Omnichannel Chaos" (missing orders in DMs) accounts for 14% of major SMB pain points.
  - **Persona Need**: Maya needs to see a vegan cake inquiry from Instagram right next to an email request for a wedding cake quote, without jumping between apps.
  - **Competitive Landscape**:
    - *Shopify/Wix*: Require complex third-party apps (e.g., Gorgias) which are expensive and geared towards larger support teams, not solo operators on mobile.
    - *Zendesk/Intercom*: Too complex, expensive, and technical for small businesses.
    - *OHC Advantage*: OHC can integrate an AI-powered triage agent that not only unifies these messages but drafts replies and suggests actions (e.g., "Send Payment Link", "Book Appointment") natively within the platform.

  ## Design Doc
  ### Architecture (Mermaid)
  ```mermaid
  graph TD
      IG[Instagram DMs] -->|Webhook| WebhookIngress
      WA[WhatsApp] -->|Webhook| WebhookIngress
      Email[Email] -->|SMTP/API| WebhookIngress

      WebhookIngress --> MessageNormalizer
      MessageNormalizer --> DB[(PostgreSQL: Unified Messages)]

      DB --> AITriage[AI Work Triage Agent]
      AITriage -->|Analyzes Sentiment & Intent| DB
      AITriage -->|Drafts Suggested Reply| DB
      AITriage -->|Identifies Action (Quote/Book)| DB

      DB --> API[OHC Backend API]
      API --> UI[Flutter/PWA Mobile App]
  ```

  ### Mobile UX Flow (375px First)
  1. **Home Feed**: The first screen shows "Needs Attention Today." A unified list of cards combining urgent messages, pending bookings, and tasks.
  2. **Message Detail View**:
     - Clean, chat-like interface.
     - **Top**: Customer context (Name, LTV, past orders).
     - **Middle**: Message thread.
     - **Bottom**: AI-drafted reply ready to send or edit, with quick action chips (e.g., [Generate Quote], [Request Deposit]).
  3. **Visual Design**: Premium translucent glass materials. Clear status indicators (Unread = solid dot, Action Needed = amber highlight).

  ### AI Agent Integration
  - **Work Triage Agent**: Triggered on new message ingestion.
    - *Task 1*: Categorize message (Inquiry, Support, Urgent).
    - *Task 2*: Generate a context-aware suggested reply using the owner's tone and business knowledge base.
    - *Task 3*: Extract structured data (e.g., date, product interest) to suggest next steps.

  ## Implementation Prompt
  Implement the "Unified Inbox & Work Triage" capability for OHC.

  **User Journey:**
  1. The owner opens the app and sees a single list of incoming messages from various channels (simulated or real).
  2. The owner taps a message and sees the customer's history alongside an AI-suggested response.
  3. The owner can tap to approve/send the AI response or edit it.
  4. The owner can tap a suggested action (like creating a booking or sending a payment link) directly from the message view.

  **Acceptance Criteria:**
  - Create a unified message feed UI that works perfectly on a 375px mobile screen.
  - Integrate an AI agent capable of drafting replies based on message context.
  - Implement the backend logic to normalize and store messages from different channels into a single timeline.
  - Provide a Playwright E2E test verifying that a simulated incoming message appears in the feed, an AI reply is generated, and the owner can review/send it.
  - Must adhere to the premium Translucent Glass visual design system.
  - Ensure strict multi-tenant data isolation.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
