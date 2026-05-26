issue_title: "[Architecture] Omni-Channel AI Conversational Commerce Hub"
issue_description: |
  ## Problem Statement
  Small business owners like Maya (a baker running her business primarily through Instagram DMs) and Carlos (a handyman communicating with clients via WhatsApp and SMS) are overwhelmed by incoming messages. When they sleep or are busy working on a job, they miss out on potential sales because they cannot instantly answer questions ("Do you do vegan cakes?", "How much to fix a leaky pipe?"), provide quotes, or take bookings/deposits. They need an invisible, always-on AI assistant that can seamlessly integrate into their existing social channels (Instagram, WhatsApp, SMS, Facebook Messenger), interact naturally with customers, automatically generate quotes, take deposits, and sync everything back to their unified OneHumanCorp (OHC) dashboard—all without requiring any complex setup or developer knowledge.

  ## Research Report
  **Competitor Systems Audit:**
  - **Shopify / Shopify Inbox:** Offers basic automated replies and chat routing, but it is heavily focused on traditional e-commerce web storefronts. Social integrations exist but usually hand off to a human rather than autonomously negotiating custom orders or bookings.
  - **ManyChat / Chatfuel:** Powerful for building complex bot flows, but they require the user (the business owner) to manually build decision trees and write copy. This completely fails the "zero code or manuals" requirement for our personas.
  - **Wix / Squarespace:** They offer unified inboxes, but autonomous AI agents capable of understanding context (e.g., custom cake requests vs. standard catalog items) and processing localized payments directly in the chat are absent or require third-party plugins.

  **Gaps Identified:**
  OHC lacks a unified ingestion and conversational engine capable of bridging third-party messaging protocols (Meta Graph API, Twilio, WhatsApp Business) with our internal AI Operations and Finance departments. We need a system that can understand intent, negotiate terms, issue localized payment links (like PIX or Apple Pay) directly in the chat, and sync state perfectly across devices, optimized for a mobile-first viewing experience.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph External Channels
          IG[Instagram DMs]
          WA[WhatsApp]
          SMS[SMS / Twilio]
      end

      subgraph OHC Edge & Ingress
          WG[Webhook Gateway]
          Sec[Spiffe/Spire Auth & Rate Limiter]
      end

      subgraph Core Conversational Hub
          MR[Message Router & State Manager]
          Mem[Episodic Context Memory]
          Intent[AI Intent Parsing Engine]
      end

      subgraph AI Departments
          Sales[AI Sales Agent]
          Ops[AI Operations/Booking Agent]
          Fin[AI Finance Agent]
      end

      subgraph Platform Services
          Ledger[Ledger & Invoicing]
          Cal[Booking & Calendar]
          Cat[Catalog & Inventory]
      end

      subgraph OHC Mobile App
          Inbox[Unified Inbox & Override]
          Dash[Analytics Dashboard]
      end

      IG --> WG
      WA --> WG
      SMS --> WG
      WG --> Sec
      Sec --> MR
      MR <--> Mem
      MR --> Intent

      Intent --> Sales
      Intent --> Ops
      Intent --> Fin

      Sales <--> Cat
      Ops <--> Cal
      Fin <--> Ledger

      MR --> Inbox
      Ledger --> Dash
      Cal --> Dash
  ```

  ### UI/UX Wireframes (375px Mobile First)
  **Screen 1: The Unified Inbox View**
  - **Header:** Clean, translucent glassmorphism top bar. "Inbox" with a toggle switch: [AI Auto-Pilot: ON / OFF].
  - **List View:** Clean cards for each conversation.
    - *Left:* Customer avatar.
    - *Middle:* Customer Name, Channel Icon (e.g., small IG logo), and snippet of the last message.
    - *Right:* Status Pill (e.g., "Quote Sent", "Deposit Paid", "Requires Human").
  - **Interaction:** Swiping a conversation left reveals "Take Over" (pauses AI) or "Mark Done".

  **Screen 2: Conversation Detail & AI Context**
  - **Main Area:** Standard chat bubbles. Customer bubbles on the left, AI responses (clearly but subtly badged as 'AI Assistant') on the right.
  - **Floating Action Button (FAB):** A unified action button to quickly generate a custom quote, insert a calendar link, or request payment if the human owner takes over.
  - **Top Context Bar:** Expandable "Deal Context" showing identified intent (e.g., "Custom Vegan Cake"), agreed price, and next steps.

  **Screen 3: Customer View (e.g., inside Instagram DM)**
  - **Experience:** Completely native to the app they are using. The AI responds naturally. When a payment is needed, the AI generates a rich link (e.g., `ohc.page/pay/123`) that opens a frictionless, mobile-optimized checkout supporting Apple Pay, Google Pay, or localized methods like Pix.

  ### Key Design Decisions
  1. **Zero-Configuration Onboarding:** Users authenticate their social channels via OAuth (e.g., "Connect Instagram"). The AI automatically reads their existing OHC catalog, pricing rules, and calendar availability to start answering immediately. No prompt engineering or flow-building required by the user.
  2. **Seamless Human Handoff:** If the AI detects an edge case it cannot handle safely (confidence < threshold) or if the customer specifically asks for a human, it pauses autonomous replies, alerts the business owner via mobile push notification, and moves the thread to a "Requires Attention" state.
  3. **Omni-Channel State Sync:** A single conversation state machine tracks intent regardless of channel. If a customer starts on IG and moves to WhatsApp, the context is maintained.
  4. **Data Isolation & Security:** Multi-tenant boundaries are strictly enforced. Each business's AI context window is securely isolated. All webhook ingestions pass through our SPIFFE/SPIRE authenticated gateway to prevent spoofing.

  ### AI Agent Integration Points
  - **Message Router:** Triggers the **Operations Agent** to fetch context (e.g., "Does Maya have time on Saturday?").
  - **Intent Parser:** Uses LLM classification to route to the appropriate specialized AI (Sales, Support, Booking).
  - **Finance Agent:** Automatically triggered when intent is "ready to buy" to draft the invoice and generate a secure payment link via the Ledger API.

  ## Implementation Prompt
  **To the Implementer Swarm:**
  Your objective is to build the backend routing, memory, and API services for the Omni-Channel AI Conversational Commerce Hub, and the mobile-first frontend unified inbox.
  - **CUJ:** Maya connects her Instagram account. A customer DMs her asking for a custom cake. The AI autonomously negotiates the details, checks Maya's availability, and sends a deposit link. Maya sees the interaction in her OHC unified inbox on her iPhone and watches the deposit clear without typing a single word.
  - **Acceptance Criteria:**
    - Secure webhook ingestion endpoints for at least one major channel (e.g., Meta Graph API).
    - Implementation of the conversational state machine and memory store.
    - Integration with the AI orchestration layer to classify intent and generate responses.
    - Mobile-first React/React Native unified inbox UI (375px optimized) using glassmorphism design tokens.
    - A "Take Over" mechanism that safely pauses AI execution for a specific thread.
    - **Constraints:** Do not prescribe specific database schemas or LLM models. Focus on the API contracts, multi-tenant security boundaries, and achieving sub-2-second response latency for the end user. Must pass all strict Zero-Trust guidelines.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
