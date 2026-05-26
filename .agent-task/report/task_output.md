issue_title: "Implement Autonomous AI Voice Receptionist"
issue_description: |
  # Architecture Issue Brief: Autonomous AI Voice Receptionist & Dispatcher

  ## Problem Statement
  Small business owners—especially service providers like Carlos (Handyman) and Priya (Boutique)—lose significant revenue because they cannot answer phone calls while working. Traditional platforms force them to rely on basic voicemail or expensive third-party answering services. This causes "Communication Lag" and "Operational Fatigue," leading to lost bookings and unhappy customers. They need a system that acts as an invisible teammate, autonomously answering calls, booking appointments, answering FAQs, and routing urgent issues, all managed effortlessly from a mobile device.

  ## Research Report
  *   **Competitor Audit:** Shopify, Wix, Squarespace, and GoDaddy offer basic text-based auto-responders or integrations with third-party tools like Twilio or specialized call centers, but none offer a native, fully integrated, autonomous voice AI teammate out of the box.
  *   **Persona Alignment:**
      *   **Carlos (Handyman):** Frequently on ladders or under sinks. Needs an AI to answer "Are you available next Tuesday?" and take a deposit immediately.
      *   **Maya (Baker):** Hands covered in flour. Needs an AI to answer "Do you make gluten-free cakes?" based on her catalog and take a custom order request.
  *   **Key Insight:** AI must move beyond text. Voice is the most frictionless medium for local service customers. An integrated voice agent that has context of the business's inventory, calendar, and FAQs is a major differentiator (Quadrant 1 "Leapfrog Zone").

  ## Design Doc

  ### System Architecture

  The Voice Receptionist sits at the edge, ingesting PSTN calls, processing them via real-time STT/TTS and LLM logic, and interacting with OHC's core event mesh to check inventory, calendar, and customer history.

  ```mermaid
  graph TD
      subgraph Edge & Telephony
      PSTN[PSTN / Twilio SIP] --> VoiceGateway[Voice Gateway Microservice]
      end

      subgraph Autonomous Voice Department
      VoiceGateway <--> STT[Speech-to-Text Engine]
      VoiceGateway <--> TTS[Text-to-Speech Engine]
      VoiceGateway <--> Orchestrator[Voice Dialog Orchestrator]
      Orchestrator <--> ContextManager[Context & Memory Manager]
      end

      subgraph OHC Core Mesh
      Orchestrator -->|Publish Intent| NATSEventMesh[NATS Event Mesh]
      NATSEventMesh -->|Check Availability| CalendarLedger[Unified Calendar Ledger]
      NATSEventMesh -->|Check Pricing| CatalogLedger[Product/Service Catalog]
      NATSEventMesh -->|Send Notification| OmnichannelInbox[Omnichannel AI Inbox]
      end

      ContextManager -->|Fetch Customer Profile| IdentityLedger[Universal Buyer Identity]
  ```

  ### Data Model & Entity Relationship

  ```mermaid
  erDiagram
      TENANT ||--o{ BUSINESS_CATALOG : "owns"
      TENANT ||--o{ CALENDAR_LEDGER : "owns"
      TENANT ||--o{ VOICE_AGENT_CONFIG : "configures"
      VOICE_AGENT_CONFIG {
          string persona_style
          string custom_instructions
          boolean is_active
      }
      TENANT ||--o{ CALL_SESSION : "records"
      CALL_SESSION {
          string session_id
          timestamp start_time
          timestamp end_time
          string caller_phone
          text transcript
          text ai_summary
      }
      CALL_SESSION ||--o{ OMNICHANNEL_INBOX_ITEM : "generates"
      UNIVERSAL_BUYER_IDENTITY ||--o{ CALL_SESSION : "makes"
  ```

  ### Multi-Tenant Isolation & Security
  *   **Zero Trust:** Each incoming call is assigned an ephemeral, tightly scoped JWT tied strictly to the specific business tenant (SPIFFE/SPIRE identity). The Voice Dialog Orchestrator can only query the Calendar Ledger and Catalog Ledger for that specific tenant.
  *   **Data Isolation:** Conversation logs and STT transcripts are stored in tenant-isolated datastores with row-level security.

  ### Mobile-First UX Flow (375px)
  The configuration must pass the "grandmother test." No SIP trunks, no IVR trees, no Twilio credentials.

  1.  **Dashboard Card:** "Set up your AI Receptionist."
  2.  **Toggle:** Enable "AI Voice Receptionist" (1-tap).
  3.  **Persona Selection:** Choose a voice style (e.g., "Professional & Friendly", "Casual & Upbeat").
  4.  **Knowledge Base:** The system automatically pulls from the business's existing catalog, FAQs, and calendar. User can add custom instructions via a simple text area: "Tell them I don't work on weekends."
  5.  **Action Feed:** Missed calls handled by the AI appear as cards in the Omnichannel Inbox: "AI booked an appointment for Carlos at 2 PM. [Listen to summary / View Transcript]."

  ## Implementation Prompt
  **Objective:** Implement the backend and mobile UI for the Autonomous AI Voice Receptionist.
  **Critical User Journey (CUJ):**
  1.  Business owner (Carlos) navigates to "Settings -> Communications" in the OHC mobile app and toggles "Enable AI Voice Receptionist".
  2.  Carlos selects a voice profile and provides a single sentence of context: "I'm currently offering a 10% discount on plumbing jobs."
  3.  A customer calls Carlos's OHC-provided phone number.
  4.  The AI answers dynamically, understands a request for plumbing, checks the Unified Calendar Ledger, offers an available slot, and books the appointment.
  5.  Carlos receives a push notification and a summary card in his Omnichannel Inbox containing the appointment details and an audio snippet of the interaction.

  **Acceptance Criteria:**
  *   1-tap activation in the mobile UI.
  *   Voice agent can access business context (calendar, catalog) without manual configuration.
  *   Interactions are recorded and summarized in the Omnichannel Inbox.
  *   Sub-500ms latency for conversational turns to ensure natural flow.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
