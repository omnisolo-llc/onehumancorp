issue_title: "Architect Autonomous Voice AI Phone Attendant Engine"
issue_description: |
  # Architect Autonomous Voice AI Phone Attendant Engine

  ## Problem Statement
  Small business owners like Carlos (handyman) and Fatima (food cart operator) miss an estimated 30-50% of customer calls because they are busy on the job, driving, or serving other customers. Missed calls mean missed revenue. Existing voicemail is slow, non-interactive, and often ignored by callers who just call the next business on Google. These owners need an invisible, always-on AI voice attendant that acts as a real receptionist: answering calls, taking deposits, providing business hours, answering common questions, and even sending SMS quotes while the owner is entirely hands-off.

  ## Research Report
  - **Market Context**: Missed calls cost SMBs thousands of dollars monthly. Solutions like Google Voice provide basic transcription, but lack interactive AI. Newer startups (e.g., Bland AI, Vapi, Retell) provide conversational AI infrastructure, but require complex developer integration.
  - **Competitor Gap**: Shopify and Wix do not offer native telephony or voice AI. They rely on third-party apps that do not deeply integrate with the merchant's calendar, inventory, or unified inbox.
  - **The OHC Opportunity**: By deeply integrating an autonomous voice agent into the OneHumanCorp platform, the AI can access the merchant's real-time calendar for booking (Leo), menu availability (Fatima), and quoting system (Carlos). It can converse naturally, take actions, and log everything seamlessly into the OHC Omnichannel AI Inbox.

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      Caller[Customer Phone] -->|PSTN Call| Twilio[Telephony Gateway]
      Twilio -->|WebRTC/SIP| VoiceAIEdge[Voice AI Edge Engine]

      subgraph OneHumanCorp Platform
          VoiceAIEdge -->|Transcribes & Generates Audio| LLM[Low-Latency LLM cluster]
          VoiceAIEdge -->|Events| VoiceContextRouter[Voice Context Router]

          VoiceContextRouter -->|Fetch Availability| CalendarService[Unified Calendar Service]
          VoiceContextRouter -->|Fetch Menu/Prices| CatalogService[Unified Catalog Service]
          VoiceContextRouter -->|Log Conversation| OmnichannelInbox[Omnichannel AI Inbox]

          VoiceContextRouter -->|Action: Send SMS| NotificationService[Notification Service]
      end

      NotificationService -->|SMS Link| CustomerPhone[Customer Phone]
  ```

  ### Sequence Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant Caller
      participant AI Edge
      participant Context Router
      participant Calendar DB
      participant AI Inbox

      Caller->>AI Edge: "Do you have an opening tomorrow at 2 PM?"
      AI Edge->>Context Router: Intent: Check Availability (Tomorrow, 2PM)
      Context Router->>Calendar DB: Query availability
      Calendar DB-->>Context Router: Return Slot Available
      Context Router-->>AI Edge: Slot Available
      AI Edge->>Caller: "Yes, I have 2 PM open! Should I book it for you?"
      Caller->>AI Edge: "Yes, please."
      AI Edge->>Context Router: Intent: Book (Tomorrow, 2PM)
      Context Router->>Calendar DB: Reserve Slot
      Context Router->>AI Inbox: Log Booking Event
      AI Edge->>Caller: "All set! I've texted you a confirmation."
  ```

  ### Entity-Relationship Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      MERCHANT ||--o{ CALL_SESSION : receives
      CALL_SESSION ||--o{ TRANSCRIPT_LOG : generates
      CALL_SESSION ||--o{ INTENT_ACTION : triggers
      INTENT_ACTION }|..|{ CALENDAR_SLOT : books
      INTENT_ACTION }|..|{ INVOICE : creates

      MERCHANT {
          string id
          string business_name
          string voice_preferences
      }

      CALL_SESSION {
          string session_id
          string merchant_id
          string caller_phone
          timestamp start_time
          timestamp end_time
      }

      TRANSCRIPT_LOG {
          string id
          string session_id
          string role "USER or AI"
          string text
          timestamp timestamp
      }
  ```

  ### UX Flow & Mobile-First Wireframes (375px)
  1. **Acquisition/Onboarding**: Maya goes to Settings > Phone in the OHC App. She taps "Activate AI Receptionist."
  2. **Configuration**: She is presented with a clean, glass-morphic card: "Select an AI Voice" (Play buttons next to friendly voices). Below it, a toggle: "Allow AI to book appointments" and "Allow AI to text callers links."
  3. **Operation (Invisible)**: A customer calls. The AI answers: "Hi, thanks for calling Maya's Bakery. Are you calling to place a custom cake order, or do you have a question?"
  4. **Post-Call (Activation/Retention)**: Immediately after the call, Maya receives a push notification: "AI Receptionist: Booked consultation with Sarah for Tuesday at 2 PM. Deposit link sent via SMS."
  5. **Unified Inbox**: Maya opens the OHC app and sees the call in her Omnichannel Inbox with a neat summary, the full transcript, and the audio recording available for playback.

  ### AI Department Coordination
  - **CS Department (Customer Service Agent)**: Handles the primary real-time voice interaction, maintaining persona and politeness.
  - **Operations Department**: Receives structured data from the CS Agent to block out calendar slots or check inventory.
  - **Finance Department**: Triggers the generation of deposit payment links sent via SMS to the caller.

  ### Technical Integrity & Zero Trust
  - **Performance Targets**: Voice-to-voice latency must be < 800ms to feel natural. WebRTC edge nodes should be geographically distributed.
  - **Zero Trust**: The Voice Context Router accesses the Calendar and Catalog services using short-lived, scoped SPIFFE/SPIRE identity tokens specific to the tenant (the merchant's OHC ID). No cross-tenant data leakage is physically possible at the network layer.
  - **Offline Resilience**: If the core OHC systems are unreachable, the Voice AI Edge gracefully falls back to a static offline greeting and records a standard voicemail, syncing it to the inbox when connectivity is restored.

  ## Implementation Prompt
  **To the Implementer:**
  Build the underlying services for the Autonomous Voice AI Phone Attendant Engine.
  1. Integrate a low-latency voice AI pipeline (e.g., using Twilio + a WebRTC voice AI provider) capable of sub-800ms response times.
  2. Create the `VoiceContextRouter` to securely connect the active call state to the merchant's calendar, catalog, and inbox.
  3. Ensure the AI can seamlessly transition a call to an SMS flow (e.g., "I've just texted you a secure link to pay the deposit").
  4. Guarantee strict multi-tenant isolation so the AI only accesses data belonging to the specific merchant being called. Do not prescribe specific database schemas; focus on achieving the end-user experience (sub-800ms latency, intelligent booking, invisible operation) mapped in the design doc.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
