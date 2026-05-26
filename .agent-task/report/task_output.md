issue_title: "Build the Invisible AI Voice Receptionist Mesh for Omnichannel SMB Call Handling"
issue_description: |
  # [Architecture] Invisible AI Voice Receptionist Mesh

  ## Title
  Build the Invisible AI Voice Receptionist Mesh for Omnichannel SMB Call Handling

  ## Problem Statement
  Many small businesses, like Fatima's food cart or Carlos's handyman service, rely heavily on phone calls to capture immediate revenue. When they are busy serving customers or on a job site, they miss calls. Missed calls mean lost revenue. Current platforms (Shopify, Wix, Squarespace) offer great web storefronts and text-based chat, but they completely ignore the *phone*—which remains the primary communication channel for local service and food businesses. SMB owners do not have the time to configure complex IVR systems, set up SIP trunks, or train a human receptionist. They need an invisible, zero-configuration system that can automatically answer calls, speak in multiple languages (like English and Arabic for Fatima), take orders, quote prices, and book appointments, seamlessly integrating this offline voice channel into their centralized digital ledger.

  ## Research Report

  ### Market Needs & Persona Impact
  - **Fatima (Food Cart, 50):** Currently misses out on bulk catering orders while dealing with a busy lunch rush. She needs a system that picks up the phone, says "Hello, Fatima's Halal Cart," takes pre-orders, and prints them directly to her thermal printer or phone, completely in Arabic if the customer speaks Arabic.
  - **Carlos (Handyman, 42):** While under a sink fixing pipes, he can't answer calls. He needs an assistant that answers, asks about the problem, provides a rough AI-generated quote based on his catalog, and books a time block on his calendar.

  ### Competitive Analysis
  - **Shopify & Wix:** Focus entirely on digital-first text interactions (Live chat, AI chat on web). They provide zero out-of-the-box voice telephony solutions. Users must rely on disjointed third-party apps like Dialpad or Aircall, which require heavy manual configuration and do not share context natively with the storefront catalog.
  - **Squarespace / GoDaddy:** Same limitation. Phone support for the SMB is merely displaying a phone number on the website.
  - **Specialized Voice AI (e.g., PolyAI, Bland AI):** Enterprise-focused, requiring complex API setups and high minimum monthly spends. They are not built for a single tap-to-enable mobile interface that a food cart owner can use.

  ### Findings
  To achieve the OHC mission (zero → live business in under 10 minutes), the platform must treat voice as a native ingress channel, parallel to web checkout and Instagram DMs. The system must automatically provision a local phone number, attach a real-time conversational AI to it, and map intents (Order, Book, FAQ) directly to the OneHumanCorp business ledger.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  graph TD
      A[Customer Phone Call] -->|PSTN/Twilio| B(Voice Ingress Gateway)
      B -->|WebRTC/Stream| C{Voice-to-Text & Diarization}
      C --> D[Conversational AI Agent]

      subgraph OHC Multi-Tenant Mesh
      D -->|Context Query| E(Tenant Context & Memory)
      D -->|Action Intent| F{Intent Router}
      F -->|Order Intent| G[Universal Capacity & Inventory Ledger]
      F -->|Booking Intent| H[Unified Booking Engine]
      F -->|Support Intent| I[Omnichannel AI Inbox]
      end

      G -->|Push Notification| J(SMB Mobile App)
      H -->|Push Notification| J
      I -->|Push Notification| J

      E -->|Catalog Data| D
  ```

  ### Key Design Decisions
  1. **Zero-Config Telephony:** We abstract away all PSTN, Twilio/Telnyx SIP trunking, and number provisioning. When a user toggles "Enable Voice Receptionist," the system automatically leases a local number in the background and activates the agent.
  2. **Real-time Latency Focus:** The voice stack (ASR -> LLM -> TTS) must achieve <800ms time-to-first-byte response to feel natural. We will leverage streaming architectures rather than turn-based HTTP polling.
  3. **Multilingual Auto-Detection:** The agent must identify the speaker's language in the first few seconds and seamlessly switch without requiring the business owner to configure languages manually.
  4. **Action-Oriented Intent Routing:** The voice agent isn't just a chatbot; it has tools. It can directly execute transactions in the `Universal Capacity & Inventory Ledger` and the `Unified Booking Engine`.
  5. **Mobile-First Visibility:** Every call results in an actionable summary card in the mobile UI, converting ephemeral voice conversations into persistent, manageable state.

  ### UI Wireframes & Screen Flow Description (375px)

  **Screen 1: The Settings Card**
  - **Material:** Translucent Glass card on the dashboard.
  - **Content:** "AI Voice Receptionist"
  - **Action:** A simple, chunky iOS-style toggle. "Off" -> "On".
  - **Feedback:** "Provisioning local number..." -> "Active: +1 (555) 123-4567"

  **Screen 2: Call Customization (Advanced Settings)**
  - **Header:** "How should I answer?"
  - **Input:** A simple text area: "Hi, thanks for calling Fatima's. Would you like to place an order or ask a question?"
  - **Sliders:** "Voice Tone" (Professional, Friendly, Casual).
  - **Toggle:** "Allow Agent to take orders" / "Allow Agent to book appointments".

  **Screen 3: The Inbox Card (Post-Call)**
  - **Material:** UniFi-style modular card in the Omnichannel AI Inbox.
  - **Content:** "Missed Call from +1 (415) ... handled by AI."
  - **Summary Text:** "Customer ordered 2 Halal Platters for 1:00 PM pickup."
  - **Action Buttons:** "View Order" | "Listen to Recording"

  ### Mobile UX Flow
  1. User navigates to the "Channels" tab on their phone.
  2. Taps "Add Phone Number". The system instantly shows a local number.
  3. User toggles "AI Receptionist".
  4. When a customer calls, the app silently receives an intent. Once the call finishes, a push notification appears: "AI booked a new plumbing job for tomorrow at 10 AM."
  5. User taps the notification to see the booking details and the AI's plain-text summary of the conversation.

  ## Implementation Prompt

  **To the Implementer Agent:**
  Implement the backend architecture and mobile UI for the "Invisible AI Voice Receptionist".
  - **Outcome:** A user must be able to toggle on a voice receptionist from their mobile app. The system must provision a phone number, answer incoming calls using a real-time conversational AI, and parse the caller's intent (e.g., placing an order, booking an appointment, asking store hours).
  - **CUJ (Critical User Journey):** Fatima toggles the receptionist ON. A customer calls, speaks in Arabic, orders two meals, and hangs up. Fatima receives a push notification and sees a new order in her queue with a summary of the call.
  - **Acceptance Criteria:**
    1. A multi-tenant isolated configuration for telephony exists.
    2. The Voice Agent has access to the tenant's specific catalog and inventory via read-only memory.
    3. The Voice Agent can mutate the tenant's ledger (create an order or booking) through secure agentic tools.
    4. The mobile UI component for toggling the service and viewing call summaries matches the OHC design system (glassmorphism, grandmother-test approved).
    5. Latency targets and Zero-Trust boundaries for multi-tenancy are strictly maintained. Do not hardcode specific telephony providers (e.g., Twilio) in the core domain; use interfaces.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
