issue_title: "[architecture] Autonomous AI Voice Receptionist & Order Taker"
issue_description: |
  # [architecture] Autonomous AI Voice Receptionist & Order Taker

  ## Problem Statement
  Small business owners like Carlos (the handyman) and Fatima (the food cart operator) rely heavily on synchronous voice communication. While they work, they frequently miss incoming phone calls. Missing a call often means losing a job or a catering order, as the customer immediately calls the next business on Yelp or Google Maps. Existing voicemail is insufficient because it creates friction for the caller, who simply wants to know "Are you open?" or "Can you fix a leaky pipe today?" They need an invisible, always-on AI voice receptionist capable of picking up missed calls in real time, answering business-specific questions, negotiating time slots, capturing pre-orders, and summarizing the interaction context into the unified inbox.

  ## Research Report
  *   **Shopify / Wix / Squarespace:** E-commerce platforms completely ignore the synchronous voice channel. They assume all customer journeys are digital-first (web chat or email). They offer no native VoIP or AI receptionist integration.
  *   **GoDaddy:** Offers a unified inbox and a "smart terminal" with basic VoIP forwarding, but lacks a sophisticated, conversational AI receptionist that can *act* on the business data (book appointments, take orders).
  *   **Stand-alone SMB VoIP (RingCentral, Google Voice):** Provides auto-attendants (press 1 for hours, press 2 for sales), which provide a poor customer experience. They are not deeply integrated into a unified business graph (calendar, inventory, pricing).
  *   **OneHumanCorp (OHC) Differentiation:** OHC will provide the **Voice Ambassador**. When Carlos is under a sink, the Voice Ambassador answers the call, speaks naturally (with localized language support, like Arabic/English for Fatima), checks Carlos's unified calendar, books a tentative estimate slot, and sends an SMS recap to the customer. All audio and transcribed data are ingested into the OHC Unified Thread model.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      PSTN_NETWORK ||--o{ TWILIO_VOIP_GATEWAY : "Inbound Call"
      TWILIO_VOIP_GATEWAY }|--|| VOICE_STREAM_ROUTER : "WebRTC/SIP Audio"

      VOICE_STREAM_ROUTER {
          string spiffe_identity "Zero Trust Routing"
          string tenant_id "Multi-tenant Isolation"
      }

      VOICE_STREAM_ROUTER ||--o{ SPEECH_TO_TEXT : "Streams audio"
      SPEECH_TO_TEXT ||--o{ VOICE_AMBASSADOR_AGENT : "Transcribed Intent"

      VOICE_AMBASSADOR_AGENT ||--o{ AGENT_DEPARTMENTS : "Consults (Ops, CS, Calendar)"
      VOICE_AMBASSADOR_AGENT ||--o{ TEXT_TO_SPEECH : "Generates response"

      TEXT_TO_SPEECH ||--o{ VOICE_STREAM_ROUTER : "Streams audio back"

      VOICE_AMBASSADOR_AGENT }|--|| UNIFIED_THREAD : "Appends transcript/summary"
      UNIFIED_THREAD ||--o{ MOBILE_UI : "Notifies human"
  ```

  ### UI Wireframes & 375px Baseline
  **Core Layout: macOS-style Translucent Glass + Ubiquiti UniFi Modular Dashboard Cards**
  *   **Global Viewport:** 375px width.
  *   **Inbox List View Integration:** The AI Voice Call appears as a standard thread item in the Unified Inbox.
      *   **Badging:** A glowing audio waveform icon indicates "AI Voice Call Completed".
  *   **Voice Thread View:**
      *   A frosted glass player at the top to play the original call audio.
      *   A clean, summarized "AI Brief" card below it (e.g., "✨ Customer booked plumbing estimate for tomorrow 2 PM").
      *   An expanding "Full Transcript" section hidden behind a translucent "Read Transcript" toggle to save vertical space.
  *   **Advanced Settings (Voice Profile):**
      *   A section where the user can choose the AI's voice (gender, accent), supported languages, and select the default behavior for missed calls (e.g., "Only pick up if I decline", or "Always pick up after 3 rings").

  ### Mobile UX Flow
  1. **The Event:** Carlos's phone rings while he is busy. He hits "Ignore" or lets it ring out.
  2. **AI Takeover:** The call seamlessly routes to the OHC Voice Ambassador, which answers: "Hi, you've reached Carlos Handyman Services. Carlos is on a job right now. How can I help you today?"
  3. **The Conversation:** The AI negotiates an appointment time based on Carlos's real-time availability.
  4. **Notification:** Carlos receives a push notification: "✨ AI booked a new estimate for tomorrow at 2 PM. Call summarized."
  5. **Review:** He opens the app, taps the notification, and lands in the Unified Inbox thread. He sees the 1-sentence summary and the calendar slot. He can tap "Listen" to hear the 30-second recording if he wishes.

  ### AI Agent Integration Points
  *   **Voice Ambassador (Frontline):** Handles the real-time LLM interaction, managing latency, interruptions, and conversational turn-taking.
  *   **Operations Department:** Provides real-time capacity checks (e.g., "Are we sold out of vegan cakes?" or "Is Carlos free at 3 PM?").
  *   **Finance Department:** Can trigger SMS follow-ups containing dynamic payment links (e.g., "I'm texting you a link to pay the $50 deposit to secure the slot.").

  ### Key Design Decisions (Why, not How)
  *   **Seamless Inbox Integration:** The user does not need a separate "Voicemail" tab. Voice calls are treated as just another omnichannel message format (like Instagram DMs or SMS), normalizing the data structure into the Unified Thread.
  *   **Ultra-Low Latency Priority:** The conversational experience relies entirely on sub-500ms latency. The architecture mandates streaming audio directly to the edge agent, bypassing heavy central database queries during the active call.
  *   **Zero-Trust Voice Routing:** Audio streams are sensitive PII. The `VOICE_STREAM_ROUTER` strictly enforces SPIFFE identities to ensure one tenant's inbound audio cannot cross into another tenant's agent context.

  ## Implementation Prompt
  **To the Implementer Swarm:**
  Build the real-time voice ingestion and processing architecture for the "Autonomous AI Voice Receptionist" so business owners can automatically capture missed calls, negotiate bookings, and receive text summaries directly in their Unified Inbox.

  **Customer User Journey (CUJ):**
  1. The business owner misses a customer call.
  2. The system intercepts the call via SIP/WebRTC and connects it to the real-time Voice Ambassador.
  3. The AI converses naturally, checks calendar/inventory via internal tool calls, and fulfills the user's intent.
  4. Upon call completion, the system generates a concise summary and full transcript, appending them to the Unified Inbox thread and firing a push notification.

  **Acceptance Criteria:**
  *   **Mobile Parity:** Provide the call summary and audio playback UI tailored for a 375px viewport using Translucent Glass materials.
  *   **Real-time Streaming:** The pipeline must support bidirectional audio streaming.
  *   **Inbox Unification:** The resulting call transcript and summary must be appended to the existing unified thread data model.
  *   **Security:** Implement Zero-Trust multi-tenant isolation at the stream router level.
  *   **Latency Target:** Ensure the architecture supports a sub-1s conversational turnaround time (TTFB).

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
