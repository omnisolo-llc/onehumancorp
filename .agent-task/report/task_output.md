issue_title: "[Research] AI Unified Communications & Multi-Channel Work Triage Architecture"
issue_description: |
  # Mission Queue Protocol: AI Unified Communications & Multi-Channel Work Triage Architecture

  ## Problem Statement
  Small business owners and operators (Maya, Carlos, Priya, Leo, Fatima) struggle to manage customer communications across multiple fragmented channels (Instagram DMs, WhatsApp, SMS, Email, Phone Calls). This fragmentation leads to lost leads, delayed responses, and overwhelming mental load. They need a single, unified "Work Triage" feed that not only aggregates messages but actively categorizes, prioritizes, and drafts responses using context from their business data (bookings, inventory, past interactions). The current OHC platform lacks a cohesive architecture for multi-channel ingestion and AI-driven triage via "The Ambassador" agent.

  ## Research Report
  - **Competitive Analysis:**
    - **Shopify Inbox:** Good for basic chat and email, but lacks deep multi-channel integration (e.g., WhatsApp, Instagram DMs) and advanced AI drafting. Relies heavily on manual responses or basic, rigid auto-replies.
    - **Wix Inbox:** Good aggregation, but AI features are mostly limited to "improving tone" or generating generic replies, not acting as an autonomous customer success agent.
    - **HubSpot:** Powerful but too complex for small business owners; feels like a CRM rather than a work assistant.
    - **Tencent Workbuddy / WeCom:** Excels at unified communications and workflow integration, serving as the gold standard for blending chat with business operations.
  - **Market Need:** SMBs lose up to 30% of potential revenue due to missed or delayed responses across various channels.
  - **Key Finding:** A simple aggregation tool is insufficient. The solution must intelligently triage messages, link them to an omnichannel identity graph, and provide actionable next steps (e.g., "Draft reply and send payment link") using "The Ambassador" agent to proactively draft complete, accurate responses.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Channels: IG, WhatsApp, SMS, Email] -->|Webhooks/APIs| B(Omnichannel Gateway)
      B --> C{Customer Identity Resolution Engine}
      C --> D[(Unified Customer Graph DB)]
      C --> E[Event Mesh / Queue]
      E --> F(The Ambassador Agent)
      F -->|Query Context via RAG| D
      F -->|Action Required Queue| G[(PostgreSQL: Unified Inbox Table)]
      G --> H[Mobile App Feed 375px]
      H -->|1-Tap Approve| I[Omnichannel Dispatcher]
      I --> A
  ```

  ### Mobile UX Flow (375px First)
  1. **Unified Feed View:** A chronological feed combining all channels. Each item shows channel icon, sender, snippet, and an AI-generated priority tag (e.g., "Urgent: Booking Inquiry", "Action Required: Deposit Pending").
  2. **Message Detail View:** Shows the full conversation history. Crucially, it includes an "AI Assistant Draft" box at the bottom.
  3. **Action Triggers:** The AI draft box includes one-tap actions (e.g., "Approve Draft & Send", "Edit Draft", "Attach Payment Link").
  4. **Context Panel (Collapsible):** Shows key customer info (past orders, loyalty status) drawn from the unified backend.

  ### AI Agent Integration Points
  - **Ingestion Analysis:** Upon receiving a message, a background worker triggers the AI to categorize the intent (inquiry, complaint, booking request, etc.).
  - **Draft Generation:** The Ambassador Agent pulls relevant business context via RAG (e.g., "Maya's calendar is full this weekend", "Vegan cakes are in stock") against the tenant's product catalog and customer history to draft a context-aware response.
  - **Action Recommendation:** The AI identifies potential actions (e.g., creating a booking, sending an invoice) and presents them as structured buttons in the UI.
  - **Zero-Touch Fallback:** If AI confidence is low, it escalates to a human-only reply but provides suggested data points.

  ### Key Design Decisions
  - **Asynchronous Processing:** AI processing happens asynchronously via an event mesh/queue to ensure real-time ingestion isn't blocked.
  - **Unified Data Model:** All messages are normalized into a single `UnifiedMessage` table, regardless of the source channel, simplifying frontend rendering and AI processing.
  - **Identity Resolution:** Links an Instagram handle to an email address if they've purchased before, creating a single `Customer` entity per tenant.
  - **Proactive Drafting & Owner-in-the-Loop:** Move from read-reply to read-approve. The AI drafts responses but does not auto-send by default (configurable per channel/intent), ensuring the owner maintains control through 1-tap "Approve".

  ## Implementation Prompt
  **Goal:** Implement the backend architecture and mobile-first frontend for the AI Unified Communications & Work Triage feature using "The Ambassador" agent.
  **CUJ:**
  1. Owner (e.g., Maya) receives an Instagram DM asking about a past order or custom cake.
  2. The message appears in the unified "Work Triage" feed on her mobile device, tagged as "High Priority: Inquiry".
  3. Maya taps the message.
  4. She sees the conversation history and a perfectly accurate AI-drafted response: "Hi! I'd love to make a custom cake for you. What date do you need it for?"
  5. Maya taps "Approve & Send".
  **Acceptance Criteria:**
  - Functional ingestion webhook endpoints for at least two channels (e.g., Mock SMS, Mock Email) that normalize data into a unified schema via an Omnichannel Gateway.
  - Asynchronous worker queue (Event Mesh) that processes incoming messages and triggers "The Ambassador" agent (mocked AI provider for tests) to generate a classification and draft response.
  - Customer Identity Resolution Engine that matches incoming identifiers to existing customer records.
  - Mobile-first (375px) Flutter/PWA UI displaying the unified feed with AI drafts and 1-tap action buttons.
  - E2E Playwright test verifying the flow from message ingestion, AI drafting, to owner approval and simulated dispatch.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
