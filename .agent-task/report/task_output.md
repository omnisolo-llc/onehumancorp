issue_title: "Build Real-Time AI Voice Receptionist & Dispatch Mesh"
issue_description: |
  # Real-Time AI Voice Receptionist & Dispatch Mesh

  ## Problem Statement

  Small business owners—like Carlos the handyman, Maya the baker, and Fatima the food cart operator—spend their days with their hands full. When they are fixing a pipe, baking a cake, or taking an order at the counter, they cannot answer the phone. Missed calls mean missed revenue.

  Currently, their options are:
  1.  **Let it go to voicemail:** The customer hangs up and calls a competitor.
  2.  **Hire a traditional answering service:** Expensive, inflexible, and incapable of negotiating a price, checking complex inventory, or securely taking a deposit.
  3.  **Hire a dedicated receptionist:** Far too costly for most solopreneurs and micro-businesses.

  Small business owners need an invisible assistant that intercepts incoming phone calls 24/7, sounds completely natural, speaks multiple languages (e.g., answering Fatima's customers in Arabic or English), negotiates quotes, checks live calendar availability, books the job, takes a deposit over the phone, and dispatches the work via push notification—all with sub-300ms conversational latency.

  ## Research Report

  ### Market Gap
  Leading SMB platforms (Shopify, Wix, Squarespace) focus almost entirely on web/e-commerce or visual point-of-sale. They treat voice calls as an out-of-band communication channel completely disconnected from the platform's core operating system. While some integrations exist for SMS, native voice remains untouched.

  ### Current Landscape
  *   **Twilio / Plivo:** Provide raw telephony APIs (SIP trunks, WebRTC) but require immense engineering effort to build conversational logic.
  *   **Bland AI / Vapi.ai:** Emerging voice API layers offering STT -> LLM -> TTS pipelines. They lower the barrier but operate outside the business's core system of record (inventory, CRM, calendar).
  *   **Traditional IVR:** "Press 1 for hours." Customers hate this.

  ### The OneHumanCorp Opportunity
  OHC has a unique advantage because we *are* the core system of record. By building a native Real-Time AI Voice Receptionist & Dispatch Mesh, OHC can seamlessly bind telephony to our universal capacity ledger, autonomous quote engine, and multi-tenant billing. This allows the AI agent to confidently state, "Yes, we have three vegan chocolate cakes left for pickup today. I can hold one for you for $15," and actually execute the transaction.

  ### Latency Targets
  To achieve a natural conversational feel, the complete turnaround (VAD -> STT -> LLM -> TTS -> Audio Playback) must remain under **300ms - 500ms**. This requires edge-streaming architectures and interrupting capabilities.

  ## Design Doc

  ### Architecture Summary

  The Voice Dispatch Mesh operates as a high-performance streaming bridge between telephony providers (e.g., Twilio SIP trunks) and OHC's internal Agentic Departments.

  ```mermaid
  graph TD
      UserPhone((Customer Phone)) <--> |SIP / RTP| TelephonyProvider[Telephony Edge / SIP Trunk]
      TelephonyProvider <--> |WebSockets / gRPC Stream| VoiceGateway[OHC Voice Gateway Edge]

      subgraph OHC Voice Dispatch Mesh
          VoiceGateway <--> |Audio Stream| STT[Streaming STT - Deepgram/Whisper]
          VoiceGateway <--> |Audio Stream| TTS[Streaming TTS - ElevenLabs/PlayHT]
          VoiceGateway <--> |Text / Tokens| Orchestrator[Voice Orchestrator]
      end

      subgraph OHC AI Departments
          Orchestrator <--> |Context / Tools| OpAgent[Operations Agent]
          Orchestrator <--> |Context / Tools| CSAgent[Customer Service Agent]
          Orchestrator <--> |Context / Tools| SalesAgent[Sales & Quoting Agent]
      end

      subgraph OHC Core Ledgers
          OpAgent <--> InventoryDB[(Inventory Ledger)]
          CSAgent <--> CRMDB[(CRM & Memory)]
          SalesAgent <--> BookingDB[(Booking & Calendar)]
      end
  ```

  ### Entity-Relationship Model (Multi-Tenant)

  ```mermaid
  erDiagram
      TENANT ||--o{ VIRTUAL_NUMBER : owns
      TENANT ||--o{ CALL_SESSION : tracks
      VIRTUAL_NUMBER ||--o{ CALL_SESSION : routes
      CALL_SESSION ||--o{ CALL_TRANSCRIPT : records
      CALL_SESSION ||--o{ AGENT_ACTION : generates

      TENANT {
          uuid id PK
          string name
          uuid organization_id
      }

      VIRTUAL_NUMBER {
          uuid id PK
          uuid tenant_id FK
          string phone_number "E.164 format"
          string provider "e.g., Twilio"
          jsonb routing_config
      }

      CALL_SESSION {
          uuid id PK
          uuid tenant_id FK
          uuid virtual_number_id FK
          string caller_id
          timestamp start_time
          timestamp end_time
          string status "active, completed, dropped"
          string recording_url
      }

      CALL_TRANSCRIPT {
          uuid id PK
          uuid call_session_id FK
          string speaker "user, agent"
          text content
          timestamp timestamp
      }

      AGENT_ACTION {
          uuid id PK
          uuid call_session_id FK
          string action_type "book_calendar, create_order, take_deposit"
          jsonb payload
      }
  ```

  ### AI Department Coordination

  1.  **Ingress:** A call hits the Virtual Number. The Voice Gateway answers and triggers the **Customer Service Agent** (CS Agent) with the `caller_id`.
  2.  **Memory Retrieval:** The CS Agent instantly queries the CRM. "Ah, this is Sarah. She ordered a cake last week."
  3.  **Conversation:** The STT/TTS pipeline handles the fluid conversation. The user asks, "Can I get the same cake for next Tuesday?"
  4.  **Handoff / Tool Use:** The CS Agent invokes the **Sales Agent** to check inventory and the **Operations Agent** to schedule the pickup slot.
  5.  **Execution:** The agents execute transactions against the ledgers invisibly.
  6.  **Dispatch:** Upon completion, the system sends a push notification to Maya's (the baker) phone: "Sarah just booked a cake for Tuesday. Deposit paid via phone."

  ### Mobile-First UX Flow (375px)

  The business owner manages this feature via the OHC Mobile App using macOS-style Translucent Glass materials and clean UniFi modular dashboard cards.

  1.  **Dashboard Hub:**
      *   A clean, frosted glass card titled "AI Receptionist".
      *   **Status Indicator:** Glowing green dot "Active & Listening".
      *   **Metrics:** "3 calls handled today • $150 revenue secured."
  2.  **Receptionist Settings Screen:**
      *   **Toggle:** Large, satisfying iOS-style toggle to turn the Receptionist On/Off.
      *   **Voice Selection:** A horizontal scrolling list of circular avatars representing different AI voices (e.g., "Professional," "Friendly," "Casual"). Tapping plays a quick audio preview.
      *   **Language Selection:** Dropdown for primary/secondary languages (e.g., English, Spanish, Arabic).
  3.  **Call Log & Transcripts Screen:**
      *   A feed of recent calls formatted like a modern messaging app.
      *   Each item shows: Caller Name (if known) or Number, Duration, and an AI-generated 1-sentence summary (e.g., "Booked a pipe repair for tomorrow at 2 PM").
      *   Tapping a call opens a translucent modal with the full text transcript and an audio playback scrubber.
  4.  **Action Approvals (Optional):**
      *   If the owner requires manual approval for high-value actions, a push notification opens a card: "Agent quoted $500 for roof repair. Approve?" with prominent "Approve" (Green) and "Reject/Take Over" (Red) buttons.

  ### Technical & Security Constraints

  *   **Latency:** STT -> LLM -> TTS pipeline must stream chunks to achieve sub-500ms time-to-first-audio.
  *   **Interruption Handling:** The WebRTC/WebSocket gateway must support immediate interruption (barge-in) when the user starts speaking over the agent.
  *   **Zero Trust / Multi-Tenancy:** `CALL_SESSION` and `VIRTUAL_NUMBER` entities must enforce strict RLS (Row Level Security) based on the `organization_id` tied to the current request context. An agent working for Carlos cannot access Maya's calendar.
  *   **Graceful Degradation:** If the LLM provider latency spikes, the Voice Gateway should seamlessly fallback to a standard intelligent voicemail prompt rather than providing dead air.

  ## Implementation Prompt

  **To the Implementer Agent:**

  Your objective is to build the core backend streaming infrastructure and data models for the Real-Time AI Voice Receptionist & Dispatch Mesh.

  **User Journey (CUJ):**
  A customer calls a business's OHC virtual number. The OHC Voice Gateway answers the call, streams the audio to the STT provider, feeds the text to the internal Customer Service AI Agent, and streams the agent's response back via TTS—all in real-time. The agent successfully books an appointment in the database and ends the call.

  **Acceptance Criteria:**
  1.  **Data Model:** Implement the multi-tenant database schemas for Virtual Numbers, Call Sessions, and Call Transcripts as defined in the ER diagram. Ensure strict tenant isolation.
  2.  **WebSocket/Streaming Edge:** Implement a high-performance API endpoint (e.g., using WebSockets or gRPC) capable of receiving real-time audio chunks from a simulated telephony provider.
  3.  **Agent Orchestration:** Create a simplified conversational loop that ties a mock STT stream -> OHC Agentic Framework -> mock TTS stream.
  4.  **Tool Execution:** Demonstrate the agent correctly invoking a system tool (e.g., booking a calendar slot) during an active call session.
  5.  **No UI implementation required:** Focus entirely on the backend streaming, data models, and agent orchestration.

  Do not prescribe specific low-level libraries unless absolutely necessary; focus on standard Rust/Go concurrency patterns suitable for the OHC environment. Ensure all code complies with our Zero Trust and mobile-first architectural mandates.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []