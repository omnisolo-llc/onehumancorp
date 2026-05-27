issue_title: "Unified Omnichannel Conversational Commerce Inbox"
issue_description: |
  # [Architecture] Unified Omnichannel Conversational Commerce Inbox

  ## Problem Statement
  Small business owners like Maya (baker selling via Instagram DMs) and Carlos (handyman handling SMS quotes) are losing potential customers because they cannot respond instantly to inquiries 24/7. Current inbox solutions from Shopify and Wix require manual intervention to close sales, answer specific questions (e.g., "do you do vegan cakes?"), or negotiate custom deposits. These platforms treat chat as a support tool, rather than an autonomous sales engine. Our target users need an invisible AI agent that can negotiate, quote, schedule, and collect payments directly within the channel the customer is already using (Instagram, SMS, WhatsApp), without the owner ever needing to open the app unless requested.

  ## Research Report
  - **Competitor Analysis**:
    - **Shopify Inbox**: Good aggregation of channels but relies on simple rules (auto-replies, FAQs). It lacks LLM-powered conversational commerce capable of context-aware negotiation or booking.
    - **Wix Chat**: Basic widget for web only, limited social integration, requires manual staff intervention for complex queries.
    - **GoDaddy Conversations**: Centralizes messages but still primarily a manual inbox.
  - **Market Findings**: 73% of consumers prefer to purchase directly through social media DMs or text if the friction of leaving the app is removed. For solo entrepreneurs (our personas), response latency is directly correlated with lost revenue.
  - **The OHC Opportunity**: Integrate an invisible "Sales & Support Agent" department that has full access to the business's inventory, pricing, and calendar. The agent autonomously handles inbound leads across Instagram, SMS, and Webchat, converts them to sales/bookings, and sends a final confirmation to the business owner.

  ## Design Doc

  ### Key Design Decisions & Architecture
  - **Zero-Trust & Identity**: Each channel webhook (e.g., Meta Graph API, Twilio) authenticates via secure Webhook Relay. The agent's actions within the tenant's data boundary are cryptographically signed using SPIFFE/SPIRE workload identities, ensuring tenant isolation.
  - **Agent Integration Points**:
    - **Sales Department Agent**: Engages in negotiation, answers product queries (RAG against the catalog).
    - **Finance Department Agent**: Generates secure payment links (Stripe/tap-to-pay) embedded in the chat.
    - **Operations Department Agent**: Updates inventory/calendar immediately upon successful payment.

  ### Mobile-First UI Wireframes (375px viewport)
  *The business owner's view of the Inbox on the OHC mobile app.*

  - **Layout**: Clean, macOS-style Translucent Glass materials. Ubiquiti UniFi modular cards.
  - **Header**: "Inbox" with a pill toggle `[ Needs Attention (2) | AI Handled (14) ]`.
  - **List View**:
    - Each thread is a card: `Customer Avatar | Name | Channel Icon (Insta/SMS)`
    - Subtitle: Last message snippet.
    - Status badge: `<Sparkle Icon> AI Negotiating` or `<Check Icon> Deposit Paid` or `<Alert Icon> Human Handoff`.
  - **Detail View**:
    - iMessage-like bubble interface.
    - AI's messages are subtly differentiated with a translucent emerald glow.
    - Bottom action bar for the owner to "Take Over" or "Approve Custom Quote".

  ### Architecture Diagram (Mermaid ER & Sequence)

  ```mermaid
  erDiagram
      TENANT ||--o{ CONVERSATION_THREAD : owns
      CONVERSATION_THREAD ||--o{ MESSAGE : contains
      CONVERSATION_THREAD }|--|| CHANNEL : originates_from
      CHANNEL {
          string type "Instagram, SMS, Web"
          string provider_id
      }
      CONVERSATION_THREAD {
          string status "AI_HANDLING, HUMAN_NEEDED, RESOLVED"
          string customer_id
      }
      MESSAGE {
          string sender "customer, ai_agent, human_owner"
          text content
          datetime timestamp
      }
      CONVERSATION_THREAD ||--o{ QUOTE : generates
  ```

  ```mermaid
  sequenceDiagram
      actor Customer
      participant MetaAPI as Instagram API
      participant WebhookRelay as OHC Webhook Relay
      participant RoutingAgent as Routing Agent (MCP)
      participant SalesAgent as Sales Agent
      participant FinanceAgent as Finance Agent
      participant DB as SIPDB / Postgres

      Customer->>MetaAPI: "Can you do a vegan cake for Saturday?"
      MetaAPI->>WebhookRelay: POST Webhook (Signed)
      WebhookRelay->>RoutingAgent: Ingest Message
      RoutingAgent->>DB: Load Tenant Context (Maya's Bakery)
      RoutingAgent->>SalesAgent: Dispatch Thread Context
      SalesAgent->>DB: Query Inventory (Vegan options = True)
      SalesAgent->>Customer: "Yes! We have chocolate and vanilla vegan cakes. $50 deposit to secure Saturday." (via MetaAPI)
      Customer->>SalesAgent: "Great, let's do chocolate."
      SalesAgent->>FinanceAgent: Request Deposit Link ($50)
      FinanceAgent->>DB: Create Pending Order & Link
      FinanceAgent-->>SalesAgent: Payment Link
      SalesAgent->>Customer: "Perfect. Here is your secure payment link: [Link]"
  ```

  ## Implementation Prompt
  **Outcome**: Implement the foundational data models, webhook receivers, and agent routing logic to enable the Unified Omnichannel Conversational Commerce Inbox.
  **CUJ (Core User Journey)**: A customer messages the business's connected Instagram account asking about a product. The OHC webhook receives the message, the Sales Agent processes the intent against the business catalog, and replies with a secure checkout link directly in the DM, all without the business owner's manual input. The business owner opens the OHC mobile app and sees the thread in the "AI Handled" tab with a "Deposit Paid" status.
  **Acceptance Criteria**:
  1. Webhook endpoints for Twilio (SMS) and Meta (Instagram/Messenger) are established with signature verification.
  2. The `ConversationThread` and `Message` entities are strictly tenant-isolated in the database.
  3. The AI routing layer successfully hands off incoming messages to the correct specialized agent (Sales vs. Support).
  4. The system can generate a contextual reply via the connected channel's API.
  5. The owner UI accurately renders the conversation history, differentiating AI vs Customer messages on a 375px width screen.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []