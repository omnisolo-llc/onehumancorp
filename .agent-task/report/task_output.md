issue_title: "[Research] AI Unified Inbox & Agent Triage Architecture"
issue_description: |
  # Research Report: AI Unified Inbox & Agent Triage Architecture

  ## 1. Problem Statement
  Small business owners and operators (like Maya the Baker or Nora the Agency Principal) are overwhelmed by work intake from multiple channels: Instagram DMs, website contact forms, WhatsApp, emails, and SMS. They lack a single place to see what needs attention and frequently drop balls on incoming opportunities. Existing helpdesk software (like Zendesk) is too complex and admin-heavy for them. They need a "Unified Inbox" that doesn't just collect messages but actively triages them, categorizes them (lead, support, spam), drafts replies using context, and turns requests into actionable work (quotes, bookings, tasks) without technical configuration.

  ## 2. Research Report
  - **Market Context:** Small businesses use 4-6 different apps to communicate with customers. Connecting them typically requires Zapier or complex integrations that owners lack the time or skill to maintain.
  - **The OHC Opportunity:** By providing a native Unified Inbox powered by the "Work Triage" agent capability, OHC can become the indispensable command center for the owner's day.
  - **Competitor Analysis:**
    - *Zendesk/Intercom:* Enterprise-focused, high learning curve, "ticket" paradigm feels robotic to small businesses.
    - *Shopify Inbox:* Good for basic eCommerce chat, but lacks deep operational integration (e.g., turning a chat into a custom service quote or booking).
    - *HubSpot CRM:* Powerful but often too sales-heavy and complex for a solopreneur or micro-business.
    - *Tencent Workbuddy / WeCom:* Excellent at unifying communication and work, but tailored for the Chinese market ecosystem. OHC needs to replicate this unified feel for global channels.

  ## 3. Design Doc

  ### Architecture Diagram (Conceptual)
  ```mermaid
  graph TD
    subgraph Channels
      IG[Instagram DMs] --> IngestAPI
      WA[WhatsApp] --> IngestAPI
      Email[Email] --> IngestAPI
      Web[Web Form/Chat] --> IngestAPI
    end

    IngestAPI[Ingestion API] --> EventBus(Event Bus)

    subgraph OHC Backend
      EventBus --> TriageAgent{Work Triage Agent}
      TriageAgent -- Evaluates --> ContextDB[(Customer Context & Memory)]
      TriageAgent -- Categorizes --> UnifiedDB[(Unified Thread DB)]

      UnifiedDB --> CustomerAgent{Customer Agent}
      CustomerAgent -- Drafts Reply --> UnifiedDB

      UnifiedDB --> OpsAgent{Operations Agent}
      OpsAgent -- Proposes Action (Quote/Booking) --> UnifiedDB
    end

    UnifiedDB --> Frontend(Flutter PWA / App)
    Frontend -- Owner Approves/Sends --> OutboundAPI[Outbound API]
    OutboundAPI --> Channels
  ```

  ### Data Model & Sync Protocol
  - **Unified Inbox Thread (`UnifiedThread`):** The core entity representing a conversation, regardless of source. Includes `tenant_id`, `customer_id`, `channel_source`, `status` (needs_attention, pending_customer, resolved), and `priority`.
  - **Message (`Message`):** Individual messages within a thread.
  - **Action Proposal (`ActionProposal`):** AI-generated suggestions attached to a thread (e.g., "Draft Quote for Custom Cake", "Schedule Consultation").

  ### AI Agent Coordination
  - **Work Triage Agent:** The first line of defense. It reads incoming messages, identifies the customer, assesses urgency, and categorizes the intent (sales lead, support question, spam). It sets the `UnifiedThread` status.
  - **Customer & Relationship Assistant:** Triggered by Triage. It looks up past orders and preferences, then drafts a context-aware reply for the owner to review.
  - **Operations / Sales Assistant:** If Triage identifies intent to buy or book, it drafts an Action Proposal (e.g., pre-fills a quote or booking link) to present alongside the message draft.

  ### Mobile UX Flow (375px)
  1. **The Feed (Home Screen):** The owner opens the app and sees a prioritized feed. Instead of "Inbox (42)", they see: "3 Urgent Inquiries, 1 Quote Ready to Send".
  2. **Thread View:** Tapping an inquiry opens the unified thread. The design uses translucent Apple-style materials.
  3. **Agent Intervention:** At the bottom of the thread, instead of just an empty keyboard, the owner sees:
     - The AI-drafted reply (editable).
     - A clearly styled Action Card (e.g., "Attach Custom Cake Quote: $150").
  4. **One-Tap Action:** The owner taps "Send & Quote", dispatching the message back through the original channel and updating the thread status.

  ## 4. Implementation Prompt
  **Feature Name:** OHC Unified Inbox & Triage Feed
  **Target Persona:** Maya the Home Baker
  **Outcome:** Maya wakes up to 5 Instagram DMs. OHC has already grouped them, drafted replies based on her availability and pricing, and prepared deposit links. She reviews and approves them in 2 minutes from her phone.

  **Critical User Journey (CUJ):**
  1. Maya opens the OHC mobile app.
  2. She navigates to the new "Triage Feed".
  3. She selects an inquiry from "Sarah" about a vegan cake.
  4. She sees the AI-drafted reply: "Hi Sarah! Yes, we do vegan cakes. A 6-inch is $65. Would you like to book for this Saturday?" and a one-tap button to attach a deposit link.
  5. Maya taps "Send with Deposit Link".

  **Next Actions for Engineering:**
  1. **Backend:** Implement the `UnifiedThread` and `Message` data models in PostgreSQL with strict `tenant_id` RLS.
  2. **API/Integration:** Create the initial Web Form/Chat ingestion endpoint to simulate external channel input.
  3. **AI Agent:** Develop the Work Triage Agent prompt and workflow to analyze incoming messages, categorize them, and trigger the Customer Assistant for drafting.
  4. **Frontend:** Build the Mobile-First "Triage Feed" and "Thread View" UI, ensuring touch targets are >= 44x44px and integrating the agent-drafted reply presentation.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
