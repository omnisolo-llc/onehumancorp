issue_title: "Native Omnichannel Chat Engine & WhatsApp Cloud Connector"
issue_description: |
  **Title**: Native Omnichannel Chat Engine & WhatsApp Cloud Connector

  **Problem Statement**:
  Small-business owners like Maya (Home Baker) and Carlos (Field Service Owner) receive customer inquiries across WhatsApp, Instagram, and web chat. They currently have to jump between different apps on their phone to reply, losing context on previous conversations, orders, and payments. They need a single, unified inbox within the OHC assistant where all customer messages arrive, where they can reply directly, and where the OHC assistant can help draft replies based on customer history and business context.

  **Research Report**:
  I have audited the Chatwoot source code repository to understand how a robust omnichannel inbox is architected. Chatwoot separates the domain into core models (`Conversation`, `Message`, `Contact`, `Inbox`) and specific channel connectors (e.g., `Channel::Whatsapp`, `Channel::WebWidget`, `Channel::Instagram`).
  For WhatsApp, Chatwoot uses the WhatsApp Cloud API (and providers like 360dialog), storing provider configurations, template syncing, and handling webhook events for incoming messages and delivery status.
  Since OHC is retiring third-party Chatwoot in favor of a native Rust implementation, we need to build a native omnichannel routing engine and our first channel connector for the WhatsApp Cloud API. This will allow OHC to directly ingest WhatsApp messages into a unified tenant-scoped inbox without relying on external Chatwoot infrastructure.
  The native engine must support tenant isolation (row-level security), robust webhook processing via our AI Job Queue (to prevent dropped messages during traffic spikes), and seamless integration with the OHC Flutter mobile app for real-time push notifications and chat UI.

  **Design Doc**:
  - **Triggers**: Customers messaging the business's WhatsApp number; OHC system sending automated updates (e.g., booking confirmations) via WhatsApp templates; Owner replying from the OHC mobile app.
  - **Actions**:
    - Webhooks from Meta (WhatsApp Cloud API) are received by an OHC API endpoint.
    - The webhook payload is securely verified and enqueued as a job.
    - A worker processes the job, identifying the correct tenant/inbox, updating or creating a Contact, and creating a Message within a Conversation.
    - Real-time events are dispatched to the owner's active OHC Flutter client.
    - Outbound messages typed by the owner (or drafted by the AI assistant and approved) are sent back to the WhatsApp Cloud API.
  - **User Experience**: The owner opens the OHC app and sees a "Work Triage" feed or "Inbox" where WhatsApp messages appear alongside other tasks. The interface clearly indicates the message came from WhatsApp. The owner can reply with text or media just like in a native chat app, and the customer receives it on their WhatsApp.

  **Implementation Prompt**:
  Implement a native Rust omnichannel chat engine and WhatsApp Cloud API connector in the `onehumancorp/mono` repository.

  Acceptance Criteria:
  1. Owners can connect their Meta WhatsApp Cloud API credentials to their OHC workspace.
  2. Incoming WhatsApp text messages arrive in the OHC unified inbox in near real-time.
  3. Owners can reply to these WhatsApp messages from the OHC interface, and the reply is successfully delivered to the customer's WhatsApp.
  4. The system reliably processes incoming Meta webhooks using a queued background job system to tolerate temporary backend degradation.
  5. All data is properly isolated by tenant in the database.
  6. The chat UI on the mobile app (375px width) works flawlessly without horizontal scrolling.

  **Priority**: P1

  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
