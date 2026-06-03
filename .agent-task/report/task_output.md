issue_title: "[Omnichannel Inbox] Unified Inbox Architecture"
issue_description: |
  # OmniChannel Unified Inbox Architecture

  ## Problem Statement
  Small business owners (like Maya the baker and Carlos the handyman) are overwhelmed by fragmented customer communication across Instagram DMs, WhatsApp, SMS, and email. This leads to missed sales, dropped leads, and frustration. They need a single, unified inbox where all customer interactions are aggregated, and where the OHC AI can autonomously draft or auto-reply to common queries, saving them hours each week.

  ## Research Report
  *   **Competitor Analysis**:
      *   **Shopify**: Requires third-party apps (e.g., Gorgias) which are expensive and complex to set up.
      *   **Wix**: Basic unified inbox exists, but lacks true AI-agentic autonomous drafting or replying.
      *   **Square**: Fragmented, POS-focused communication.
  *   **Market Need**: The "Social Seller" segment is rapidly growing, where the primary channel is social DMs. 65% of SMBs report losing track of orders in Instagram DMs.
  *   **OHC Differentiation**: Unlike a static inbox, the OHC Unified Inbox will have AI-native capabilities. The "Customer Success Agent" will monitor incoming messages, identify intent (e.g., "Do you do vegan cakes?"), match against business knowledge (from the Universal Ledger/Catalog), and draft a response or reply autonomously if confident.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph External Channels
          IG[Instagram DM]
          WA[WhatsApp]
          SMS[SMS / Twilio]
          Email[Email / SendGrid]
      end

      subgraph OHC Backend
          IH[Ingress Handlers]
          MQ[Message Queue / Redis]
          CS_Agent[Customer Success AI Agent]
          DB[(PostgreSQL)]
          UI_API[GraphQL / REST API]
      end

      IG --> IH
      WA --> IH
      SMS --> IH
      Email --> IH

      IH --> MQ
      MQ --> CS_Agent
      MQ --> DB
      CS_Agent --> DB

      DB --> UI_API
  ```

  ### UX Flow (Mobile First - 375px)
  1.  **Bottom Navigation**: A new "Inbox" icon in the main tab bar, showing unread count.
  2.  **Inbox List View**: A unified list of threads, clearly displaying the channel icon (Instagram, SMS, etc.) next to the sender's avatar.
  3.  **Thread View**:
      *   Message history displayed like a standard chat interface.
      *   If the AI has drafted a response, it appears in a translucent "Glassmorphism" bubble above the input field with a quick "Approve & Send" or "Edit" button.
      *   Input field allows manual typing, attaching images (from catalog or camera), and quick actions (Send Quote, Send Payment Link).
  4.  **Settings**: "AI Autopilot" toggle per channel (Draft only vs. Auto-reply for high confidence queries).

  ### AI Agent Integration
  The Customer Success Agent subscribes to the message ingress event stream. Upon a new message, it uses context from the business's knowledge base and past interactions to determine the appropriate response strategy.

  ### Key Design Decisions & Why
  *   **Asynchronous Ingestion**: Messages must be ingested asynchronously via a message queue to ensure high availability and prevent webhook timeouts from external providers.
  *   **Unified Data Model**: All messages, regardless of source, must be normalized into a unified thread structure to allow the AI agent to maintain context across different communication channels for the same customer.
  *   **Human-in-the-Loop Default**: By default, the AI drafts responses rather than auto-replying. This builds trust with the non-technical business owner before they enable full autopilot.

  ## Implementation Prompt
  **Objective**: Implement the backend data foundation and API surface for the OmniChannel Unified Inbox, enabling the system to ingest, store, and serve messages across various channels.

  **Acceptance Criteria**:
  1.  Design a multi-tenant isolated database structure to store external messages and link them to unified conversations.
  2.  Develop the necessary API mechanisms to securely receive incoming webhooks from external channels (e.g., Instagram, WhatsApp).
  3.  Build the API capabilities to allow the frontend to fetch unified conversation threads and their underlying messages with proper pagination.
  4.  Implement the event hook to trigger the "Customer Success" AI agent upon successful message ingestion, enabling it to asynchronously generate a draft response.

  **Scope**: Medium
  **Priority**: P0
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
