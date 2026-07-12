issue_title: "Implement the Omnichannel AI Unified Inbox with Action Cards"
issue_description: |
  # Architecture Design: Omnichannel AI Unified Inbox

  ## Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" simply aggregate messages without context.

  OHC needs an "Invisible AI Automation" capability: an Omnichannel Gateway that ingests messages across channels, classifies their intent, and proactively drafts context-aware responses as "Action Cards" in the owner's Agent Feed for simple mobile-first approval.

  ## Research Report & Competitive Analysis
  - **Traditional Inboxes (Shopify, Wix)**: Aggregate channels but rely on manual typing or basic auto-replies. Lack deep AI integration for proactive customer success.
  - **OHC Differentiation**: The system acts as a true assistant. It leverages the customer's full identity graph (purchases, past bookings) across all channels to draft a personalized reply, significantly reducing cognitive load. The owner just sees "Action Required: Approve Reply".

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram DM] -->|Webhook| E(Omnichannel Gateway)
      B[WhatsApp] -->|Webhook| E
      C[SMS/Email] -->|Webhook| E
      E --> F{Intent & Context Engine}
      F -->|Context: Inventory, Bookings, DB| G[LLM Draft Generator]
      G --> H[Action Card / Agent Feed]
      H -->|Mobile Approval| I[Response Dispatcher]
      I --> J[Customer Channel]
  ```

  ### Core Components & Data Model
  - **Message Entity**: Stores channel (Instagram, SMS, WhatsApp), external_message_id, customer_id, content, and metadata.
  - **Omnichannel Gateway Service**: Unifies incoming webhook webhooks into standard `Message` objects.
  - **Intent & Draft Engine**: Uses the LLM (Gemini Pro) to classify the intent of a message and drafts a contextual reply using tenant data.
  - **Action Card**: A feed item representing the drafted reply, allowing "Approve", "Edit", or "Discard".

  ### Mobile UX Flow (375px First)
  1. **Notification**: Owner gets a push notification: "New DM from Sarah about Vegan Cakes".
  2. **Agent Feed**: Owner opens OHC app to the Agent Feed. Top item is an Action Card.
  3. **Action Card UI**:
     - *Header*: Sarah (Instagram) • 2m ago
     - *Original Message*: "Do you have any vegan cakes available for pickup today?"
     - *AI Draft*: "Hi Sarah! Yes, we have 2 vegan chocolate cakes in stock for pickup today. Would you like me to hold one for you?"
     - *Actions*: A large primary "Approve & Send" button, and secondary "Edit" / "Discard" buttons.
  4. **Action**: Owner taps "Approve & Send". The card gracefully disappears with a success animation.

  ### AI Agent Integration
  The `Customer Success Agent` orchestrates the drafting process. It accesses tools for `CheckInventory`, `CheckBookingAvailability`, and `GenerateDraftReply`.

  ## Implementation Prompt
  **To Implementer:**
  Implement the backend core for the Omnichannel Unified Inbox.
  1. Create the database schemas for unifying cross-channel messages (e.g., `conversations`, `messages`, `channels`). Ensure strict multi-tenant isolation.
  2. Build the `OmnichannelGateway` service that can ingest simulated webhooks for at least two channels (e.g., Instagram, SMS) and standardize them.
  3. Integrate the LLM draft generation: When a new message is ingested, trigger a background job to draft a reply and create an `ActionCard` entry in the database.
  4. Build the API endpoints for the mobile app to fetch pending `ActionCard`s and approve/send them.

  **Acceptance Criteria:**
  - Full E2E Playwright test simulating an incoming message, the AI generating a draft Action Card, and the user approving it in the UI.
  - Unit tests covering the gateway and LLM integration.

  ## Priority
  P0
  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
