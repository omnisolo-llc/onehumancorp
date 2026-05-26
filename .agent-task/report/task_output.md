issue_title: "Implement an Omnichannel AI Voice Receptionist & Interactive IVR"
issue_description: |
  ## Title
  Implement an Omnichannel AI Voice Receptionist & Interactive IVR

  ## Problem Statement
  Small business owners like Carlos (handyman) and Fatima (food cart) lose high-value customers because they cannot physically answer the phone while working. A missed call often means a lost lead or order. While text-based agents handle online inquiries, the traditional phone channel remains a critical, yet unoptimized, entry point for less tech-savvy customers or urgent requests.
  From a non-technical owner's perspective: "I'm on a ladder fixing a roof and my phone rings. I can't answer it. The customer calls someone else. I just lost a $500 job because I couldn't pick up the phone."

  ## Research Report
  **Findings:**
  - **SMB Reliance on Phone:** Despite digital trends, local services (plumbers, handymen, restaurants) receive a significant portion of their highest-intent leads via phone.
  - **Competitor Analysis:**
    - *Shopify/Wix:* Lack native voice AI receptionists. They rely on third-party VoIP integrations that are hard to set up.
    - *GoDaddy:* Offers basic call forwarding and voicemail, but no intelligent AI handling.
  - **Technology Enablers:** Low-latency Speech-to-Text (STT) and Text-to-Speech (TTS) models (e.g., ElevenLabs, Whisper) combined with LLMs now allow natural, real-time voice interactions.
  - **Opportunity:** Integrating voice natively into OHC allows the AI to not just take messages, but actually book appointments, quote prices, and accept pre-orders directly over the phone, syncing instantly with the unified inbox.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      Caller[Customer calling via PSTN] -->|Twilio/SignalWire SIP Trunk| IngressGateway[Voice Ingress Gateway];
      IngressGateway <-->|WebRTC/WebSockets| VoiceAgent[AI Voice Agent Orchestrator];
      VoiceAgent <--> STT[Speech-to-Text Engine];
      VoiceAgent <--> LLM[LLM Context Engine];
      VoiceAgent <--> TTS[Text-to-Speech Engine];
      LLM <--> Memory[Episodic Memory & Unified Inbox Sync];
      LLM <--> AgentDepartments[AI Departments:\nBooking, Quoting, Orders];
      Memory --> OHC_App[OHC Mobile/Desktop App];
  ```

  ### Data Model Entity-Relationship Diagram
  ```mermaid
  erDiagram
      BUSINESS ||--o{ VOICE_AGENT_CONFIG : "has"
      VOICE_AGENT_CONFIG ||--o{ CALL_LOG : "generates"
      CALL_LOG ||--o{ TRANSCRIPT : "contains"
      BUSINESS ||--o{ UNIFIED_INBOX : "has"
      CALL_LOG }|--|| UNIFIED_INBOX : "syncs to"

      BUSINESS {
          string id PK
          string name
          string phone_number
      }
      VOICE_AGENT_CONFIG {
          string id PK
          string business_id FK
          boolean enabled
          string voice_style
          string instructions
      }
      CALL_LOG {
          string id PK
          string business_id FK
          string caller_number
          datetime start_time
          datetime end_time
          string audio_recording_url
      }
      TRANSCRIPT {
          string id PK
          string call_log_id FK
          string role "user | agent"
          string text
          datetime timestamp
      }
      UNIFIED_INBOX {
          string id PK
          string business_id FK
      }
  ```

  ### UI Wireframes & Screen Flow (375px)
  - **Call Settings Dashboard (Mobile):**
    - **Header:** "Phone Agent Settings"
    - **Toggle:** "Enable AI Receptionist" (Large, accessible switch).
    - **Card 1: Business Identity:** "Agent Voice" (Dropdown to select voice style - e.g., Professional, Friendly).
    - **Card 2: Capabilities:** Checkboxes for what the agent is allowed to do (e.g., "Book Appointments", "Give Price Estimates", "Take Messages Only").
    - **Card 3: Business Context:** A simple text box: "What should the agent know? (e.g., We are closed on Tuesdays, we don't do roof repairs)."

  ### Mobile UX Flow
  1. **Activation:** User navigates to the "Communications" tab, taps "Phone Agent", and toggles it on.
  2. **Configuration:** User selects a voice and ticks a few boxes on what the agent can handle.
  3. **Execution:** An incoming call is routed to the agent.
  4. **Review:** A push notification appears: "New Voice Lead: John booked a plumbing inspection for tomorrow at 2 PM." Tapping it opens the unified inbox showing the call transcript and audio recording.

  ### AI Agent Integration Points
  - **Customer Service (CS) Department:** Handles the primary interaction, greeting, and intent routing.
  - **Operations Department:** Interacts with the booking engine or order management system if the caller wants to schedule a service or place an order.
  - **Memory & Inbox:** The interaction must be serialized and pushed to the unified inbox, maintaining continuity across SMS, Web, and Voice channels.

  ### Key Design Decisions
  - **Real-time Streaming over WebSockets:** To achieve conversational latency (< 800ms), audio streams must be processed over WebSockets rather than distinct HTTP requests.
  - **Native Twilio/SIP Integration:** Hiding the complexity of SIP trunks and phone numbers. OHC provisions the number automatically for the user.
  - **"Grandmother Test" Configuration:** Avoiding complex IVR flow builders. Instead, the user just types instructions in plain English, and the LLM determines the conversational flow.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Implement the backend infrastructure and mobile-first UI for the Omnichannel AI Voice Receptionist.
  **User Facing Outcome (CUJ):** A business owner can activate an AI phone agent with one tap. When a customer calls their provided OHC business number, they speak naturally to an AI that can answer FAQs, book appointments, or take messages, based on the owner's plain-English instructions. The owner receives a push notification and can view the call transcript/recording in their unified inbox.
  **Acceptance Criteria:**
  - 375px-optimized UI for configuring the voice agent.
  - Backend ingestion service capable of receiving SIP/Twilio streams.
  - Real-time processing loop linking STT -> LLM -> TTS with < 1s latency.
  - Full multi-tenant isolation ensuring voice context strictly belongs to the respective business.
  - Integration with the Unified Inbox to display call logs and transcripts.

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
