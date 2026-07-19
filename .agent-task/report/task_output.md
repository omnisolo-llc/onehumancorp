issue_title: "[Architecture] Implement AI Omnichannel Customer Support Engine"
issue_description: |
  # Research Report: AI Omnichannel Customer Support Engine

  ## 1. Problem Statement
  Small business owners (e.g., Maya the Home Baker) receive customer inquiries across a multitude of channels: Instagram DMs, WhatsApp, Email, SMS, and website chat. Keeping up with these messages, which often ask the same basic questions (hours, pricing, policies), is a massive drain on their time. If they miss a message, they lose a sale or damage their reputation. Existing tools like Zendesk are too complex and expensive, while basic auto-responders lack context.

  ## 2. Research Report
  - **Market Context**: Consumers expect fast responses, often within minutes, regardless of the channel they choose. For a solo operator, checking 5 different apps while working is impossible. Tools like Intercom or Zendesk are designed for dedicated support teams, not the business owner.
  - **The OHC Opportunity**: OHC can provide an "Omnichannel Inbox" powered by a Customer Support AI Agent. This agent intercepts messages from all connected channels, attempts to resolve them automatically using the business's knowledge base (pricing, inventory, policies), and escalates to the owner only when necessary.
  - **Competitor Gaps**:
    - *Shopify Inbox*: Good for store chat and Instagram, but lacks deep AI resolution (mostly suggested replies).
    - *Zendesk/Intercom*: High learning curve, expensive, requires manual setup of complex routing rules.
    - *ManyChat*: Powerful for social media, but complex to configure and often feels like a rigid bot rather than a helpful assistant.

  ## 3. Design Doc
  ### Architecture
  - **Webhook Gateway**: A unified endpoint to receive webhooks from Meta (Instagram/WhatsApp), Twilio (SMS), and email providers.
  - **Message Normalization**: Convert incoming messages into a standard `OmniMessage` format regardless of source.
  - **AI Triage & Resolution Agent**: Analyzes the `OmniMessage`, queries the business context (inventory, active orders, knowledge base), and drafts a response.
  - **Routing Engine**: Decides if the AI should auto-reply or if the message requires owner intervention (escalation).

  ```mermaid
  sequenceDiagram
      participant Customer
      participant WebhookGateway
      participant MessageNormalizer
      participant AITriageAgent
      participant RoutingEngine
      participant UnifiedInbox

      Customer->>WebhookGateway: Sends Instagram DM
      WebhookGateway->>MessageNormalizer: Forwards Raw Payload
      MessageNormalizer->>AITriageAgent: Passes Standardized OmniMessage
      AITriageAgent->>AITriageAgent: Analyzes Intent & Queries Knowledge Base
      AITriageAgent->>RoutingEngine: Proposes Draft Response
      RoutingEngine->>UnifiedInbox: Flags as 'Needs Attention' (Escalation)
      UnifiedInbox->>Customer: Owner Approves & Sends Reply
  ```

  ### Data Model (PostgreSQL)
  - `Conversation`: Represents a thread with a customer (tenant_id, customer_id, channel, status).
  - `Message`: Individual messages within a conversation (tenant_id, conversation_id, direction, content, ai_generated).

  ### Mobile UX Flow (375px)
  1. **Unified Inbox**: A single, clean list of active conversations. Badges indicate the source channel (e.g., an Instagram icon).
  2. **AI Action State**: Conversations handled by the AI show an "AI Handled" chip.
  3. **Escalation View**: When the AI cannot resolve an issue, it pushes the conversation to a "Needs Attention" tab. The owner sees the full history and an AI-suggested draft reply, which they can edit or send with one tap.

  ## 4. Implementation Prompt
  **Feature Name**: AI Omnichannel Customer Support Engine
  **Target Persona**: Maya the Home Baker
  **Outcome**: Maya connects her Instagram and Email to OHC. When a customer DMs "Do you deliver to downtown?", the AI agent checks her delivery zones and auto-replies "Yes we do! Delivery to downtown is $5." Maya only sees the notification if the customer asks a complex custom question.

  **Next Actions**:
  1. Implement the `Conversation` and `Message` schemas.
  2. Build the Webhook Gateway to handle incoming Instagram DMs (as a starting point).
  3. Develop the AI Triage Agent prompt and logic to analyze intents and draft responses.
  4. Create the Unified Inbox UI in Flutter, prioritizing the "Needs Attention" flow.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
