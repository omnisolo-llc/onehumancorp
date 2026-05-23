issue_title: "[Architecture] AI-Native Conversational Commerce & One-Click Checkout Engine"
issue_description: |
  # [Architecture] AI-Native Conversational Commerce & One-Click Checkout Engine

  ## Problem Statement
  Small business owners conduct a significant portion of their business via social media DMs (Instagram, WhatsApp, TikTok). The gap is that conversational sales are decoupled from the core commerce and ledger systems. They need an AI agent that can invisibly handle these DMs, understand intent, generate dynamic quotes, and instantly provide a secure, one-click checkout link within the chat—all while syncing with inventory and the global ledger.

  ## Research Report
  **Competitor Systems Audit:**
  - **ManyChat / Chatfuel:** Good for automated conversational flows but completely disconnected from backend inventory, scheduling, and payment ledgers. Requires manual integration via Zapier.
  - **Shopify Inbox:** Connects chat with Shopify products, but requires a rigid e-commerce setup. It struggles with custom, service-based quotes or complex booking deposits.
  - **Stripe Payment Links:** Secure and scalable, but lacks conversational context. The merchant still has to manually generate and copy-paste the link.

  **Gaps Identified:**
  OHC lacks a unified, multi-tenant conversational commerce layer where AI agents can autonomously interact across omnichannel inboxes and securely generate contextual, Zero-Trust localized checkout sessions without human intervention.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Omnichannel Inbox
          IG[Instagram DM] --> OmniGateway[OHC Omnichannel API Gateway];
          WA[WhatsApp] --> OmniGateway;
          SMS[SMS] --> OmniGateway;
      end

      OmniGateway --> AI_Inbox[AI Conversational Engine];

      AI_Inbox -- "Intent & Context" --> OHC_Brain[KAIROS Master Orchestrator];

      OHC_Brain --> Inventory[Universal Capacity & Inventory Ledger];
      OHC_Brain --> Pricing[Dynamic Pricing & Quoting Engine];

      Pricing -- "Generate Quote/Deposit" --> CheckoutEngine[Zero-Click Checkout Engine];
      CheckoutEngine --> Payments[Localized Payment Gateways];
      CheckoutEngine -- "Secure Payment Link" --> AI_Inbox;

      AI_Inbox -- "Reply + Link" --> OmniGateway;

      subgraph Mobile Device (375px)
          App[OHC Mobile Dashboard] --> StatUI[Live Inbox & Agent Feed];
          StatUI --> LocalDB[(SQLite SIPDB Cache)];
      end

      OHC_Brain -- "Sync State" --> App;
  ```

  ### Entity-Relationship Diagram (ERD)
  ```mermaid
  erDiagram
      ORGANIZATION ||--o{ CONVERSATION : owns
      CONVERSATION ||--o{ MESSAGE : contains
      CONVERSATION {
          uuid id PK
          string tenant_id FK
          string channel "Instagram, WhatsApp, etc."
          string customer_id
          string status "active, pending_human, resolved"
      }
      MESSAGE {
          uuid id PK
          uuid conversation_id FK
          string sender "customer, ai, human"
          text content
          timestamp created_at
      }
      CONVERSATION ||--o| QUOTE : generates
      QUOTE ||--o| CHECKOUT_SESSION : triggers
      QUOTE {
          uuid id PK
          uuid conversation_id FK
          jsonb items
          int total_amount
          string currency
      }
      CHECKOUT_SESSION {
          uuid id PK
          uuid quote_id FK
          string payment_link
          string status "pending, paid, expired"
          timestamp expires_at
      }
  ```

  ### Sequence Diagram (Checkout Flow)
  ```mermaid
  sequenceDiagram
      participant Customer
      participant OmniGateway
      participant AI_Inbox
      participant Inventory
      participant CheckoutEngine

      Customer->>OmniGateway: "Can I get a vegan cake for Saturday?"
      OmniGateway->>AI_Inbox: Route Message
      AI_Inbox->>Inventory: Check Capacity(Saturday, Vegan)
      Inventory-->>AI_Inbox: Available
      AI_Inbox->>CheckoutEngine: Generate Secure Link($50)
      CheckoutEngine-->>AI_Inbox: return https://pay.ohc/link
      AI_Inbox->>OmniGateway: "Yes! Here is the link to deposit: [link]"
      OmniGateway->>Customer: Deliver Message
  ```

  ### Mobile UX Flow & UI Guidelines (375px First)
  - **Inbox View (Dashboard Card):** A glassmorphic card on the home screen showing "Active AI Conversations (3)".
  - **Conversation Thread:** Displays the chat history. AI-generated responses are distinguished by a subtle translucent badge. A toggle switch at the top allows the user to "Take Over" the conversation manually.
  - **Agent Approval Mode:** Before sending a payment link over $500, the AI pushes a notification to the business owner: "Drafted quote for Wedding Cake ($600). [Approve & Send]".
  - **Visuals:** Use `backdrop-filter: blur(20px) saturate(200%)` and `background: rgba(255, 255, 255, 0.03)` for premium feel. All complex NLU settings are hidden behind an "Advanced Settings" switch to pass the grandmother test.

  ### AI Agent Integration Points
  - **Customer Support & Sales Dept:** NLU models parse incoming messages for product availability and pricing inquiries.
  - **Finance Dept:** Generates secure payment links and records pending deposits in the ledger.
  - **Operations Dept:** Temporarily reserves inventory or calendar slots while the checkout link is active (abandoned cart recovery handles timeouts).

  ### Key Design Decisions
  - **Decoupled Omnichannel Gateway:** To easily add new channels (e.g., TikTok) without rewriting the core conversational engine.
  - **Optimistic Locking on Inventory:** Prevents double-booking while an AI-generated payment link is pending in a chat, without permanently locking up resources if the customer ghosts.
  - **Zero Trust Multi-tenancy:** Multi-tenant isolation and secure identity (SPIFFE/SPIRE) are guaranteed at the KAIROS level, meaning the Conversational Engine only sees data for the specific tenant it's serving.

  ## Implementation Prompt
  **To Implementer Agent:**
  Build the unified Conversational Commerce Engine that connects the existing Omnichannel API Gateway to the KAIROS Orchestrator. Implement the AI agent logic to intercept messages, query the Universal Capacity Ledger, and interface with the Checkout Engine to generate a secure, localized payment link. Ensure the AI can format the response naturally and append the payment link. On the frontend, create the 375px-optimized "Live Inbox & Agent Feed" component using Slint/Rust, ensuring it follows the premium visual design mandate (glassmorphism) and includes a seamless "Take Over" toggle for human intervention. The component must gracefully degrade to a local SQLite cache for offline review. Do not include specific SQL implementation details.
issue_priority: P0
issue_estimated_scope: Large
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
