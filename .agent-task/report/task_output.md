issue_title: "Invisible Multilingual Voice AI Receptionist"
issue_description: |
  # Invisible Multilingual Voice AI Receptionist

  **Priority:** P0
  **Estimated Scope:** Large

  ## Problem Statement

  Small business owners—especially those working with their hands (like Carlos the handyman or Fatima the food cart operator)—cannot answer the phone while on the job. Missing a call often means losing a customer, but answering it interrupts their critical work. For non-native English speakers like Fatima, language barriers on the phone can add friction and limit customer reach. Existing "AI receptionists" are complex to configure, require a separate app or VOIP setup, and sound robotic. OHC users need an invisible, zero-config AI voice agent that simply answers calls when they are busy, speaks the customer's language natively, takes pre-orders, answers FAQs, schedules bookings, and gracefully hands off the transcript and intent to the unified inbox.

  ## Research Report

  ### Market Gap & Competitive Analysis

  - **Current SMB Solutions:** Most small businesses rely on basic voicemail or miss calls entirely. Paid solutions like RingCentral or traditional answering services are expensive ($100s/month), lack context about the business's actual inventory/schedule, and require complex PBX configuration.
  - **AI Competitors (Bland AI, Vapi, Retell):** Highly capable but designed for developers. A baker (Maya) cannot configure a webhook to sync her calendar with a Vapi assistant.
  - **The OHC Opportunity:** By leveraging our existing unified data layer (inventory, calendar, pricing), we can auto-generate a custom Voice AI receptionist for *every* OHC merchant instantly. The agent already knows the menu, the prices, and the open calendar slots.

  ### Data Findings

  - 62% of calls to small businesses go unanswered.
  - Unanswered calls result in an estimated 40% loss of potential revenue for service-based businesses.
  - Real-time latency for natural voice conversation needs to be < 800ms.

  ## Design Doc

  ### User Journey & Activation (Zero-Config)

  1. **Acquisition/Onboarding:** When setting up their OHC profile, the user provides their basic business info. OHC automatically provisions a local phone number.
  2. **Activation:** The user flips a single toggle: "AI Receptionist: ON". They can choose the voice style (Friendly, Professional, Casual).
  3. **The Customer Experience:** A customer calls the OHC provisioned number. The AI answers naturally: "Hi! Thanks for calling Fatima's Halal Cart. Are you looking to place a pre-order or check our location today?"
  4. **Multilingual Support:** If the customer speaks Spanish, the AI seamlessly switches to Spanish, matching the native accent and tone, translating menu items contextually.
  5. **The Merchant Experience:** Carlos gets a push notification: "New Booking: Sink Repair for Maria on Tuesday at 2 PM. Deposit Paid." The full call transcript and audio recording are available in the OHC Unified Inbox.

  ### Architecture

  ```mermaid
  erDiagram
      MERCHANT ||--o{ BUSINESS_PROFILE : configures
      BUSINESS_PROFILE ||--|{ INVENTORY_LEDGER : references
      BUSINESS_PROFILE ||--|{ CALENDAR_BOOKINGS : references
      BUSINESS_PROFILE ||--|| AI_RECEPTIONIST_CONFIG : has
      AI_RECEPTIONIST_CONFIG {
          string provisioned_phone_number
          boolean is_active
          string voice_persona
          string language_fallback
      }
      INCOMING_CALL ||--|| AI_RECEPTIONIST_SESSION : initiates
      AI_RECEPTIONIST_SESSION ||--|{ CONVERSATION_TURN : contains
      AI_RECEPTIONIST_SESSION }|--|| UNIFIED_INBOX_MESSAGE : generates
      CONVERSATION_TURN {
          string intent
          string extracted_entities
          string audio_snippet_url
      }
  ```

  ```mermaid
  sequenceDiagram
      actor Customer
      participant Telephony_Gateway (Twilio/SignalWire)
      participant OHC_Voice_Edge (WebRTC)
      participant Core_LLM_Orchestrator
      participant OHC_Data_Layer (Inventory/Calendar)
      participant OHC_Unified_Inbox

      Customer->>Telephony_Gateway: Dials Merchant Number
      Telephony_Gateway->>OHC_Voice_Edge: Inbound WebRTC Stream
      OHC_Voice_Edge->>Core_LLM_Orchestrator: Initiates Session with Merchant Context
      Core_LLM_Orchestrator->>OHC_Data_Layer: Fetch Live Inventory/Schedule
      Core_LLM_Orchestrator->>OHC_Voice_Edge: Streams Audio Greeting
      OHC_Voice_Edge->>Customer: "Hi, how can I help?"
      Customer->>OHC_Voice_Edge: "Do you have vegan cakes for tomorrow?"
      OHC_Voice_Edge->>Core_LLM_Orchestrator: Speech-to-Text & Intent parsing
      Core_LLM_Orchestrator->>OHC_Data_Layer: Query 'Vegan Cake' availability for tomorrow
      Core_LLM_Orchestrator->>OHC_Voice_Edge: Streams Audio Response
      OHC_Voice_Edge->>Customer: "Yes, we do! Would you like to order one?"
      Customer->>Telephony_Gateway: Hangs up after ordering
      Core_LLM_Orchestrator->>OHC_Unified_Inbox: Creates Actionable Message (Order Intent + Transcript)
  ```

  ### Mobile UX Flow (375px)

  - **Home Screen Card:** "AI Receptionist answered 3 calls today. 1 new booking." -> Tappable to open Inbox.
  - **Settings Screen (Advanced):**
    - Clean Translucent Glass header.
    - Large toggle switch: "AI Receptionist".
    - Dropdown: "Voice Style" (with play button preview).
    - Toggles for capabilities: "Allow AI to take bookings", "Allow AI to process pre-orders".
    - NO mention of prompts, LLMs, or WebRTC.

  ### AI Department Coordination

  - **CS Department (Voice Edge):** Handles real-time STT/TTS and conversational flow.
  - **Operations Department:** Triggered by CS to check inventory ledgers or calendar slots in the background.
  - **Finance Department:** Triggered if a deposit is needed (sends an SMS payment link to the caller while still on the phone).

  ### Security & Multi-Tenancy (Zero Trust)

  - The LLM context window must be strictly partitioned by `tenant_id`. The prompt template injects only the specific merchant's data.
  - PII from callers (phone numbers, names) is encrypted at rest and bound to the merchant's isolated tenant space.

  ## Implementation Prompt

  **Objective:** Build the edge-native Voice AI Receptionist module for OneHumanCorp.

  **User Facing Outcome:** A merchant can turn on an AI voice assistant with a single toggle. Customers calling the merchant's provisioned number experience a sub-800ms latency, natural-sounding conversation capable of answering questions based on the merchant's live OHC data (inventory, calendar) and seamlessly shifting languages based on the caller's speech.

  **Key Requirements:**

  1. Integrate a telephony provider (e.g., Twilio) to route inbound calls to an edge worker via WebRTC/SIP.
  2. Implement an orchestrator that utilizes a fast STT engine, a reasoning LLM, and a low-latency TTS engine.
  3. The LLM must be dynamically grounded in the specific merchant's data (prices, availability) fetched from the OHC internal APIs, isolated securely.
  4. Output structured intents (Bookings, Orders, General Inquiries) and a full transcript to the `UNIFIED_INBOX` upon call completion.
  5. The system must support mid-conversation language switching automatically.

  **Out of Scope for this iteration:** Do not build the frontend UI for this yet; focus on the backend orchestration, telephony routing, real-time context injection, and inbox integration. Do not prescribe the specific LLM provider, but ensure the architecture supports swapping models for latency optimization.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
