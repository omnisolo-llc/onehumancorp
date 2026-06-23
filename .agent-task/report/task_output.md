issue_title: "[Architecture] Autonomous Agentic Booking and Quoting Engine"
issue_description: |
  # OHC Autonomous Agentic Booking and Quoting Engine

  ## Problem Statement
  Service providers like Leo (the music tutor) and Carlos (the handyman) need to manage their time and quotes efficiently. Currently, if Carlos visits a client to assess a repair, he has to write down a quote on paper or use a separate app. Similarly, Leo has to manually sync his calendar to avoid double bookings. They need an integrated system where an AI agent can automatically reply to routine questions, draft quotes directly from chat, and sync bookings with their existing external calendars (Google/Outlook).

  ## Research Report
  Our research has identified the following competitor capabilities and market gaps:
  *   **Squarespace / Calendly:** Handle booking effectively but require users to manage a separate, standalone tool disconnected from their primary inbox and CRM.
  *   **Shopify:** Focuses heavily on standard e-commerce and lacks native B2B or custom service quoting capabilities without expensive add-on apps.
  *   **Market Needs:** The modern solopreneur expects a unified workflow on their 375px mobile device. They need "Invisible AI Automation" that can:
      *   Draft instant localized invoices from quotes.
      *   Sync external calendar events (Google Calendar API, Microsoft Graph API) using simple OAuth flows to prevent double-booking.
      *   Surface complex queries from an Omnichannel Inbox while handling routine pricing/availability questions automatically.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Mobile Device
          App[OHC Mobile App 375px] --> InboxUI[Unified Inbox UI];
          App --> QuoteUI[Quote & Invoice Builder];
          QuoteUI --> LocalCRDT[(Local Cache CRDT)];
          InboxUI --> LocalCRDT;
      end

      App -- "Sync & Generate" --> Gateway[OHC API Gateway];

      Gateway --> MessageRouter[Omni-Channel Router Engine];
      Gateway --> BillingEngine[Instant Invoicing Engine];
      Gateway --> CalendarSync[Calendar Integration Engine];

      MessageRouter --> MainDB[(Cloud Postgres Ledger)];
      BillingEngine --> MainDB;
      CalendarSync --> MainDB;

      subgraph External Integrations
          MessageRouter <--> Instagram[Instagram / Twilio / Email];
          BillingEngine <--> PaymentProvider[Localized Payment Gateways];
          CalendarSync <--> ExtCalendar[Google / Outlook Calendar APIs];
      end

      Gateway --> Agents[AI Agent Swarm];

      subgraph Agent Departments
          Agents --> CSAgent[Customer Success: Auto-Reply & Triage];
          Agents --> SalesAgent[Sales: Draft Quotes from Chat];
          Agents --> OpsAgent[Ops: Update Order Status & Bookings];
      end
  ```

  ### Mobile UX Flow
  1. **Omnichannel Inbox:** User opens the OHC mobile app (375px) to view aggregated messages (Instagram, SMS, WhatsApp) in a glassmorphism-styled UI.
  2. **AI Triage:** Routine inquiries ("Are you available Tuesday?") are marked with an AI sparkle icon and handled by the Customer Success Agent checking connected calendar availability.
  3. **Quote Generation:** When a customer asks for pricing in chat, the Sales Agent drafts a quote. The user reviews the drafted Quote Card and taps "Approve & Send".
  4. **Booking & Invoice:** Upon customer approval, the quote is converted to an invoice via the Instant Invoicing Engine, applying local tax rates and payment methods. A corresponding calendar event is created through the Operations Agent.
  5. **Offline Mode:** Using local CRDT syncing, users can read past messages and draft quotes even when offline.

  ### AI Agent Integration Points
  *   **Customer Success Agent:** Uses RAG on the business's knowledge base and calendar to auto-reply to routine availability queries.
  *   **Sales Agent:** Detects buying intent and drafts formalized OHC Quotes for one-tap approval.
  *   **Operations Agent:** Syncs bookings with external Google/Outlook calendars and updates order statuses.

  ## Implementation Prompt
  Implement the Autonomous Agentic Booking and Quoting Engine.
  *   **User-Facing Outcome:** Users can view and reply to multi-channel messages, have AI draft context-aware quotes within the chat, generate localized invoices, and automatically sync bookings with their external calendars—all from a 375px mobile UI.
  *   **CUJ (Critical User Journey):**
      1. A customer requests a quote and availability via Instagram DM.
      2. The message appears in the unified OHC inbox.
      3. The AI Sales Agent checks the connected Google Calendar for availability and drafts a reply containing a quote for the requested service.
      4. The business owner reviews the draft on their mobile app and taps "Approve".
      5. The customer accepts the quote and pays via a localized link.
      6. The system automatically creates a booked event in the owner's Google Calendar and updates the CRM ledger.
  *   **Acceptance Criteria:**
      *   Mobile-first UI adhering to the 375px baseline and OHC design system.
      *   Support for OAuth connection flows for Google Calendar.
      *   AI Agents successfully intercept, triage, and draft quotes for incoming messages before presenting them to the user.
      *   Offline CRDT support for reading and drafting when disconnected.
      *   Strict zero-trust multi-tenancy isolation for webhook ingestion and financial data.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
