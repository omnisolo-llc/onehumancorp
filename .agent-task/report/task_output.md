issue_title: "Agentic Omnichannel Customer Memory & Unified Inbox"
issue_description: |
  ## Mission Queue Protocol: Agentic Omnichannel Customer Memory & Unified Inbox

  **Title**: Agentic Omnichannel Customer Memory & Unified Inbox

  **Problem Statement**:
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, web forms, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" (e.g., Shopify Inbox, Wix Inbox) simply aggregate messages without context. They require the owner to manually type responses, often lacking the customer's purchase history or past interactions across other channels. This creates a reactive, labor-intensive process that doesn't scale for a solopreneur.

  **Research Report**:
  Based on the competitive analysis from `[research]_ai_unified_inbox_differentiation.md` and `ohc_smb_market_report.md` (Pain Point #3: Omnichannel Chaos - 14%):
  - **Shopify Inbox**: Aggregates chat and email but relies heavily on manual responses or basic, rigid auto-replies. It does not proactively draft contextual responses based on full customer history across all channels.
  - **Wix Inbox**: Good aggregation, but AI features are mostly limited to "improving tone" or generating generic replies, not acting as an autonomous customer success agent.
  - **Zendesk/Intercom**: Enterprise-grade and far too complex/expensive for a single-person SMB.
  - **OHC Opportunity**: Leverage our "Teammate" AI philosophy. The Customer Success Agent (The Ambassador) doesn't just aggregate messages; it reads them, queries the customer's omnichannel identity graph (purchase history, past bookings, previous DMs), and proactively drafts a complete, accurate response. The owner just sees an "Action Required: Approve Reply" card in their mobile feed.

  **Design Doc**:

  *Architecture Diagram*
  ```mermaid
  graph TD
      A[External Channel: IG/WhatsApp/Email] -->|Webhook| B(Omnichannel Gateway API)
      B --> C{Identity Resolution Engine}
      C -->|Lookup/Create| D[(Unified Customer Graph DB)]
      C --> E[Event Bus]
      E --> F[The Ambassador Agent]
      F -->|Query Context| D
      F -->|Query Inventory/Availability| G[(OHC Core DB)]
      F -->|Draft Reply| H[Action Required Queue]
      H --> I[Mobile App Feed 375px]
      I -->|1-Tap Approve| J[Omnichannel Dispatcher API]
      J -->|Send Response| A
  ```

  *Mobile UX Flow*
  1. Maya receives an Instagram DM: "Do you have any vegan cakes available for tomorrow?"
  2. The Omnichannel Gateway receives the webhook. The Identity Resolution Engine links this IG handle to an existing customer profile.
  3. The Ambassador Agent queries inventory, sees vegan cakes are in stock, and drafts a reply.
  4. Maya opens the OHC app. Her feed shows a high-priority Action Card:
     - **Context**: "Customer inquiry from Jane Doe (IG: @jane.doe). Past purchases: 2 custom cakes."
     - **Draft**: "Hi Jane! Yes, we have vegan chocolate cakes available for tomorrow. Would you like to reserve one? Here's the deposit link: [Link]"
  5. Maya taps the large "Approve & Send" button (touch target > 44px). The reply is sent via IG DM.

  *Key Design Decisions*
  - **Identity Resolution**: A deterministic mapping system linking disparate identities (email, phone, social handles) to a single `UnifiedCustomer` record.
  - **Agentic Proactivity**: The system must not wait for the user to open the inbox to start drafting. Drafts are generated asynchronously upon message receipt.
  - **Zero Trust & Security**: Strict multi-tenant isolation via `tenant_id` on all message, customer, and identity records.

  **Implementation Prompt**:
  Implement the core backend data models and the Identity Resolution Engine for the Omnichannel Customer Memory feature.
  1. Create the `unified_customers` and `customer_identities` (linking emails, phone numbers, and social handles to the customer) database schemas with strict row-level security (RLS) for tenant isolation.
  2. Implement an `OmnichannelGateway` service that accepts normalized incoming messages and performs identity resolution to map the message to a `unified_customer`.
  3. Create the `inbox_messages` schema to store conversations, linked to the `unified_customer` rather than a single disjointed session.
  4. Ensure the Ambassador Agent is triggered upon a new message, provided with the full customer context, and generates a draft reply stored in an `action_required` queue.
  5. The target persona is Maya the Baker, who needs to handle Instagram DMs seamlessly without losing context of her repeat customers.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
