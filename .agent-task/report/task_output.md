issue_title: "OHC Voice AI Assistant for Offline Demand Capture"
issue_description: |
  # Research Report: Voice AI Assistant for Offline Demand Capture

  ## 1. Problem Statement
  Offline service providers like Carlos (Handyman) and fast-paced operators like Fatima (Food Cart) often miss incoming calls while they are actively working. Current voicemail solutions just leave recordings that the owner must manually transcribe and process later. They need an integrated voice AI agent that can act as a receptionist—answering calls, answering basic questions (e.g., availability, location), and capturing booking intents or orders directly into the OHC platform.

  ## 2. Research Report
  - **Market Context**: Most SMBs rely on basic cellular voicemail or clunky IVR systems. Modern AI solutions (like Twilio Voice AI or Bland AI) are standalone and do not integrate directly into an SMB's inventory, scheduling, or customer relationship systems.
  - **The OHC Opportunity**: OHC already has a mock `VoiceAIEdgeEngine` and scheduling/inventory capabilities. By tying these together, OHC can provide an intelligent agent that answers the phone, understands natural language, queries availability via the Operations Agent, and logs actionable intents directly into the unified inbox.
  - **Competitor Gaps**:
    - *Shopify*: Text/Web only. No native voice capabilities.
    - *Wix*: No voice-based booking or order capture.
    - *Traditional Voicemail*: Passive, no data extraction, creates manual work for the owner.

  ## 3. Design Doc
  ### Architecture
  - **Voice Ingestion (Twilio Integration)**: WebRTC/SIP integration using Twilio Programmable Voice to route incoming calls to the `VoiceAIEdgeEngine`.
  - **Streaming Transcription & LLM**: Stream audio chunks to a Speech-to-Text service, feed the text to the Gemini/MiniMax LLM acting as the receptionist, and stream the response back via Text-to-Speech.
  - **Data Model (PostgreSQL)**:
    - Enhance `CallSession` to include `linked_task_id` or `booking_id`.
    - `IntentAction` linked directly to the unified Work Triage feed.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant C as Customer
      participant T as Twilio Programmable Voice
      participant V as VoiceAIEdgeEngine
      participant L as LLM / "Receptionist"
      participant O as Operations Agent
      participant D as DB (PostgreSQL)

      C->>T: Incoming Call
      T->>V: WebRTC Stream
      V->>L: STT & RAG Context
      L->>V: Draft Response
      V->>T: TTS Stream
      T->>C: Voice Response
      L->>O: Parse Intent (e.g. PLACE_ORDER)
      O->>D: Save IntentAction / Create Order
  ```

  ### AI Integration
  - **Customer Success Agent ("The Receptionist")**: Answers the call, uses RAG on the owner's FAQs, menu, and availability.
  - **Operations Agent**: Processes the parsed intents (e.g., `CHECK_AVAILABILITY`, `PLACE_ORDER`) from the call and updates the backend.

  ### Mobile UX Flow (375px)
  1. **Owner View (Work Triage)**: A new incoming task card appears in the 375px mobile feed: "Voice Order from +1234567890: 2 Halal Platters for 12:30 PM Pickup".
  2. The card includes the parsed intent details, a short summary of the call, and a playback button for the audio recording.
  3. The owner clicks "Approve Order" to finalize it in the POS system or "Call Back" to manually address it.

  ## 4. Implementation Prompt
  **Feature Name**: OHC Voice AI Edge Engine Integration
  **Target Personas**: Carlos the Handyman, Fatima the Food Cart Operator

  **Outcome**: An AI voice agent answers missed calls, captures the customer's intent (booking or order), and places an actionable card in the owner's Work Triage feed.

  **Next Actions**:
  1. Upgrade `VoiceAIEdgeEngine` in `src/server/voice` from a mock implementation to support real-time audio streams via Twilio.
  2. Implement the streaming LLM context for "The Receptionist" agent, allowing it to query inventory and calendar availability.
  3. Add a unified feed card in the frontend (375px optimized) that displays parsed `IntentAction` records (e.g., "Requested Booking for Tomorrow") with a 1-tap approval action.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
